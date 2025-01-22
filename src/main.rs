use axum::{
    routing::{get, post}, 
    Router, Json, extract::Path,
    response::Response,
    http::{StatusCode, Uri},
    body::Body,
};
use serde::Deserialize;
use std::{collections::HashMap, net::SocketAddr, sync::{Arc, Mutex}};
use tracing::{debug, info};
use tracing_subscriber;
use tower_http;

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

    let dynamic_routes: Arc<Mutex<HashMap<String, DynamicRoute>>> = Arc::new(Mutex::new(HashMap::new()));

    let app = Router::new()
        .route("/", get(root_handler))
        .route("/debug", get(debug_handler))
        .route("/create/{name}", post(create_handler))
        .fallback(dynamic_route_handler)
        .layer(tower_http::add_extension::AddExtensionLayer::new(dynamic_routes.clone()));

    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    info!("Starting server on http://{}", addr);

    let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn root_handler() -> &'static str {
    info!("Handling request to /");
    "Welcome to the Rust backend!"
}

async fn debug_handler() -> &'static str {
    debug!("Handling request to /debug");
    "This is the debug route!"
}

#[derive(Deserialize)]
struct OpenApiSpec {
}

async fn create_handler(
    Path(name): Path<String>,
    axum::extract::Extension(dynamic_routes): axum::extract::Extension<Arc<Mutex<HashMap<String, DynamicRoute>>>>,
    Json(_spec): Json<OpenApiSpec>,
) -> (StatusCode, &'static str) {
    info!("Creating new route for: {}", name);

    let mut routes = dynamic_routes.lock().unwrap();
    
    if routes.contains_key(&name) {
        return (StatusCode::CONFLICT, "Route already exists");
    }

    routes.insert(
        name.clone(), 
        DynamicRoute {
            path: format!("/{}", name),
        }
    );

    (StatusCode::CREATED, "Route created successfully")
}

async fn dynamic_route_handler(
    uri: Uri,
    axum::extract::Extension(dynamic_routes): axum::extract::Extension<Arc<Mutex<HashMap<String, DynamicRoute>>>>,
) -> Response<Body> {
    let path = uri.path();
    let routes = dynamic_routes.lock().unwrap();
    
    let route_name = path.trim_start_matches('/');
    
    if let Some(route) = routes.get(route_name) {
        Response::builder()
            .status(StatusCode::OK)
            .body(Body::from(format!("Handling dynamic route: {}", route.path)))
            .unwrap()
    } else {
        Response::builder()
            .status(StatusCode::NOT_FOUND)
            .body(Body::from("Route not found"))
            .unwrap()
    }
}