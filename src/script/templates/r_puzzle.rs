// RPuzzle script template
use super::super::{Script, OP_DROP, OP_SIZE, OP_SHA256, OP_EQUAL, OP_EQUALVERIFY};

#[derive(Debug, Default, Clone)]
pub struct RPuzzleTemplate;

impl RPuzzleTemplate {
    // Placeholder kept for compatibility until parity is finalized
    pub fn locking_script(r_value: &[u8]) -> Script {
        Script::new()
            .push_data(r_value)
            .push_opcode(OP_DROP)
    }

    // Common RPuzzle pattern: provide a 32-byte SHA256 digest; spending must push a 32-byte preimage
    // Script: OP_SIZE 32 OP_EQUALVERIFY OP_SHA256 <digest32> OP_EQUAL
    pub fn locking_script_sha256(digest32: [u8; 32]) -> Script {
        Script::new()
            .push_opcode(OP_SIZE)
            .push_small_int(32)
            .push_opcode(OP_EQUALVERIFY)
            .push_opcode(OP_SHA256)
            .push_data(&digest32)
            .push_opcode(OP_EQUAL)
    }

    // Unlocking script pushes the preimage
    pub fn unlocking_script_preimage(preimage: &[u8]) -> Script {
        Script::new().push_data(preimage)
    }
}
