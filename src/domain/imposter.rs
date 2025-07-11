#[derive(Debug, Clone)]
pub struct Imposter {
    path: String,
}

impl Imposter {
    pub fn new(path: impl Into<String>) -> Self {
        Self { path: path.into() }
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
