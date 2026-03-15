//! WebSocket handler for view-count requests over the unified
//! `/api/connect/` endpoint.

use crate::{
  controllers::views,
  protocol::socket,
  models::connect::ConnectResponse,
  models::views::{ViewsRequest, ViewsResponse},
};

use actix_ws::Session;
use mongodb::Client;

/// Handles a views-scoped WebSocket request.
///
/// Returns `false` if the send failed (connection should be closed).
pub async fn handle_ws_request(
  db_client: &Client,
  session: &mut Session,
  request: ViewsRequest,
) -> bool {
  let response = match request {
    ViewsRequest::List => {
      let all = views::get_all_views(db_client).await.unwrap_or_default();
      ConnectResponse::Views(ViewsResponse::List { views: all })
    }
  };
  socket::send_json(session, &response).await
}
