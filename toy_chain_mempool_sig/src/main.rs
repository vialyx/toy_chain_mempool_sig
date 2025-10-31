use toy_chain_mempool_sig::*;
use ed25519_dalek::SigningKey;
use rand::rngs::OsRng;

fn main() {
    let mut rng = OsRng;
    let sender = SigningKey::generate(&mut rng);
    let recipient = SigningKey::generate(&mut rng);

    let mut chain = Blockchain::new();

    let tx1 = Transaction::new(&sender, &recipient.verifying_key(), 50);
    chain.add_transaction(tx1);

    chain.mine_pending();
    println!("Blockchain valid: {}", chain.verify_chain());
}