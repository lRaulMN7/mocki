mod prelude;
mod routing;
use crate::prelude::*;

use axum::{
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, Mutex},
};

#[derive(Clone)]
struct DynamicRoute {
    path: String,
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .json()
        .with_max_level(tracing::Level::DEBUG)
        .init();

    let dynamic_routes: Arc<Mutex<HashMap<String, DynamicRoute>>> =
        Arc::new(Mutex::new(HashMap::new()));

    let app = Router::new()
        .route("/", get(routing::root_handler))
        .route("/debug", get(routing::debug_handler))
        .route("/create/{name}", post(routing::utils::create_handler))
        .fallback(routing::utils::dynamic_route_handler)
        .layer(tower_http::add_extension::AddExtensionLayer::new(
            dynamic_routes.clone(),
        ));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("Starting server on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

#[derive(Deserialize)]
struct OpenApiSpec {}
