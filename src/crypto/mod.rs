// Cryptographic primitives (placeholders)
use crate::error::{Result, SdkError};
use sha2::{Digest as _, Sha256 as Sha2};
use ripemd::Ripemd160 as Ripemd;

#[derive(Debug, Clone, Default)]
pub struct PrivateKey(pub [u8; 32]);
#[derive(Debug, Clone)]
pub struct PublicKey(pub [u8; 33]); // compressed
impl Default for PublicKey {
    fn default() -> Self { Self([0u8; 33]) }
}
#[derive(Debug, Clone, Default)]
pub struct KeyPair { pub private: PrivateKey, pub public: PublicKey }

#[derive(Debug, Clone, Default)]
pub struct Sha256(pub [u8; 32]);
#[derive(Debug, Clone, Default)]
pub struct Ripemd160(pub [u8; 20]);
#[derive(Debug, Clone, Default)]
pub struct Hash160(pub [u8; 20]);

pub fn sha256(data: &[u8]) -> Sha256 {
    let mut hasher = Sha2::new();
    hasher.update(data);
    let out = hasher.finalize();
    let mut bytes = [0u8; 32];
    bytes.copy_from_slice(&out);
    Sha256(bytes)
}

pub fn ripemd160(data: &[u8]) -> Ripemd160 {
    let mut hasher = Ripemd::new();
    use ripemd::Digest as _;
    hasher.update(data);
    let out = hasher.finalize();
    let mut bytes = [0u8; 20];
    bytes.copy_from_slice(&out);
    Ripemd160(bytes)
}

pub fn hash160(data: &[u8]) -> Hash160 {
    let Sha256(sha) = sha256(data);
    let Ripemd160(r) = ripemd160(&sha);
    Hash160(r)
}

/// Double SHA-256 (aka hash256) used by Base58Check
pub fn sha256d(data: &[u8]) -> [u8; 32] {
    let Sha256(first) = sha256(data);
    let Sha256(second) = sha256(&first);
    second
}

pub fn generate_keypair() -> Result<KeyPair> { Err(SdkError::NotImplemented("generate_keypair")) }
pub fn sign(_privkey: &PrivateKey, _msg: &[u8]) -> Result<Vec<u8>> { Err(SdkError::NotImplemented("sign")) }
pub fn verify(_pubkey: &PublicKey, _msg: &[u8], _sig: &[u8]) -> Result<bool> { Err(SdkError::NotImplemented("verify")) }
