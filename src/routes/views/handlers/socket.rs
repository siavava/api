//! WebSocket handler for view-count requests over the unified
//! `/api/connect/` endpoint.

use crate::{
  controllers::views::{ViewsIncrement, ViewsOps},
  models::{
    connect::ConnectResponse,
    views::{ViewsRequest, ViewsResponse},
  },
  protocol::socket,
};

use actix_ws::Session;

/// Handles a views-scoped WebSocket request.
///
/// Returns `false` if the send failed (connection should be closed).
pub async fn handle_ws_request(
  db_client: &impl ViewsOps,
  session: &mut Session,
  request: ViewsRequest,
) -> bool {
  let response = match request {
    ViewsRequest::List { namespace } => {
      let all = db_client
        .get_all_views(namespace.as_deref())
        .await
        .unwrap_or_default();
      ConnectResponse::Views(ViewsResponse::List { views: all })
    }
    ViewsRequest::Get { path } => {
      let page = db_client
        .get_views(&path, ViewsIncrement::INCREMENT)
        .await
        .unwrap_or_default();
      ConnectResponse::Views(ViewsResponse::Update { views: page })
    }
  };
  socket::send_json(session, &response).await
}
