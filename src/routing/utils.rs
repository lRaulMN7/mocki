use crate::prelude::*;
use crate::{DynamicRoute, OpenApiSpec};
use axum::body::Body;
use axum::extract::Path;
use axum::http::{StatusCode, Uri};
use axum::response::Response;
use axum::Json;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

pub async fn create_handler(
    Path(name): Path<String>,
    axum::extract::Extension(dynamic_routes): axum::extract::Extension<
        Arc<Mutex<HashMap<String, DynamicRoute>>>,
    >,
    Json(_spec): Json<OpenApiSpec>,
) -> (StatusCode, &'static str) {
    info!("[😑] Creating new route for: {}", name);

    let mut routes = dynamic_routes.lock().unwrap();

    if routes.contains_key(&name) {
        return (StatusCode::CONFLICT, "Route already exists");
    }

    routes.insert(
        name.clone(),
        DynamicRoute {
            path: format!("/{name}"),
        },
    );

    (StatusCode::CREATED, "Route created successfully")
}

pub async fn dynamic_route_handler(
    uri: Uri,
    axum::extract::Extension(dynamic_routes): axum::extract::Extension<
        Arc<Mutex<HashMap<String, DynamicRoute>>>,
    >,
) -> Response<Body> {
    let path = uri.path();
    let routes = dynamic_routes.lock().unwrap();

    let route_name = path.trim_start_matches('/');

    if let Some(route) = routes.get(route_name) {
        Response::builder()
            .status(StatusCode::OK)
            .body(Body::from(format!(
                "Handling dynamic route: {}",
                route.path
            )))
            .unwrap()
    } else {
        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("Route not found"))
            .unwrap()
    }
}
