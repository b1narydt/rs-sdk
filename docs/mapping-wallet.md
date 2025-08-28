# Mapping: Wallet

Sources
- ts-sdk: High-level wallet API (UTXO management, fee estimation, build/sign/send)
- go-sdk: wallet.go, wallet_keys.go, fee quote, broadcast helpers

Rust target (rs-sdk)
- Module: wallet
  - Types
    - WalletConfig { network: String }
    - Wallet<S: Storage> { cfg, keypair, storage }
  - Methods (planned)
    - balance() -> Result<u64>
    - list_utxos(addr) -> Result<Vec<Utxo>>
    - build_payment(to: Address, amount: u64, fee_rate: FeeRate) -> Result<Transaction>
    - sign(tx: &Transaction) -> Result<Transaction>
    - broadcast(tx: &Transaction, client: &impl HttpClient) -> Result<TxId>
  - Supporting types
    - Utxo { outpoint, value, script_pubkey }
    - FeeRate { sat_per_byte: u64 }
    - TxId([u8;32])

Parity notes
- Keep API ergonomic and close to ts-sdk. Use net::HttpClient to abstract RPCs.
- Pull default fee policy and coin selection heuristics from ts-sdk/go-sdk where available.
