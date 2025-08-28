# Mapping: Overlay

Sources
- ts-sdk: WalletWire*, XDM, overlay protocol message formats and flows
- go-sdk: wallet_wire_*.go, xdm tests and vector files

Rust target (rs-sdk)
- Module: overlay
  - Types
    - OverlayMessage { kind: String, payload: Vec<u8> }
    - WalletWireCall, WalletWireResponse (planned)
    - XdmEnvelope (planned)
  - Traits
    - OverlayClient { send, receive }
  - Components (planned)
    - wallet_wire::{Calls, Processor, Transceiver}
    - xdm::{Envelope, encode, decode}

Parity notes
- Mirror message schemas from ts/go; keep a serde-compatible struct shape.
- Consider feature-gated transports (in-process, http, ws) behind OverlayClient.
