# Mapping: Identity

Sources
- ts-sdk: identity/profile management (keys to identity mapping)
- go-sdk: wallet_keys.go integration with identities

Rust target (rs-sdk)
- Module: identity
  - Types
    - Identity { id: String, pubkey: PublicKey }
    - Profile (planned) { display_name, avatar_url, metadata }
  - Functions
    - IdentityManager::from_private_key(pk) -> Identity
    - parse_identity(str) -> Identity (planned)
    - to_string(identity) -> String (planned)

Parity notes
- Identity string format to mirror ts-sdk; generate vectors in ts to assert parse/format parity.
