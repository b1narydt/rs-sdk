// Overlay module (application-layer protocols on BSV)
use crate::error::{Result, SdkError};

#[derive(Debug, Default, Clone)]
pub struct OverlayMessage {
    pub kind: String,
    pub payload: Vec<u8>,
}

pub trait OverlayClient {
    fn send(&self, _msg: &OverlayMessage) -> Result<()> { Err(SdkError::NotImplemented("OverlayClient::send")) }
    fn receive(&self) -> Result<OverlayMessage> { Err(SdkError::NotImplemented("OverlayClient::receive")) }
}
