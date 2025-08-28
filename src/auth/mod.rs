// Authentication module (placeholders)
use crate::error::{Result, SdkError};

// Mirror ts-sdk subfolders
pub mod certificates;
pub mod clients;
pub mod transports;
pub mod utils;

#[derive(Debug, Default, Clone)]
pub struct AuthSession {
    pub token: Option<String>,
}

pub trait AuthProvider {
    fn start(&self) -> Result<AuthSession> { Err(SdkError::NotImplemented("AuthProvider::start")) }
    fn complete(&self, _session: &AuthSession) -> Result<AuthSession> { Err(SdkError::NotImplemented("AuthProvider::complete")) }
}
