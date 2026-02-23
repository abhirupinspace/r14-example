//! Basic Keygen — generate a Root14 keypair and print the owner hash.
//!
//! No network, no ZK proofs. Just key generation.
//! Run: cargo run -p basic-keygen

use r14_sdk::wallet::{crypto_rng, fr_to_hex};
use r14_sdk::{owner_hash, SecretKey};

fn main() {
    let mut rng = crypto_rng();

    // generate a random secret key
    let sk = SecretKey::random(&mut rng);

    // compute the public owner hash (this is what recipients share)
    let oh = owner_hash(&sk);

    println!("secret_key:  {}", fr_to_hex(&sk.0));
    println!("owner_hash:  {}", fr_to_hex(&oh.0));
    println!();
    println!("The owner_hash is your public identifier.");
    println!("Share it with senders. Keep the secret_key private.");
}
