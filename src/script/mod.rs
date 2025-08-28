// Script module placeholders
pub mod templates; // mirror ts-sdk script/templates

// Common opcode constants (subset)
pub const OP_0: u8 = 0x00;
pub const OP_DUP: u8 = 0x76;
pub const OP_HASH160: u8 = 0xA9;
pub const OP_EQUALVERIFY: u8 = 0x88;
pub const OP_CHECKSIG: u8 = 0xAC;
pub const OP_DROP: u8 = 0x75;
pub const OP_EQUAL: u8 = 0x87;
pub const OP_SIZE: u8 = 0x82;
pub const OP_SHA256: u8 = 0xA8;
pub const OP_PUSHDATA1: u8 = 0x4C;
pub const OP_PUSHDATA2: u8 = 0x4D;
pub const OP_PUSHDATA4: u8 = 0x4E;

#[derive(Debug, Default, Clone, PartialEq, Eq)]
pub struct Script(pub Vec<u8>);

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Chunk {
    Op(u8),
    Push(Vec<u8>),
}

impl Script {
    pub fn new() -> Self { Self(Vec::new()) }
    pub fn into_bytes(self) -> Vec<u8> { self.0 }
    pub fn as_bytes(&self) -> &[u8] { &self.0 }

    pub fn push_opcode(mut self, op: u8) -> Self {
        self.0.push(op);
        self
    }

    pub fn push_small_int(mut self, n: u8) -> Self {
        // OP_1 .. OP_16 are 0x51..0x60; OP_0 is 0x00
        match n {
            0 => self.0.push(OP_0),
            1..=16 => self.0.push(0x50 + n),
            _ => self = self.push_data(&[n]),
        }
        self
    }

    pub fn push_data(mut self, data: &[u8]) -> Self {
        let len = data.len();
        if len < 0x4c { // direct length push
            self.0.push(len as u8);
        } else if len <= 0xff {
            self.0.push(OP_PUSHDATA1);
            self.0.push(len as u8);
        } else if len <= 0xffff {
            self.0.push(OP_PUSHDATA2);
            self.0.extend_from_slice(&(len as u16).to_le_bytes());
        } else {
            self.0.push(OP_PUSHDATA4);
            self.0.extend_from_slice(&(len as u32).to_le_bytes());
        }
        self.0.extend_from_slice(data);
        self
    }

    // Parse the script into chunks of opcodes and pushed data
    pub fn parse(&self) -> Result<Vec<Chunk>, String> {
        let mut i = 0usize;
        let b = &self.0;
        let mut chunks = Vec::new();
        while i < b.len() {
            let opcode = b[i];
            i += 1;
            match opcode {
                0x01..=0x4b => {
                    let len = (opcode as usize);
                    if i + len > b.len() { return Err("push length exceeds script size".into()); }
                    chunks.push(Chunk::Push(b[i..i+len].to_vec()));
                    i += len;
                }
                OP_PUSHDATA1 => {
                    if i + 1 > b.len() { return Err("PUSHDATA1 missing length".into()); }
                    let len = b[i] as usize; i += 1;
                    if i + len > b.len() { return Err("PUSHDATA1 length exceeds script size".into()); }
                    chunks.push(Chunk::Push(b[i..i+len].to_vec()));
                    i += len;
                }
                OP_PUSHDATA2 => {
                    if i + 2 > b.len() { return Err("PUSHDATA2 missing length".into()); }
                    let len = u16::from_le_bytes([b[i], b[i+1]]) as usize; i += 2;
                    if i + len > b.len() { return Err("PUSHDATA2 length exceeds script size".into()); }
                    chunks.push(Chunk::Push(b[i..i+len].to_vec()));
                    i += len;
                }
                OP_PUSHDATA4 => {
                    if i + 4 > b.len() { return Err("PUSHDATA4 missing length".into()); }
                    let len = u32::from_le_bytes([b[i], b[i+1], b[i+2], b[i+3]]) as usize; i += 4;
                    if i + len > b.len() { return Err("PUSHDATA4 length exceeds script size".into()); }
                    chunks.push(Chunk::Push(b[i..i+len].to_vec()));
                    i += len;
                }
                op => {
                    chunks.push(Chunk::Op(op));
                }
            }
        }
        Ok(chunks)
    }
}
