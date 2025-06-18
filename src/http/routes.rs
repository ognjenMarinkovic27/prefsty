use axum::{
    Router,
    routing::{any, get, post},
};

use crate::http::{ApiContext, controllers};
use tower::ServiceBuilder;
use tower_http::trace::TraceLayer;

pub async fn app(api_context: ApiContext) -> Router {
    let auth_routes = Router::new()
        .route("/auth/login", post(controllers::auth::login))
        .route("/auth/signup", post(controllers::auth::signup));

    let games_routes = Router::new()
        .route("/games", get(controllers::game::get_all))
        .route("/games/{id}", get(controllers::game::get_by_id))
        .route(
            "/games/{id}/joined",
            get(controllers::game::get_joined_by_game_id),
        )
        .route("/games/{id}/join", post(controllers::game::join))
        .route("/games", post(controllers::game::create));

    let ws_route = Router::new().route("/ws/{game_id}", any(controllers::ws::handler));

    Router::new()
        .merge(auth_routes)
        .merge(games_routes)
        .merge(ws_route)
        .layer(ServiceBuilder::new().layer(TraceLayer::new_for_http().on_failure(())))
        .with_state(api_context)
}
