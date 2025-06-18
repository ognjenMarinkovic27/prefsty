use axum::{
    extract::{State, WebSocketUpgrade, ws::WebSocket},
    response::Response,
};

use crate::http::{ApiContext, extractors::AuthUser};

pub async fn handler(
    ws: WebSocketUpgrade,
    user: AuthUser,
    State(state): State<ApiContext>,
) -> Response {
    ws.on_upgrade(|socket| handle_socket(socket, user, state))
}

async fn handle_socket(socket: WebSocket, user: AuthUser, state: ApiContext) {}
