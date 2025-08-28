# Mapping: Storage

Sources
- ts-sdk: storage abstractions and implementations (e.g., IndexedDB for browser)
- go-sdk: minimal state persistence patterns (where applicable)

Rust target (rs-sdk)
- Module: storage
  - Trait
    - Storage: get(key) -> Option<Vec<u8>>, put(key, value), delete(key)
  - Planned adapters
    - memory (dev/testing)
    - kv (sled or sqlite as separate crate)
    - wasm (IndexedDB via wasm-bindgen, feature-gated)

Parity notes
- Keep keys as UTF-8 strings and values as opaque bytes for compatibility with TS.
- For WASM, mirror IndexedDB semantics where possible (async wrappers in wasm feature).
