//! Two-Party Transfer — Alice deposits, transfers to Bob, both check balances.
//!
//! Demonstrates the full two-party private payment flow offline.
//! Run: cargo run -p two-party

use r14_sdk::wallet::{self, crypto_rng, fr_to_hex, NoteEntry};
use r14_sdk::{commitment, merkle, owner_hash, MerklePath, Note, SecretKey, MERKLE_DEPTH};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut rng = crypto_rng();

    // ── Alice: keygen ───────────────────────────────────────────────
    println!("=== Alice: Keygen ===");
    let alice_sk = SecretKey::random(&mut rng);
    let alice_owner = owner_hash(&alice_sk);
    println!("alice owner_hash: {}", fr_to_hex(&alice_owner.0));

    // ── Bob: keygen ─────────────────────────────────────────────────
    println!("\n=== Bob: Keygen ===");
    let bob_sk = SecretKey::random(&mut rng);
    let bob_owner = owner_hash(&bob_sk);
    println!("bob owner_hash:   {}", fr_to_hex(&bob_owner.0));

    // ── Alice: deposit 1000 ─────────────────────────────────────────
    println!("\n=== Alice: Deposit 1000 ===");
    let alice_note = Note::new(1000, 1, alice_owner.0, &mut rng);
    let alice_cm = commitment(&alice_note);
    let alice_entry = NoteEntry {
        value: alice_note.value,
        app_tag: alice_note.app_tag,
        owner: fr_to_hex(&alice_note.owner),
        nonce: fr_to_hex(&alice_note.nonce),
        commitment: fr_to_hex(&alice_cm),
        index: Some(0), // simulate on-chain
        spent: false,
    };
    println!("deposited 1000, commitment={}...", &fr_to_hex(&alice_cm)[..18]);

    // ── Bob: shares owner_hash with Alice ───────────────────────────
    println!("\n=== Bob shares owner_hash with Alice ===");
    println!("bob sends: {}", fr_to_hex(&bob_owner.0));

    // ── Alice: transfer 300 to Bob ──────────────────────────────────
    println!("\n=== Alice: Transfer 300 to Bob ===");
    let transfer_value = 300u64;
    let change_value = alice_entry.value - transfer_value;

    let consumed = Note::with_nonce(
        alice_entry.value,
        alice_entry.app_tag,
        wallet::hex_to_fr(&alice_entry.owner)?,
        wallet::hex_to_fr(&alice_entry.nonce)?,
    );

    let empty_fr = wallet::hex_to_fr(&merkle::empty_root_hex())?;
    let path = MerklePath {
        siblings: vec![empty_fr; MERKLE_DEPTH],
        indices: vec![false; MERKLE_DEPTH],
    };

    // output notes: 300 to Bob, 700 change to Alice
    let bob_note = Note::new(transfer_value, 1, bob_owner.0, &mut rng);
    let alice_change = Note::new(change_value, 1, alice_owner.0, &mut rng);

    println!("generating Groth16 proof...");
    use ark_std::rand::{rngs::StdRng, SeedableRng};
    let (pk, _vk) = r14_sdk::prove::setup(&mut StdRng::seed_from_u64(42));
    let (proof, pi) = r14_sdk::prove::prove(
        &pk,
        alice_sk.0,
        consumed,
        path,
        [bob_note.clone(), alice_change.clone()],
        &mut rng,
    );
    let (sp, _) = r14_sdk::prove::serialize_proof_for_soroban(&proof, &pi.to_vec());
    println!("proof: a={}...", &sp.a[..20]);
    println!("nullifier: {}...", &fr_to_hex(&pi.nullifier)[..18]);

    // ── Balances ────────────────────────────────────────────────────
    println!("\n=== Final Balances ===");

    let bob_cm = commitment(&bob_note);
    let alice_change_cm = commitment(&alice_change);

    println!("alice:");
    println!("  original note (1000): spent");
    println!("  change note ({}):    commitment={}...", change_value, &fr_to_hex(&alice_change_cm)[..18]);
    println!("  balance: {change_value}");

    println!("bob:");
    println!("  received note ({}):  commitment={}...", transfer_value, &fr_to_hex(&bob_cm)[..18]);
    println!("  balance: {transfer_value}");

    println!("\ndone. Alice sent {transfer_value} to Bob privately with ZK proof.");
    Ok(())
}
