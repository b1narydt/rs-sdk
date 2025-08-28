// Transaction primitives and builder (placeholders)
use crate::error::{Result, SdkError};
use crate::script::{Script, templates::P2pkhTemplate};

// Submodules for sighash logic
pub mod sighash;
// Mirror ts-sdk subfolders
pub mod broadcasters;
pub mod chaintrackers;
pub mod fee_models;
pub mod http;

#[derive(Debug, Default, Clone)]
pub struct OutPoint { pub txid: [u8; 32], pub vout: u32 }

#[derive(Debug, Default, Clone)]
pub struct TxIn { pub prevout: OutPoint, pub script_sig: Vec<u8>, pub sequence: u32 }

#[derive(Debug, Default, Clone)]
pub struct TxOut { pub value: u64, pub script_pubkey: Vec<u8> }

#[derive(Debug, Default, Clone)]
pub struct Transaction { pub version: i32, pub vin: Vec<TxIn>, pub vout: Vec<TxOut>, pub locktime: u32 }

fn put_varint(buf: &mut Vec<u8>, n: u64) {
    if n < 0xFD { buf.push(n as u8); }
    else if n <= 0xFFFF { buf.push(0xFD); buf.extend_from_slice(&(n as u16).to_le_bytes()); }
    else if n <= 0xFFFF_FFFF { buf.push(0xFE); buf.extend_from_slice(&(n as u32).to_le_bytes()); }
    else { buf.push(0xFF); buf.extend_from_slice(&n.to_le_bytes()); }
}

fn get_varint(data: &[u8], i: &mut usize) -> Result<u64> {
    if *i >= data.len() { return Err(SdkError::ParseError("varint: EOF")); }
    let p = data[*i]; *i += 1;
    Ok(match p {
        x @ 0x00..=0xFC => x as u64,
        0xFD => { if *i + 2 > data.len() { return Err(SdkError::ParseError("varint16: EOF")); } let v = u16::from_le_bytes([data[*i], data[*i+1]]) as u64; *i += 2; v }
        0xFE => { if *i + 4 > data.len() { return Err(SdkError::ParseError("varint32: EOF")); } let v = u32::from_le_bytes([data[*i], data[*i+1], data[*i+2], data[*i+3]]) as u64; *i += 4; v }
        0xFF => { if *i + 8 > data.len() { return Err(SdkError::ParseError("varint64: EOF")); } let v = u64::from_le_bytes([data[*i], data[*i+1], data[*i+2], data[*i+3], data[*i+4], data[*i+5], data[*i+6], data[*i+7]]); *i += 8; v }
    })
}

pub fn serialize(tx: &Transaction) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.extend_from_slice(&tx.version.to_le_bytes());
    put_varint(&mut buf, tx.vin.len() as u64);
    for tin in &tx.vin {
        // prev txid (as stored), vout index
        buf.extend_from_slice(&tin.prevout.txid);
        buf.extend_from_slice(&tin.prevout.vout.to_le_bytes());
        put_varint(&mut buf, tin.script_sig.len() as u64);
        buf.extend_from_slice(&tin.script_sig);
        buf.extend_from_slice(&tin.sequence.to_le_bytes());
    }
    put_varint(&mut buf, tx.vout.len() as u64);
    for tout in &tx.vout {
        buf.extend_from_slice(&tout.value.to_le_bytes());
        put_varint(&mut buf, tout.script_pubkey.len() as u64);
        buf.extend_from_slice(&tout.script_pubkey);
    }
    buf.extend_from_slice(&tx.locktime.to_le_bytes());
    buf
}

pub fn deserialize(data: &[u8]) -> Result<Transaction> {
    let mut i = 0usize;
    if i + 4 > data.len() { return Err(SdkError::ParseError("tx: missing version")); }
    let version = i32::from_le_bytes([data[i], data[i+1], data[i+2], data[i+3]]); i += 4;
    let vin_len = get_varint(data, &mut i)? as usize;
    let mut vin = Vec::with_capacity(vin_len);
    for _ in 0..vin_len {
        if i + 32 + 4 > data.len() { return Err(SdkError::ParseError("txin: prevout EOF")); }
        let mut txid = [0u8; 32]; txid.copy_from_slice(&data[i..i+32]); i += 32;
        let vout = u32::from_le_bytes([data[i], data[i+1], data[i+2], data[i+3]]); i += 4;
        let script_len = get_varint(data, &mut i)? as usize;
        if i + script_len > data.len() { return Err(SdkError::ParseError("txin: script EOF")); }
        let script_sig = data[i..i+script_len].to_vec(); i += script_len;
        if i + 4 > data.len() { return Err(SdkError::ParseError("txin: seq EOF")); }
        let sequence = u32::from_le_bytes([data[i], data[i+1], data[i+2], data[i+3]]); i += 4;
        vin.push(TxIn { prevout: OutPoint { txid, vout }, script_sig, sequence });
    }
    let vout_len = get_varint(data, &mut i)? as usize;
    let mut vout = Vec::with_capacity(vout_len);
    for _ in 0..vout_len {
        if i + 8 > data.len() { return Err(SdkError::ParseError("txout: value EOF")); }
        let value = u64::from_le_bytes([data[i], data[i+1], data[i+2], data[i+3], data[i+4], data[i+5], data[i+6], data[i+7]]); i += 8;
        let spk_len = get_varint(data, &mut i)? as usize;
        if i + spk_len > data.len() { return Err(SdkError::ParseError("txout: spk EOF")); }
        let script_pubkey = data[i..i+spk_len].to_vec(); i += spk_len;
        vout.push(TxOut { value, script_pubkey });
    }
    if i + 4 > data.len() { return Err(SdkError::ParseError("tx: locktime EOF")); }
    let locktime = u32::from_le_bytes([data[i], data[i+1], data[i+2], data[i+3]]);
    Ok(Transaction { version, vin, vout, locktime })
}

pub const SEQUENCE_FINAL: u32 = 0xFFFF_FFFF;

#[derive(Debug, Default)]
pub struct TxBuilder { tx: Transaction }

impl TxBuilder {
    pub fn new() -> Self { Self { tx: Transaction { version: 1, vin: vec![], vout: vec![], locktime: 0 } } }

    pub fn version(mut self, v: i32) -> Self { self.tx.version = v; self }
    pub fn locktime(mut self, lt: u32) -> Self { self.tx.locktime = lt; self }

    pub fn input(mut self, prevout: OutPoint) -> Self {
        self.tx.vin.push(TxIn { prevout, script_sig: Vec::new(), sequence: SEQUENCE_FINAL });
        self
    }

    pub fn input_with(mut self, prevout: OutPoint, script_sig: Script, sequence: u32) -> Self {
        self.tx.vin.push(TxIn { prevout, script_sig: script_sig.into_bytes(), sequence });
        self
    }

    pub fn output(mut self, value: u64, script_pubkey: Script) -> Self {
        self.tx.vout.push(TxOut { value, script_pubkey: script_pubkey.into_bytes() });
        self
    }

    pub fn p2pkh_output(mut self, value: u64, pubkey_hash20: [u8; 20]) -> Self {
        let spk = P2pkhTemplate::locking_script(pubkey_hash20);
        self.tx.vout.push(TxOut { value, script_pubkey: spk.into_bytes() });
        self
    }

    pub fn build(self) -> Transaction { self.tx }
}
