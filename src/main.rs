mod prelude;

mod domain;
mod infrastructure;

use crate::prelude::*;

use crate::domain::imposter::Imposter;
use crate::infrastructure::http::axum_handlers::{
    create_handler, debug_handler, dynamic_route_handler, root_handler,
};
use axum::{
    routing::{get, post},
    Router,
};
use serde::Deserialize;
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::sync::Mutex;

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

    let dynamic_routes: Arc<Mutex<HashMap<String, Imposter>>> =
        Arc::new(Mutex::new(HashMap::new()));

    let app = Router::new()
        .route("/", get(root_handler))
        .route("/debug", get(debug_handler))
        .route("/create/{name}", post(create_handler))
        .fallback(dynamic_route_handler)
        .with_state(dynamic_routes.clone());

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("Starting server on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr)
        .await
        .unwrap_or_else(|_| panic!("Failed to bind TCP listener to address: {addr}"));

    axum::serve(listener, app)
        .await
        .expect("Server failed during execution");
}

#[derive(Deserialize)]
struct OpenApiSpec {}
