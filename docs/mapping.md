# rs-sdk Mapping: ts-sdk and go-sdk → Rust

Purpose: define a deterministic map from existing TypeScript (ts-sdk) and Go (go-sdk) SDKs to the Rust rs-sdk modules to streamline translation and validation.

Guiding principles
- Public API shape follows ts-sdk where practical (naming, ergonomics, async boundaries).
- Low-level algorithms and serialization follow go-sdk where it reduces ambiguity (byte layouts, digest flows, tx/script details).
- Traits abstract I/O and environment specifics (networking, storage), enabling server/native and WASM targets.

Top-level Rust modules
- util: hex, varint, endian, buffers
- crypto: secp256k1, hashes (SHA-256, RIPEMD-160, HASH160), HMAC
- net: HttpClient trait + concrete impls behind features
- auth: sessions, challenges, token issuance/verification
- identity: keys, identities, DID/profile (if present)
- overlay: overlay protocol messages/clients
- storage: trait and pluggable backends (memory/kv/indexed)
- wallet: façade combining keys, storage, net (balance, UTXO, send)
- transaction: primitives (Tx, TxIn, TxOut), builder, ser/de, sighash
- message: sign/verify messages (BSV standard)
- wasm: optional bindings

Domain-by-domain mapping

1) Crypto and Keys
- ts-sdk
  - hash: sha256, ripemd160, hash160
  - keys: PrivateKey, PublicKey, WIF import/export
- go-sdk
  - crypto folder: ECDSA/secp256k1 usage, hashing, HMAC (see crypto/*.go)
- rs-sdk
  - crypto::{PrivateKey, PublicKey, KeyPair, sha256, ripemd160, hash160, sign, verify}
  - util::{hex_encode, hex_decode}
  - TODO: base58, bech32 (if applicable for BSV), WIF import/export

2) Addressing
- ts-sdk
  - Address encode/decode (Base58Check), network prefix use
- go-sdk
  - wallet_keys.go: WIF/address helpers
- rs-sdk
  - crypto/base58 + address module (under crypto or identity)
  - TODO: Address {to_string, from_pubkey, parse}

3) Transaction primitives and ser/de
- ts-sdk
  - Transaction class, TxIn/TxOut, serialize/deserialize, varint parsing
- go-sdk
  - transaction.go + utils/hex/varint
- rs-sdk
  - transaction::{Transaction, TxIn, TxOut, OutPoint, serialize, deserialize}
  - util::{read_varint, write_varint}
  - TODO: scriptSig/scriptPubKey proper types; witness (if any), locktime/version handling

4) Sighash and signing transactions
- ts-sdk
  - SIGHASH flag enum and preimage creation (if exposed)
- go-sdk
  - sighash.go / transaction_signing.go
- rs-sdk
  - transaction::sighash module (flags, preimage, digest)
  - crypto::sign used with derived preimage
  - TODO: support common BSV variants used by wallet-toolbox

5) Script
- ts-sdk
  - Script class, opcodes, builder/parser
- go-sdk
  - script.go, opcodes, minimal encoding utilities
- rs-sdk
  - transaction::script module with Opcode enum, Script builder/parser
  - TODO: minimal-num encoding, canonical checks, standard templates (P2PKH)

6) Wallet façade
- ts-sdk
  - Wallet with high-level ops (UTXO fetch, fee estimate, build+sign+send)
- go-sdk
  - wallet.go, wallet_keys.go, fee quote, broadcast
- rs-sdk
  - wallet::Wallet<S: Storage> uses net::HttpClient and storage::Storage
  - TODO: balance(), list_utxos(), build_payment(), sign_and_broadcast()

7) Networking
- ts-sdk
  - HTTP calls to broadcast, fee quote, etc.
- go-sdk
  - wallet_wire*, network utilities
- rs-sdk
  - net::HttpClient trait; structs for FeeQuote, BroadcastResponse
  - Feature-gated reqwest impl for native; wasm fetch in wasm feature

8) Auth
- ts-sdk
  - auth flows (challenge/response, token issuance)
- go-sdk
  - auth/*.go, vector tests
- rs-sdk
  - auth::{AuthProvider, AuthSession}
  - TODO: HMAC/signature verification helpers via crypto; serialization formats to match SDKs

9) Identity
- ts-sdk
  - identity/profile management
- go-sdk
  - identity handling tied to keys/wallet
- rs-sdk
  - identity::{Identity, IdentityManager::from_private_key}
  - TODO: DID/document/profile types if required

10) Overlay
- ts-sdk
  - overlay protocol messages and wire handlers (WalletWire*, XDM, etc.)
- go-sdk
  - wallet_wire*.go, xdm tests
- rs-sdk
  - overlay::{OverlayMessage, OverlayClient}
  - TODO: map WalletWireCalls/Processor/Transceiver and XDM equivalents to traits/structs

11) Storage
- ts-sdk
  - abstract interfaces + implementations (e.g., IndexedDB)
- go-sdk
  - simple key-value or file-based state where used in tests
- rs-sdk
  - storage::Storage trait; adapters per backend as separate crates in future (sqlite, indexed)

12) Message signing (BSV message)
- ts-sdk
  - message signing/verification (if provided)
- go-sdk
  - verifySignature, makeSignature vector tests
- rs-sdk
  - message::{Message, SignedMessage, sign_message, verify_message}

Test strategy
- Extract vector tests from go-sdk testdata and convert into Rust fixtures
- Generate hex/JSON vectors via ts-sdk for address, WIF, transaction ser/de, sighash digests
- Run in native and wasm (where applicable) via cargo tests and wasm-bindgen-test

File-level pointers
- ts-sdk key areas (not exhaustive):
  - packages/wallet/src/core/transaction/ (tx, builder, script)
  - packages/wallet/src/core/keys/ (private/public, WIF)
  - packages/wallet/src/network/ (http, broadcast, fee)
  - packages/wallet/src/wire/ (WalletWire*, XDM)
- go-sdk key areas (not exhaustive):
  - transaction.go, script.go, wallet.go, wallet_keys.go
  - auth/*.go, wire/*.go (wallet_wire_*)
  - testdata/ vectors

Open questions / decisions
- secp256k1 crate choice (rust-secp256k1 vs k256) and wasm-compat story.
- Address formats required (Base58Check only vs additional formats).
- Scope of script engine (full eval vs builder/parser only initially).
- Which network endpoints to support first (broadcast, fee quote, UTXO source).

Next actions
1) Implement util (hex/varint) and crypto hashes with tests.
2) Add base58 + WIF + Address (encode/decode) with vectors.
3) Implement transaction ser/de; create vectors from ts-sdk/go-sdk.
4) Define sighash flags and preimage construction; verify against go-sdk vectors.
5) Flesh out Wallet façade methods and net::HttpClient example impls (feature-gated).
