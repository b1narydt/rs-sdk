// Networking abstractions behind a trait so clients can provide their own
use crate::error::{Result, SdkError};

pub trait HttpClient {
    fn get(&self, _url: &str) -> Result<Vec<u8>> { Err(SdkError::NotImplemented("HttpClient::get")) }
    fn post(&self, _url: &str, _body: &[u8]) -> Result<Vec<u8>> { Err(SdkError::NotImplemented("HttpClient::post")) }
}

// Example request types to be refined
#[derive(Debug, Default, Clone)]
pub struct FeeQuote;
#[derive(Debug, Default, Clone)]
pub struct BroadcastResponse;
