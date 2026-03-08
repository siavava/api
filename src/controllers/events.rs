//! # Events Broadcaster
//!
//! Generic Server-Sent Events (SSE) broadcaster backed by MongoDB change
//! streams.
//!
//! Provides [`EventsBroadcaster<T>`], which watches a MongoDB collection for
//! real-time changes and pushes updates to connected SSE clients. Includes
//! automatic heartbeat pings to detect and prune stale connections.

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

/// How often heartbeat pings are sent.
///
/// Should be half (or less) of the acceptable client timeout so that
/// stale connections are detected promptly.
const HEARTBEAT_INTERVAL: Duration = Duration::from_secs(3);

/// Generic SSE (Server-Sent Events) broadcaster backed by a MongoDB change
/// stream.
///
/// # Overview
///
/// Watches a MongoDB collection for changes and pushes updates to all
/// connected SSE clients whose filter matches the changed document.
/// Also runs a periodic heartbeat to detect and remove stale clients.
///
/// # Type Parameter
///
/// * `T` — The document/event type (e.g. [`PageViews`](crate::models::views::PageViews)).
///   Must be `Debug + Clone + Send + Sync + Serialize + Default + Eq`.
pub struct EventsBroadcaster<T: 'static + Debug + Clone + Send + Sync + Serialize + Default + Eq> {
  /// Mutex-protected inner state (client list + collection handle).
  mutex: Mutex<BroadcasterInner<T>>,
  /// If `true`, broadcasts the current listener count to all clients whenever
  /// a client connects or disconnects.
  notify_listener_count: bool,
}

/// A connected SSE client: its mpsc sender channel and the filter it
/// subscribed with.
#[derive(Debug, Clone)]
struct SenderData<T> {
  /// Channel sender for pushing SSE events to the client.
  sender: Sender<Event>,
  /// The filter value the client subscribed with.
  /// Events are only forwarded if the event matches this filter or the
  /// filter is `T::default()` (wildcard).
  filter: T,
}

/// Convenience macro for constructing a [`SenderData`] from a sender and
/// filter pair.
macro_rules! sender {
  ($sender:expr, $filter:expr) => {
    SenderData {
      sender: $sender,
      filter: $filter,
    }
  };
}

/// Internal state behind the [`EventsBroadcaster`] mutex.
#[derive(Debug, Clone)]
struct BroadcasterInner<T: 'static + Debug + Clone + Send + Sync + Serialize + Default + Eq> {
  /// Currently connected SSE clients.
  clients: Vec<SenderData<T>>,
  /// The MongoDB collection being watched for changes.
  collection: Collection<T>,
}

/// Payload for the `"count"` SSE event, sent when `notify_listener_count`
/// is enabled.
#[derive(Debug, Clone, Serialize, Deserialize)]
struct ActiveListeners {
  /// Number of currently connected SSE clients.
  count: usize,
}

/// Serializes [`ActiveListeners`] to JSON for SSE transmission.
impl std::convert::From<ActiveListeners> for ByteString {
  fn from(listeners: ActiveListeners) -> Self {
    let bytes_str = serde_json::to_string(&listeners);
    match bytes_str {
      Ok(value) => ByteString::from(value),
      Err(_) => ByteString::default(),
    }
  }
}

impl<T: 'static + Debug + Clone + Send + Sync + Serialize + Default + Eq> BroadcasterInner<T> {
  /// Creates a new inner state with an empty client list.
  ///
  /// # Arguments
  ///
  /// * `collection` — The MongoDB collection to watch.
  ///
  /// # Returns
  ///
  /// A `BroadcasterInner<T>` with no connected clients.
  fn new(collection: Collection<T>) -> Self {
    Self {
      clients: vec![],
      collection,
    }
  }
}

impl<
  T: Debug
    // + 'static
    + Clone
    + Send
    + Sync
    + Serialize
    + Default
    + Eq
    + for<'a> Deserialize<'a>
    + Into<ByteString>
    + From<ByteString>,
> EventsBroadcaster<T>
where
  for<'a> Watch<'a, T>: IntoFuture,
{
  /// Constructs a new broadcaster and spawns background tasks.
  ///
  /// # Arguments
  ///
  /// * `collection` — The MongoDB collection to watch.
  /// * `notify_listener_count` — If `true`, a `"count"` event is broadcast
  ///   whenever the number of connected clients changes.
  ///
  /// # Returns
  ///
  /// An `Arc<Self>` ready to register new SSE clients via [`new_client`](Self::new_client).
  ///
  /// # Background Tasks
  ///
  /// Two Actix-rt tasks are spawned automatically:
  /// 1. **Ping loop** — sends heartbeat pings at [`HEARTBEAT_INTERVAL`] and
  ///    removes unresponsive clients.
  /// 2. **Change-stream listener** — watches the collection and broadcasts
  ///    updates to matching clients.
  pub fn create(collection: Collection<T>, notify_listener_count: bool) -> Arc<Self> {
    let this = Arc::new(EventsBroadcaster {
      mutex: Mutex::new(BroadcasterInner::new(collection)),
      notify_listener_count,
    });

    Self::spawn_ping(Arc::clone(&this));
    Self::spawn_listen(Arc::clone(&this));

    this
  }

  /// Spawns a background task that pings clients at [`HEARTBEAT_INTERVAL`]
  /// and removes any that fail to respond.
  ///
  /// # Arguments
  ///
  /// * `this` — An `Arc` reference to the broadcaster instance.
  fn spawn_ping(this: Arc<Self>) {
    actix_web::rt::spawn(async move {
      let mut interval = interval(HEARTBEAT_INTERVAL);

      loop {
        interval.tick().await;
        this.remove_stale_clients().await;
      }
    });
  }

  /// Spawns a background task that opens a MongoDB change stream and
  /// broadcasts changes to all matching SSE clients.
  ///
  /// # Arguments
  ///
  /// * `this` — An `Arc` reference to the broadcaster instance.
  fn spawn_listen(this: Arc<Self>) {
    actix_web::rt::spawn(async move {
      this.listen().await;
    });
  }

  /// Listens to the collection for changes and broadcasts them to all
  /// clients.
  ///
  /// # Behavior
  ///
  /// Opens a MongoDB change stream with `FullDocumentType::UpdateLookup`
  /// so that every change event includes the full document.
  /// This is a long-running call and should be run in a background task
  /// (see [`spawn_listen`](Self::spawn_listen)).
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
          match event.full_document {
            Some(data) => {
              // broadcast the change to appropriate clients
              info!("notifying about event");
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

  /// Removes all non-responsive clients from the broadcast list.
  ///
  /// Sends a heartbeat ping to every client; those that fail to receive it
  /// are dropped. If the client count changes and `notify_listener_count`
  /// is enabled, a `"count"` event is broadcast.
  async fn remove_stale_clients(&self) {
    let clients = self.mutex.lock().clients.clone();
    let prev_clients_count = clients.len();

    let mut ok_clients = vec![];

    for client in clients {
      let status = client.sender.send(Event::Comment("ping".into())).await;

      if status.is_ok() {
        ok_clients.push(client.clone());
      } else {
        info!("removing stale client for {:?}", client.filter);
      }
    }

    let ok_clients_count = ok_clients.len();
    self.mutex.lock().clients = ok_clients;

    if ok_clients_count != prev_clients_count {
      self.maybe_notify_listener_count().await;
    }

    // ? self.log_listeners().await;
  }

  /// Sends a `"count"` SSE event to all connected clients with the current
  /// number of active listeners.
  ///
  /// This is a no-op if `notify_listener_count` is `false`.
  pub async fn maybe_notify_listener_count(&self) {
    if self.notify_listener_count {
      let clients = self.mutex.lock().clients.clone();
      let count = clients.len(); //.to_string();
      let send_futures = clients.iter().map(|client| {
        let SenderData { sender, filter: _ } = client;
        sender.send(
          sse::Data::new(ActiveListeners { count })
            .event("count")
            .into(),
        )
      });

      future::join_all(send_futures).await;
    }
  }

  /// Registers a new SSE client with the broadcaster.
  ///
  /// # Arguments
  ///
  /// * `filter` — The subscription filter.
  ///   Only events matching this value (or `T::default()` as wildcard) will
  ///   be forwarded to the client.
  ///
  /// # Returns
  ///
  /// An SSE response body that can be returned directly from an Actix-Web
  /// handler. A `"connected"` event is sent immediately upon registration.
  pub async fn new_client(&self, filter: T) -> Sse<InfallibleStream<ReceiverStream<sse::Event>>> {
    let (tx, rx) = mpsc::channel(10);

    tx.send(sse::Data::new("connected").event("connected").into())
      .await
      .unwrap();

    // self.mutex.lock().collection.find(sender.filter)

    info!("connected client with filter: {:?}", filter);
    self.mutex.lock().clients.push(sender!(tx, filter));

    // ? self.log_listeners().await;
    self.maybe_notify_listener_count().await;

    Sse::from_infallible_receiver(rx)
  }

  /// Broadcasts `msg` to all clients whose filter matches.
  ///
  /// # Arguments
  ///
  /// * `msg` — The event data to broadcast.
  ///
  /// # Behavior
  ///
  /// A client receives the event if either:
  /// - Its filter equals `msg`, **or**
  /// - Its filter is `T::default()` (wildcard — receives all events).
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

  /// Logs all currently connected clients' filters at `info` level.
  pub async fn log_listeners(&self) {
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
