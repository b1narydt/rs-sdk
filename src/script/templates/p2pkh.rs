// P2PKH script template
use super::super::{Script, OP_DUP, OP_HASH160, OP_EQUALVERIFY, OP_CHECKSIG};

#[derive(Debug, Default, Clone)]
pub struct P2pkhTemplate;

impl P2pkhTemplate {
    pub fn locking_script(pubkey_hash: [u8; 20]) -> Script {
        Script::new()
            .push_opcode(OP_DUP)
            .push_opcode(OP_HASH160)
            .push_data(&pubkey_hash)
            .push_opcode(OP_EQUALVERIFY)
            .push_opcode(OP_CHECKSIG)
    }
    pub fn unlocking_script(sig: &[u8], pubkey: &[u8]) -> Script {
        Script::new()
            .push_data(sig)
            .push_data(pubkey)
    }
}
