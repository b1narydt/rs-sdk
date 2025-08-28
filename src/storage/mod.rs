// Storage abstractions (pluggable backends)
use crate::error::{Result, SdkError};

#[derive(Debug, Default, Clone)]
pub struct Record { pub key: String, pub value: Vec<u8> }

pub trait Storage {
    fn get(&self, _key: &str) -> Result<Option<Vec<u8>>> { Err(SdkError::NotImplemented("Storage::get")) }
    fn put(&mut self, _key: &str, _value: &[u8]) -> Result<()> { Err(SdkError::NotImplemented("Storage::put")) }
    fn delete(&mut self, _key: &str) -> Result<()> { Err(SdkError::NotImplemented("Storage::delete")) }
}
