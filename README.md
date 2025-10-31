# Toy Blockchain in Rust

This project implements a minimal blockchain in Rust featuring:

- Proof of Work mining  
- Mempool for pending transactions  
- Ed25519 digital signatures for transaction validation  
- Basic chain verification  
- Expandable architecture for Proof of Stake, P2P, and persistence

---

## 🧩 Project Structure

```
toy_chain_mempool_sig/
├── Cargo.toml
└── src/
    ├── lib.rs
    └── main.rs
```

---

## 🚀 Features

| Feature | Description |
|----------|-------------|
| **Proof of Work** | Blocks are mined by finding a hash starting with N zeros. |
| **Mempool** | Pending transactions are stored until mined. |
| **Digital Signatures** | Transactions are signed with Ed25519 keys for authenticity. |
| **Chain Verification** | Each block links via the previous block’s hash. |
| **Easy to Extend** | Add PoS, disk persistence, or P2P networking. |

---

## 🦀 Build & Run

```bash
git clone <your-repo-url>
cd toy_chain_mempool_sig
cargo run
```

Expected output:
```
Blockchain valid: true
Blockchain: Blockchain { chain: [...], mempool: [] }
```

---

## 🧠 Next Enhancements

- [ ] Mempool prioritization  
- [ ] Difficulty adjustment  
- [ ] Block persistence (using serde and files)  
- [ ] P2P networking with gossip protocol  
- [ ] Toy Proof of Stake consensus  
- [ ] REST API for block/transaction submission  

---

## 📜 License
MIT License © 2025 Maksim Vialyx
