use crate::infrastructure::http::axum_handlers::DefaultResponse;

#[derive(Debug, Clone)]
pub struct Imposter {
    path: String,
    pub default_response: Option<DefaultResponse>,
}

impl Imposter {
    pub fn new(path: impl Into<String>, default_response: Option<DefaultResponse>) -> Self {
        Self {
            path: path.into(),
            default_response,
        }
    }

    pub fn path(&self) -> &str {
        &self.path
    }
}

// pub enum ResponseTypes {
//     Is,
// }
// pub struct ResponseStub {}
//
// pub enum Predicates {
//     Equals,
// }
//
// pub struct PredicateStub {}
// pub struct ImposterModule {
//     port: u16,
//     protocol: String,
// }
