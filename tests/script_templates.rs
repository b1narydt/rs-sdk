use rs_sdk::script::{templates::{P2pkhTemplate, PushDropTemplate, RPuzzleTemplate}, OP_DUP, OP_HASH160, OP_EQUALVERIFY, OP_CHECKSIG, OP_DROP};

#[test]
fn p2pkh_locking_script_encoding() {
    let h20 = [0x11u8; 20];
    let s = P2pkhTemplate::locking_script(h20).into_bytes();
    let mut expected = Vec::new();
    expected.push(OP_DUP);
    expected.push(OP_HASH160);
    expected.push(20u8); // push 20 bytes
    expected.extend_from_slice(&h20);
    expected.push(OP_EQUALVERIFY);
    expected.push(OP_CHECKSIG);
    assert_eq!(s, expected);
}

#[test]
fn p2pkh_unlocking_script_encoding() {
    let sig = vec![0x30, 0x44, 0x02, 0x20, 0xaa, 0xbb, 0xcc]; // fake sig bytes
    let pubkey = vec![0x02, 0x01, 0x02, 0x03]; // fake compressed pubkey
    let s = P2pkhTemplate::unlocking_script(&sig, &pubkey).into_bytes();

    let mut expected = Vec::new();
    // push sig
    expected.push(sig.len() as u8);
    expected.extend_from_slice(&sig);
    // push pubkey
    expected.push(pubkey.len() as u8);
    expected.extend_from_slice(&pubkey);

    assert_eq!(s, expected);
}

#[test]
fn push_drop_locking_script_encoding() {
    let data = vec![1u8, 2, 3, 4, 5];
    let s = PushDropTemplate::locking_script(&data).into_bytes();

    let mut expected = Vec::new();
    expected.push(data.len() as u8);
    expected.extend_from_slice(&data);
    expected.push(OP_DROP);

    assert_eq!(s, expected);
}

#[test]
fn r_puzzle_locking_script_placeholder_encoding() {
    let r = vec![0xAB; 32];
    let s = RPuzzleTemplate::locking_script(&r).into_bytes();

    let mut expected = Vec::new();
    expected.push(r.len() as u8);
    expected.extend_from_slice(&r);
    expected.push(OP_DROP);

    assert_eq!(s, expected);
}

// Parser roundtrip checks
use rs_sdk::script::{Script, Chunk};

#[test]
fn parse_p2pkh_locking_chunks() {
    let h20 = [0x22u8; 20];
    let script = P2pkhTemplate::locking_script(h20);
    let chunks = script.parse().expect("parse ok");
    assert!(matches!(chunks[0], Chunk::Op(op) if op == OP_DUP));
    assert!(matches!(chunks[1], Chunk::Op(op) if op == OP_HASH160));
    match &chunks[2] { Chunk::Push(data) => assert_eq!(data.as_slice(), &h20), _ => panic!("expected push") }
    assert!(matches!(chunks[3], Chunk::Op(op) if op == OP_EQUALVERIFY));
    assert!(matches!(chunks[4], Chunk::Op(op) if op == OP_CHECKSIG));
    assert_eq!(chunks.len(), 5);
}

#[test]
fn parse_push_drop_chunks() {
    let data = vec![9u8,8,7,6,5,4];
    let script = PushDropTemplate::locking_script(&data);
    let chunks = script.parse().expect("parse ok");
    match &chunks[0] { Chunk::Push(d) => assert_eq!(d, &data), _ => panic!("expected push") }
    assert!(matches!(chunks[1], Chunk::Op(op) if op == OP_DROP));
    assert_eq!(chunks.len(), 2);
}
