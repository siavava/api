use actix_web::rt::time::interval;
use actix_web_lab::{
  sse::{self, Sse},
  util::InfallibleStream,
};
use bytestring::ByteString;
use futures::StreamExt;
use futures_util::future;
use mongodb::{
  action::Watch,
  change_stream::{event::ChangeStreamEvent, ChangeStream},
  options::FullDocumentType,
  Collection,
};
use mpsc::Sender;
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use sse::Event;
use std::{fmt::Debug, future::IntoFuture};
use std::{sync::Arc, time::Duration};
use tokio::sync::mpsc;
use tokio_stream::wrappers::ReceiverStream;

pub struct EventsBroadcaster<T: 'static + Debug + Clone + Send + Sync> {
  mutex: Mutex<BroadcasterInner<T>>,
}

#[derive(Debug, Clone)]
struct SenderData<T> {
  sender: Sender<Event>,
  filter: T,
}

/// sender macro
/// This takes a sender and a filter and returns a struct of the two.
macro_rules! sender {
  ($sender:expr, $filter:expr) => {
    SenderData {
      sender: $sender,
      filter: $filter,
    }
  };
}

#[derive(Debug, Clone)]
struct BroadcasterInner<T: 'static + Debug + Clone + Send + Sync + Serialize + Default + Eq> {
  clients: Vec<SenderData<T>>,
  collection: Collection<T>,
}

impl<T: 'static + Debug + Clone + Send + Sync> BroadcasterInner<T> {
  fn new(collection: Collection<T>) -> Self {
    BroadcasterInner {
      clients: Vec::new(),
      collection,
    }
  }
}

// impl

impl<
    T: 'static + Debug + Clone + Send + Sync + Serialize + Default + Eq + for<'a> Deserialize<'a>,
  > EventsBroadcaster<T>
where
  for<'a> T: Into<ByteString> + From<ByteString>,
  for<'a> Watch<'a, T>: IntoFuture,
{
  /// Constructs new broadcaster and spawns ping loop.
  pub fn create(collection: Collection<T>) -> Arc<Self> {
    let this = Arc::new(EventsBroadcaster {
      mutex: Mutex::new(BroadcasterInner::new(collection)),
    });

    EventsBroadcaster::spawn_ping(Arc::clone(&this));
    EventsBroadcaster::spawn_listen(Arc::clone(&this));

    this
  }

  /// Pings clients every 10 seconds to see if they are alive and remove them from the broadcast
  /// list if not.
  fn spawn_ping(this: Arc<Self>) {
    actix_web::rt::spawn(async move {
      let mut interval = interval(Duration::from_secs(10));

      loop {
        interval.tick().await;
        this.remove_stale_clients().await;
        // EventsBroadcaster::<T>::remove_stale_clients(&this).await;
      }
    });
  }

  fn spawn_listen(this: Arc<Self>) {
    actix_web::rt::spawn(async move {
      this.listen().await;
    });
  }

  /// # title
  /// Listens to collection for changes and broadcasts them to all clients.  
  /// This is a blocking call and should be run in a separate thread.
  pub async fn listen(&self) {
    println!("Listening for changes...");

    let collection = {
      let lock = self.mutex.lock();
      lock.collection.clone()
    };

    println!("Watching collection...");
    let thing: <Watch<'_, T> as IntoFuture>::Output = collection
      .watch()
      .full_document(FullDocumentType::UpdateLookup)
      .await;
    println!("Watch initiated...");

    // let mut change_stream: ChangeStream<ChangeStreamEvent<T>> = thing.into();

    let mut change_stream = unsafe {
      // SAFETY: this is safe because we are using the same type as the output of the watch
      std::mem::transmute_copy::<_, ChangeStream<ChangeStreamEvent<T>>>(&thing)
    };

    std::mem::forget(thing);

    println!("HERE....");

    while let Some(change) = change_stream.next().await {
      match change {
        Ok(event) => {
          // get the data
          let data = event.full_document;

          match data {
            Some(data) => {
              // println!("Change detected: {:?}", data);
              // broadcast the change to all clients
              println!("Broadcasting change: {:?}", data);
              self.broadcast(&data).await;
            }
            None => {
              eprintln!("No data in change event");
            }
          }

          // self.broadcast(&data).await;
        }
        Err(e) => {
          eprintln!("Error watching changes: {:?}", e);
        }
      }
    }
  }

  /// Removes all non-responsive clients from broadcast list.
  async fn remove_stale_clients(&self) {
    let clients = self.mutex.lock().clients.clone();

    let mut ok_clients = Vec::new();

    for client in clients {
      if client
        .sender
        .send(sse::Event::Comment("ping".into()))
        .await
        .is_ok()
      {
        ok_clients.push(client.clone());
      }
    }

    self.mutex.lock().clients = ok_clients;
  }

  /// Registers client with broadcaster, returning an SSE response body.
  pub async fn new_client(&self, filter: T) -> Sse<InfallibleStream<ReceiverStream<sse::Event>>> {
    let (tx, rx) = mpsc::channel(10);

    tx.send(sse::Data::new("connected").into()).await.unwrap();

    self.mutex.lock().clients.push(sender!(tx, filter));

    Sse::from_infallible_receiver(rx)
  }

  /// Broadcasts `msg` to all clients.
  pub async fn broadcast(&self, msg: &T) {
    let clients = self.mutex.lock().clients.clone();

    let send_futures = clients
      .iter()
      .filter(|client| {
        let SenderData { sender: _, filter } = client;
        println!("FILTER: {:?}", filter);
        filter == &T::default() || msg == filter
      })
      .map(|client| {
        let SenderData { sender, filter: _ } = client;
        sender.send(sse::Data::new(msg.clone()).into())
      });

    let _ = future::join_all(send_futures).await;
  }
}
