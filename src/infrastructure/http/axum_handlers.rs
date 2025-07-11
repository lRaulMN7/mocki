use crate::domain::imposter::Imposter;
use axum::body::Body;
use axum::extract::{Path, State};
use axum::http::{StatusCode, Uri};
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use tracing::{debug, info};

pub type SharedRouteMap = Arc<Mutex<HashMap<String, Imposter>>>;

#[derive(Deserialize)]
pub struct ImposterPayload {
    pub port: u16,
    pub protocol: String,
}
pub async fn create_handler(
    Path(name): Path<String>,
    State(routes): State<SharedRouteMap>,
    Json(_spec): Json<ImposterPayload>,
) -> impl IntoResponse {
    let mut routes = routes.lock().unwrap();

    if routes.contains_key(&name) {
        return (StatusCode::CONFLICT, "Route already exists");
    }
    info!("[😑] Creating new route for: {}", name);
    let imposter = Imposter::new(name);
    routes.insert(imposter.path().to_string(), imposter);
    (StatusCode::CREATED, "Route created successfully")
}

pub async fn dynamic_route_handler(
    uri: Uri,
    State(routes): State<SharedRouteMap>,
) -> impl IntoResponse {
    let path = uri.path();
    let route_name = path.trim_start_matches('/');

    let routes = routes.lock().unwrap();

    let Some(route) = routes.get(route_name) else {
        return Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("Route not found"))
            .unwrap();
    };

    Response::builder()
        .status(StatusCode::OK)
        .body(Body::from(format!(
            "Handling dynamic route: {}",
            route.path()
        )))
        .unwrap()
}

pub async fn root_handler() -> &'static str {
    info!("Handling request to /");
    "[😑] Ready to mock"
}

pub async fn debug_handler() -> &'static str {
    debug!("Handling request to /debug");
    "This is the debug route!"
}
