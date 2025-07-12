use crate::domain::imposter::Imposter;
use axum::body::Body;
use axum::extract::{Path, State};
use axum::http::{StatusCode, Uri};
use axum::response::{IntoResponse, Response};
use axum::Json;
use serde::Deserialize;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use tracing::{debug, info};

pub type SharedRouteMap = Arc<Mutex<HashMap<String, Imposter>>>;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ImposterPayload {
    pub port: u16,
    pub protocol: String,
    pub default_response: Option<DefaultResponse>,
}

#[derive(Deserialize, Debug, Clone)]
#[serde(rename_all = "camelCase")]
pub struct DefaultResponse {
    pub status_code: u16,
    pub body: String,
    pub headers: HashMap<String, String>,
}

pub async fn create_handler(
    Path(name): Path<String>,
    State(routes): State<SharedRouteMap>,
    Json(spec): Json<ImposterPayload>,
) -> impl IntoResponse {
    let mut routes = routes.lock().await;

    if routes.contains_key(&name) {
        return (StatusCode::CONFLICT, "Route already exists");
    }
    info!("[😑] Creating new route for: {}", name);
    let imposter = Imposter::new(name, spec.default_response);
    routes.insert(imposter.path().to_string(), imposter);
    (StatusCode::CREATED, "Route created successfully")
}

pub async fn dynamic_route_handler(
    uri: Uri,
    State(routes): State<SharedRouteMap>,
) -> impl IntoResponse {
    let path = uri.path();
    let route_name = path.trim_start_matches('/');

    let routes = routes.lock().await;

    let Some(route) = routes.get(route_name) else {
        return Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("Route not found"))
            .unwrap();
    };

    let fallback = DefaultResponse {
        status_code: u16::from(StatusCode::OK),
        body: format!("Handling dynamic route: {}", route.path()),
        headers: Default::default(),
    };

    let response = route.default_response.as_ref().unwrap_or(&fallback);

    let mut response_builder = Response::builder().status(response.status_code);

    for (key, value) in &response.headers {
        response_builder = response_builder.header(key, value);
    }

    response_builder
        .body(Body::from(response.body.clone()))
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
