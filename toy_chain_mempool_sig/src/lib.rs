use chrono::Utc;
use ed25519_dalek::{Signature, Signer, Verifier, SigningKey, VerifyingKey};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::collections::VecDeque;
use base64::{encode, decode};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializablePublicKey(pub String);

impl From<&VerifyingKey> for SerializablePublicKey {
    fn from(pk: &VerifyingKey) -> Self {
        SerializablePublicKey(encode(pk.to_bytes()))
    }
}

impl Into<VerifyingKey> for SerializablePublicKey {
    fn into(self) -> VerifyingKey {
        let bytes = decode(&self.0).unwrap();
        VerifyingKey::from_bytes(&bytes.try_into().unwrap()).unwrap()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SerializableSignature(pub String);

impl From<&Signature> for SerializableSignature {
    fn from(sig: &Signature) -> Self {
        SerializableSignature(encode(sig.to_bytes()))
    }
}

impl Into<Signature> for SerializableSignature {
    fn into(self) -> Signature {
        let bytes = decode(&self.0).unwrap();
        Signature::from_bytes(&bytes.try_into().unwrap())
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Transaction {
    pub sender: SerializablePublicKey,
    pub recipient: SerializablePublicKey,
    pub amount: u64,
    pub signature: SerializableSignature,
}

impl Transaction {
    pub fn new(sender_sk: &SigningKey, recipient: &VerifyingKey, amount: u64) -> Self {
        let sender_pk = sender_sk.verifying_key();
        let tx_data = serde_json::to_vec(&(&sender_pk.to_bytes(), &recipient.to_bytes(), &amount)).unwrap();
        let sig = sender_sk.sign(&tx_data);
        Transaction {
            sender: SerializablePublicKey::from(&sender_pk),
            recipient: SerializablePublicKey::from(recipient),
            amount,
            signature: SerializableSignature::from(&sig),
        }
    }

    pub fn verify(&self) -> bool {
        let sender_vk: VerifyingKey = self.sender.clone().into();
        let sig: Signature = self.signature.clone().into();
        let data = serde_json::to_vec(&(&self.sender.0, &self.recipient.0, &self.amount)).unwrap();
        sender_vk.verify(&data, &sig).is_ok()
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Block {
    pub index: u64,
    pub timestamp: i64,
    pub transactions: Vec<Transaction>,
    pub prev_hash: String,
    pub nonce: u64,
    pub hash: String,
    pub difficulty: u32,
}

impl Block {
    pub fn new(index: u64, transactions: Vec<Transaction>, prev_hash: String, difficulty: u32) -> Self {
        let timestamp = Utc::now().timestamp();
        let mut block = Block {
            index,
            timestamp,
            transactions,
            prev_hash,
            nonce: 0,
            hash: String::new(),
            difficulty,
        };
        block.mine();
        block
    }

    pub fn calculate_hash(&self) -> String {
        let block_data = serde_json::to_string(&(&self.index, &self.timestamp, &self.transactions, &self.prev_hash, &self.nonce)).unwrap();
        let mut hasher = Sha256::new();
        hasher.update(block_data);
        format!("{:x}", hasher.finalize())
    }

    pub fn mine(&mut self) {
        loop {
            self.hash = self.calculate_hash();
            if self.hash.starts_with(&"0".repeat(self.difficulty as usize)) {
                break;
            }
            self.nonce += 1;
        }
        println!("⛏️  Mined block {}: {}", self.index, self.hash);
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Blockchain {
    pub chain: Vec<Block>,
    pub mempool: VecDeque<Transaction>,
    pub difficulty: u32,
}

impl Blockchain {
    pub fn new() -> Self {
        let genesis = Block::new(0, vec![], String::from("0"), 2);
        Blockchain {
            chain: vec![genesis],
            mempool: VecDeque::new(),
            difficulty: 2,
        }
    }

    pub fn add_transaction(&mut self, tx: Transaction) {
        if tx.verify() {
            println!(
                "✅ TX verified: {} → {} : {}",
                tx.sender.0, tx.recipient.0, tx.amount
            );
            self.mempool.push_back(tx);
        } else {
            println!("❌ Invalid transaction rejected");
        }
    }

    pub fn mine_pending(&mut self) {
        if self.mempool.is_empty() {
            println!("No transactions to mine!");
            return;
        }

        let txs: Vec<_> = self.mempool.drain(..).collect();
        let prev_hash = self.chain.last().unwrap().hash.clone();
        let block = Block::new(self.chain.len() as u64, txs, prev_hash, self.difficulty);
        self.chain.push(block);

        if self.chain.len() % 5 == 0 {
            self.difficulty += 1;
            println!("⏫ Difficulty increased to {}", self.difficulty);
        }
    }

    pub fn verify_chain(&self) -> bool {
        for i in 1..self.chain.len() {
            let curr = &self.chain[i];
            let prev = &self.chain[i - 1];
            if curr.prev_hash != prev.hash || curr.calculate_hash() != curr.hash {
                return false;
            }
        }
        true
    }
}