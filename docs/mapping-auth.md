# Mapping: Auth

Sources
- ts-sdk: packages likely under auth/ (challenge/response, token issuance)
- go-sdk: auth/*.go and vector tests in testdata/auth/*.json

Rust target (rs-sdk)
- Module: auth
  - Types
    - AuthSession { token: Option<String> }
    - AuthChallenge { nonce: Vec<u8>, expires_at: u64 } (planned)
    - AuthToken { value: String, issued_at: u64, expires_at: u64 } (planned)
  - Traits
    - AuthProvider: start() -> AuthSession, complete(session) -> AuthSession
  - Functions (planned)
    - make_challenge(pubkey) -> AuthChallenge
    - verify_signature(challenge, signature, pubkey) -> bool
    - issue_token(identity, challenge) -> AuthToken

Parity notes
- Use crypto::sign/verify for signatures; use go-sdk vectors to validate.
- Token format to mirror ts-sdk/go-sdk (decide on JSON/JWT-like or compact form).
