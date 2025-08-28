# Mapping: Message

Sources
- ts-sdk: message signing/verification helpers (if present)
- go-sdk: verifySignature/makeSignature vectors in testdata/auth

Rust target (rs-sdk)
- Module: message
  - Types
    - Message { data: Vec<u8> }
    - SignedMessage { data, signature, pubkey }
  - Functions
    - sign_message(msg, privkey) -> SignedMessage
    - verify_message(signed) -> bool

Parity notes
- Align signature format (DER vs compact) and hashing scheme with ts/go SDKs.
- Add vectors from go-sdk testdata to confirm correctness.
