// Wallet façade over keys, storage, and network
use crate::{crypto::KeyPair, storage::Storage, error::{Result, SdkError}};

#[derive(Debug, Default)]
pub struct WalletConfig {
    pub network: String,
}

#[derive(Debug, Default)]
pub struct Wallet<S: Storage> {
    pub cfg: WalletConfig,
    pub keypair: Option<KeyPair>,
    pub storage: S,
}

impl<S: Storage> Wallet<S> {
    pub fn new(cfg: WalletConfig, storage: S) -> Self { Self { cfg, keypair: None, storage } }
    pub fn set_keypair(&mut self, kp: KeyPair) { self.keypair = Some(kp) }
    pub fn balance(&self) -> Result<u64> { Err(SdkError::NotImplemented("Wallet::balance")) }
}
