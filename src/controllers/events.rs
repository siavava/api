use actix_web::rt::time::interval;
use actix_web_lab::{
  sse::{self, Event, Sse},
  util::InfallibleStream,
};
use bytestring::ByteString;
use futures::StreamExt;
use futures_util::future;
use mongodb::{
  Collection,
  action::Watch,
  change_stream::{ChangeStream, event::ChangeStreamEvent},
  options::FullDocumentType,
};
use parking_lot::Mutex;
use serde::{Deserialize, Serialize};
use std::{fmt::Debug, future::IntoFuture, sync::Arc, time::Duration};
use tokio::sync::mpsc::{self, Sender};
use tokio_stream::wrappers::ReceiverStream;
use tracing::{info, warn};

pub struct EventsBroadcaster<T: 'static + Debug + Clone + Send + Sync + Serialize + Default + Eq> {
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

impl<T: 'static + Debug + Clone + Send + Sync + Serialize + Default + Eq> BroadcasterInner<T> {
  fn new(collection: Collection<T>) -> Self {
    BroadcasterInner {
      clients: Vec::new(),
      collection,
    }
  }
}

// impl

impl<T: 'static + Debug + Clone + Send + Sync + Serialize + Default + Eq + for<'a> Deserialize<'a>>
  EventsBroadcaster<T>
where
  T: Into<ByteString> + From<ByteString>,
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

  /// PINGS clients every 10 seconds to see if they are alive.  
  /// REMOVES them from the broadcast list if not.
  fn spawn_ping(this: Arc<Self>) {
    actix_web::rt::spawn(async move {
      let mut interval = interval(Duration::from_secs(10));

      loop {
        interval.tick().await;
        this.remove_stale_clients().await;
      }
    });
  }

  fn spawn_listen(this: Arc<Self>) {
    actix_web::rt::spawn(async move {
      this.listen().await;
    });
  }

  /// # `listen`
  /// Listens to collection for changes and broadcasts them to all clients.  
  /// This is a blocking call and should be run in a separate thread.
  pub async fn listen(&self) {
    let collection = {
      let lock = self.mutex.lock();
      lock.collection.clone()
    };

    let watch_handle = collection
      .watch()
      .full_document(FullDocumentType::UpdateLookup)
      .await;

    let mut change_stream = unsafe {
      // SAFETY: this is safe because we are using the same type as the output of the watch
      std::mem::transmute_copy::<_, ChangeStream<ChangeStreamEvent<T>>>(&watch_handle)
    };

    std::mem::forget(watch_handle);

    while let Some(change) = change_stream.next().await {
      match change {
        Ok(event) => {
          let data = event.full_document;

          match data {
            Some(data) => {
              // broadcast the change to appropriate clients
              info!("notifying event");
              self.broadcast(&data).await;
            }
            None => {
              warn!("No data in change event");
            }
          }
        }

        Err(e) => {
          warn!("Error watching changes: {:?}", e);
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
        .send(Event::Comment("ping".into()))
        .await
        .is_ok()
      {
        ok_clients.push(client.clone());
      } else {
        info!("removing stale client for {:?}", client.filter);
      }
    }

    self.mutex.lock().clients = ok_clients;

    // ? self.log_listeners().await;
  }

  /// Registers client with broadcaster, returning an SSE response body.
  pub async fn new_client(&self, filter: T) -> Sse<InfallibleStream<ReceiverStream<sse::Event>>> {
    let (tx, rx) = mpsc::channel(10);

    tx.send(sse::Data::new("connected").event("connected").into())
      .await
      .unwrap();

    // self.mutex.lock().collection.find(sender.filter)

    info!("connected client with filter: {:?}", filter);
    self.mutex.lock().clients.push(sender!(tx, filter));

    // ? self.log_listeners().await;

    Sse::from_infallible_receiver(rx)
  }

  /// Broadcasts `msg` to all clients.
  pub async fn broadcast(&self, msg: &T) {
    let clients = self.mutex.lock().clients.clone();

    let send_futures = clients.iter().filter_map(|client| {
      let SenderData { sender, filter } = client;
      if msg == filter || filter == &T::default() {
        info!("notifying client for filter: {:?}", filter);
        Some(sender.send(sse::Data::new(msg.clone()).event("update").into()))
      } else {
        None
      }
    });

    future::join_all(send_futures).await;
  }

  pub async fn log_listeners(&self) {
    // print all listeners (just their filters)
    let filters: Vec<T> = self
      .mutex
      .lock()
      .clients
      .iter()
      .map(|client| client.filter.clone())
      .collect();

    info!("CURRENT CLIENTS: {:?}", filters);
  }
}
