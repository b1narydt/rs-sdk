// Address and WIF helpers (placeholders)
use crate::crypto::{PublicKey, PrivateKey};
use crate::error::{Result, SdkError};

#[derive(Debug, Clone, Copy)]
pub enum Network { Mainnet, Testnet }
impl Default for Network { fn default() -> Self { Network::Mainnet } }

#[derive(Debug, Default, Clone)]
pub struct Address { pub network: Network, pub payload: [u8; 20] }

pub fn from_pubkey(_pk: &PublicKey, _net: Network) -> Address { Address { network: Network::Mainnet, payload: [0u8; 20] } }
pub fn parse(_s: &str) -> Result<Address> { Err(SdkError::NotImplemented("address::parse")) }
pub fn to_string(_a: &Address) -> String { String::new() }

pub fn wif_from_private_key(_pk: &PrivateKey, _net: Network, _compressed: bool) -> String { String::new() }
pub fn wif_to_private_key(_wif: &str) -> Result<(PrivateKey, Network, bool)> { Err(SdkError::NotImplemented("wif_to_private_key")) }
