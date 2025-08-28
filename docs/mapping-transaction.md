# Mapping: Transaction

Sources
- ts-sdk: Transaction class, TxIn/TxOut, builder, varint encoding, script serialization.
- go-sdk: transaction.go, varint helpers, serializer/deserializer, test vectors.

Rust target (rs-sdk)
- Module: transaction
  - Types
    - OutPoint { txid: [u8;32], vout: u32 }
    - TxIn { prevout: OutPoint, script_sig: Vec<u8>, sequence: u32 }
    - TxOut { value: u64, script_pubkey: Vec<u8> }
    - Transaction { version: i32, vin: Vec<TxIn>, vout: Vec<TxOut>, locktime: u32 }
  - Functions
    - serialize(&Transaction) -> Vec<u8>
    - deserialize(&[u8]) -> Result<Transaction>
  - Submodules
    - script: Opcode enum, Script builder/parser
    - sighash: SigHashType, preimage construction
  - Utilities
    - util::{read_varint, write_varint} for compact int encoding

Parity notes
- Match ts-sdk wire format exactly; use go-sdk vectors to validate roundtrip.
- Ensure minimal pushdata encoding where applicable for script building.
- Confirm endianness for txid (little-endian in wire, big-endian human)

Vectors
- Import from go-sdk/testdata for canonical transactions.
- Generate from ts-sdk by serializing known transactions and storing hex.
