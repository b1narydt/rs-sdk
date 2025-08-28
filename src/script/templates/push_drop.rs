// PushDrop script template
use super::super::{Script, OP_DROP};

#[derive(Debug, Default, Clone)]
pub struct PushDropTemplate;

impl PushDropTemplate {
    pub fn locking_script(data: &[u8]) -> Script {
        // <data> OP_DROP
        Script::new()
            .push_data(data)
            .push_opcode(OP_DROP)
    }
}
