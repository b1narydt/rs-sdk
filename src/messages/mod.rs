// Signed/unencrypted messaging over BSV (placeholders)
use crate::{crypto::{PrivateKey, PublicKey}, error::{Result, SdkError}};

#[derive(Debug, Default, Clone)]
pub struct Message { pub data: Vec<u8> }

#[derive(Debug, Default, Clone)]
pub struct SignedMessage { pub data: Vec<u8>, pub signature: Vec<u8>, pub pubkey: PublicKey }

pub fn sign_message(_msg: &Message, _priv: &PrivateKey) -> Result<SignedMessage> { Err(SdkError::NotImplemented("sign_message")) }
pub fn verify_message(_signed: &SignedMessage) -> Result<bool> { Err(SdkError::NotImplemented("verify_message")) }
