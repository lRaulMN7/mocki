pub mod utils;
use crate::prelude::*;

pub async fn root_handler() -> &'static str {
    info!("Handling request to /");
    "[😑] Ready to mock"
}

pub async fn debug_handler() -> &'static str {
    debug!("Handling request to /debug");
    "This is the debug route!"
}
