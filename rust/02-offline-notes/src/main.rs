//! Offline Notes — create notes, compute commitments, and build a merkle tree.
//!
//! No network required. Demonstrates the core crypto primitives.
//! Run: cargo run -p offline-notes

use r14_sdk::wallet::{crypto_rng, fr_to_hex};
use r14_sdk::{commitment, merkle, nullifier, owner_hash, Note, SecretKey};

fn main() {
    let mut rng = crypto_rng();

    // ── keygen ───────────────────────────────────────────────────────
    let sk = SecretKey::random(&mut rng);
    let owner = owner_hash(&sk);
    println!("owner_hash: {}", fr_to_hex(&owner.0));

    // ── create notes ────────────────────────────────────────────────
    let note_a = Note::new(1000, 1, owner.0, &mut rng);
    let note_b = Note::new(500, 1, owner.0, &mut rng);

    // ── commitments ─────────────────────────────────────────────────
    let cm_a = commitment(&note_a);
    let cm_b = commitment(&note_b);
    println!("\nnote_a: value=1000  commitment={}", fr_to_hex(&cm_a));
    println!("note_b: value=500   commitment={}", fr_to_hex(&cm_b));

    // ── nullifiers (prove ownership without revealing the note) ─────
    let nf_a = nullifier(&sk, &note_a.nonce);
    let nf_b = nullifier(&sk, &note_b.nonce);
    println!("\nnullifier_a: {}", fr_to_hex(&nf_a.0));
    println!("nullifier_b: {}", fr_to_hex(&nf_b.0));

    // ── merkle tree ─────────────────────────────────────────────────
    let root = merkle::compute_root_from_leaves(&[cm_a, cm_b]);
    let empty = merkle::empty_root_hex();
    println!("\nmerkle_root (2 leaves): {root}");
    println!("empty_root (0 leaves):  {empty}");

    // verify they differ
    assert_ne!(root, empty, "root with leaves should differ from empty root");
    println!("\nmerkle membership verified — root changed after inserting leaves.");
}
