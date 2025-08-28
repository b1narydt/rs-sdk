// Utility helpers (buffer, hex, varint placeholders)
use crate::error::{Result, SdkError};
use bs58;
use crate::crypto::sha256d;

pub fn hex_encode(bytes: &[u8]) -> String {
    const HEX: &[u8; 16] = b"0123456789abcdef";
    let mut s = String::with_capacity(bytes.len() * 2);
    for b in bytes {
        s.push(HEX[(b >> 4) as usize] as char);
        s.push(HEX[(b & 0x0f) as usize] as char);
    }
    s
}

pub fn hex_decode(s: &str) -> Result<Vec<u8>> {
    let s = s.trim();
    if s.len() % 2 != 0 { return Err(SdkError::InvalidArgument("hex must have even length")); }
    let mut out = Vec::with_capacity(s.len() / 2);
    let bytes = s.as_bytes();
    for i in (0..bytes.len()).step_by(2) {
        let hi = from_hex_nibble(bytes[i])?;
        let lo = from_hex_nibble(bytes[i + 1])?;
        out.push((hi << 4) | lo);
    }
    Ok(out)
}

fn from_hex_nibble(c: u8) -> Result<u8> {
    match c {
        b'0'..=b'9' => Ok(c - b'0'),
        b'a'..=b'f' => Ok(c - b'a' + 10),
        b'A'..=b'F' => Ok(c - b'A' + 10),
        _ => Err(SdkError::InvalidArgument("invalid hex nibble")),
    }
}

// Bitcoin CompactSize (varint) encoding
pub fn read_varint(data: &[u8]) -> Result<(u64, usize)> {
    if data.is_empty() { return Err(SdkError::InvalidArgument("empty buffer")); }
    let first = data[0];
    match first {
        n @ 0x00..=0xfc => Ok((n as u64, 1)),
        0xfd => {
            if data.len() < 3 { return Err(SdkError::InvalidArgument("varint 0xfd needs 2 bytes")); }
            let v = u16::from_le_bytes([data[1], data[2]]) as u64;
            Ok((v, 3))
        }
        0xfe => {
            if data.len() < 5 { return Err(SdkError::InvalidArgument("varint 0xfe needs 4 bytes")); }
            let v = u32::from_le_bytes([data[1], data[2], data[3], data[4]]) as u64;
            Ok((v, 5))
        }
        0xff => {
            if data.len() < 9 { return Err(SdkError::InvalidArgument("varint 0xff needs 8 bytes")); }
            let v = u64::from_le_bytes([data[1], data[2], data[3], data[4], data[5], data[6], data[7], data[8]]);
            Ok((v, 9))
        }
    }
}

pub fn write_varint(n: u64, out: &mut Vec<u8>) {
    match n {
        0x0000_0000_0000_0000..=0x0000_0000_0000_00fc => out.push(n as u8),
        0x0000_0000_0000_00fd..=0x0000_0000_0000_ffff => {
            out.push(0xfd);
            out.extend_from_slice(&(n as u16).to_le_bytes());
        }
        0x0000_0000_0001_0000..=0x0000_0000_ffff_ffff => {
            out.push(0xfe);
            out.extend_from_slice(&(n as u32).to_le_bytes());
        }
        _ => {
            out.push(0xff);
            out.extend_from_slice(&n.to_le_bytes());
        }
    }
}

// Base58Check helpers: version byte + payload with 4-byte checksum
pub fn base58check_encode(version: u8, payload: &[u8]) -> String {
    let mut raw = Vec::with_capacity(1 + payload.len() + 4);
    raw.push(version);
    raw.extend_from_slice(payload);
    let checksum_full = sha256d(&raw);
    raw.extend_from_slice(&checksum_full[0..4]);
    bs58::encode(raw).into_string()
}

pub fn base58check_decode(s: &str) -> Result<(u8, Vec<u8>)> {
    let raw = bs58::decode(s).into_vec().map_err(|_| SdkError::InvalidArgument("invalid base58"))?;
    if raw.len() < 5 { return Err(SdkError::InvalidArgument("base58check too short")); }
    let (ver_and_payload, check) = raw.split_at(raw.len() - 4);
    let calc = sha256d(ver_and_payload);
    if check != &calc[0..4] { return Err(SdkError::InvalidArgument("bad base58check checksum")); }
    if ver_and_payload.is_empty() { return Err(SdkError::InvalidArgument("empty payload")); }
    Ok((ver_and_payload[0], ver_and_payload[1..].to_vec()))
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn hex_roundtrip() {
        let data = b"hello";
        let h = hex_encode(data);
        assert_eq!(h, "68656c6c6f");
        let back = hex_decode(&h).unwrap();
        assert_eq!(&back, data);
    }

    #[test]
    fn varint_cases() {
        let mut buf = Vec::new();
        write_varint(0xfc, &mut buf);
        assert_eq!(buf, vec![0xfc]);
        let (v, n) = read_varint(&buf).unwrap();
        assert_eq!(v, 0xfc);
        assert_eq!(n, 1);

        buf.clear();
        write_varint(0xfd, &mut buf);
        assert_eq!(buf, vec![0xfd, 0xfd, 0x00]);
        let (v, n) = read_varint(&buf).unwrap();
        assert_eq!(v, 0xfd);
        assert_eq!(n, 3);

        buf.clear();
        write_varint(0x1_0000, &mut buf);
        assert_eq!(buf[0], 0xfe);
        let (v, n) = read_varint(&buf).unwrap();
        assert_eq!(v, 0x1_0000);
        assert_eq!(n, 5);

        buf.clear();
        write_varint(0x1_0000_0000, &mut buf);
        assert_eq!(buf[0], 0xff);
        let (v, n) = read_varint(&buf).unwrap();
        assert_eq!(v, 0x1_0000_0000);
        assert_eq!(n, 9);
    }

    #[test]
    fn b58check_roundtrip() {
        let version = 0x00u8;
        let payload = [0x12u8; 20];
        let s = base58check_encode(version, &payload);
        let (v2, p2) = base58check_decode(&s).unwrap();
        assert_eq!(v2, version);
        assert_eq!(p2, payload);
    }
}
