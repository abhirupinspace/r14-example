//! Private Payments — User A deposits 1000, transfers 300 to User B, both check balances.
//!
//! Full two-party private payment flow with Groth16 ZK proofs (offline).
//! Run: cargo run -p private-payments

use r14_sdk::wallet::{self, crypto_rng, fr_to_hex, NoteEntry};
use r14_sdk::{commitment, merkle, owner_hash, MerklePath, Note, SecretKey, MERKLE_DEPTH};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut rng = crypto_rng();

    // ── 1. User A: keygen ─────────────────────────────────────────────
    println!("=== 1. User A: Keygen ===");
    let user_a_sk = SecretKey::random(&mut rng);
    let user_a_owner = owner_hash(&user_a_sk);
    println!("user_a owner_hash: {}", fr_to_hex(&user_a_owner.0));

    // ── 2. User B: keygen ─────────────────────────────────────────────
    println!("\n=== 2. User B: Keygen ===");
    let user_b_sk = SecretKey::random(&mut rng);
    let user_b_owner = owner_hash(&user_b_sk);
    println!("user_b owner_hash: {}", fr_to_hex(&user_b_owner.0));

    // ── 3. User A: deposit 1000 ───────────────────────────────────────
    println!("\n=== 3. User A: Deposit 1000 ===");
    let user_a_note = Note::new(1000, 1, user_a_owner.0, &mut rng);
    let user_a_cm = commitment(&user_a_note);
    let user_a_entry = NoteEntry {
        value: user_a_note.value,
        app_tag: user_a_note.app_tag,
        owner: fr_to_hex(&user_a_note.owner),
        nonce: fr_to_hex(&user_a_note.nonce),
        commitment: fr_to_hex(&user_a_cm),
        index: Some(0),
        spent: false,
    };
    println!("deposited 1000, commitment={}...", &fr_to_hex(&user_a_cm)[..18]);

    // ── 4. User A → User B: transfer 300 ─────────────────────────────
    println!("\n=== 4. User A: Transfer 300 to User B ===");
    let transfer_value = 300u64;
    let change_value = user_a_entry.value - transfer_value;

    let consumed = Note::with_nonce(
        user_a_entry.value,
        user_a_entry.app_tag,
        wallet::hex_to_fr(&user_a_entry.owner)?,
        wallet::hex_to_fr(&user_a_entry.nonce)?,
    );

    let empty_fr = wallet::hex_to_fr(&merkle::empty_root_hex())?;
    let path = MerklePath {
        siblings: vec![empty_fr; MERKLE_DEPTH],
        indices: vec![false; MERKLE_DEPTH],
    };

    let user_b_note = Note::new(transfer_value, 1, user_b_owner.0, &mut rng);
    let user_a_change = Note::new(change_value, 1, user_a_owner.0, &mut rng);

    println!("generating Groth16 proof...");
    use ark_std::rand::{rngs::StdRng, SeedableRng};
    let (pk, _vk) = r14_sdk::prove::setup(&mut StdRng::seed_from_u64(42));
    let (proof, pi) = r14_sdk::prove::prove(
        &pk,
        user_a_sk.0,
        consumed,
        path,
        [user_b_note.clone(), user_a_change.clone()],
        &mut rng,
    );
    let (sp, _) = r14_sdk::prove::serialize_proof_for_soroban(&proof, &pi.to_vec());

    println!("proof:     a={}...", &sp.a[..20]);
    println!("nullifier: {}...", &fr_to_hex(&pi.nullifier)[..18]);

    // ── 5. Final balances ─────────────────────────────────────────────
    println!("\n=== 5. Final Balances ===");
    let user_b_cm = commitment(&user_b_note);
    let user_a_change_cm = commitment(&user_a_change);

    println!("user_a:");
    println!("  original note (1000): spent");
    println!(
        "  change note ({}):    commitment={}...",
        change_value,
        &fr_to_hex(&user_a_change_cm)[..18]
    );
    println!("  balance: {change_value}");

    println!("user_b:");
    println!(
        "  received note ({}):  commitment={}...",
        transfer_value,
        &fr_to_hex(&user_b_cm)[..18]
    );
    println!("  balance: {transfer_value}");

    println!("\nUser A sent {transfer_value} to User B privately with ZK proof.");
    Ok(())
}

#[cfg(test)]
mod tests {
    use ark_ff::Zero;
    use ark_std::rand::{rngs::StdRng, SeedableRng};
    use r14_sdk::wallet::{self, fr_to_hex};
    use r14_sdk::{commitment, merkle, nullifier, owner_hash};
    use r14_sdk::{MerklePath, Note, SecretKey, MERKLE_DEPTH};

    fn test_rng() -> StdRng {
        StdRng::seed_from_u64(12345)
    }

    #[test]
    fn keygen_produces_distinct_owners() {
        let mut rng = test_rng();
        let sk_a = SecretKey::random(&mut rng);
        let sk_b = SecretKey::random(&mut rng);
        let owner_a = owner_hash(&sk_a);
        let owner_b = owner_hash(&sk_b);
        assert_ne!(owner_a.0, owner_b.0, "two keys must produce different owners");
        assert!(!owner_a.0.is_zero());
        assert!(!owner_b.0.is_zero());
    }

    #[test]
    fn commitment_is_deterministic() {
        let mut rng = test_rng();
        let sk = SecretKey::random(&mut rng);
        let owner = owner_hash(&sk);
        let note = Note::new(1000, 1, owner.0, &mut rng);
        let cm1 = commitment(&note);
        let cm2 = commitment(&note);
        assert_eq!(cm1, cm2, "same note must produce same commitment");
        assert!(!cm1.is_zero());
    }

    #[test]
    fn different_notes_different_commitments() {
        let mut rng = test_rng();
        let sk = SecretKey::random(&mut rng);
        let owner = owner_hash(&sk);
        let note_a = Note::new(1000, 1, owner.0, &mut rng);
        let note_b = Note::new(1000, 1, owner.0, &mut rng);
        // different nonces → different commitments
        assert_ne!(commitment(&note_a), commitment(&note_b));
    }

    #[test]
    fn nullifier_derived_correctly() {
        let mut rng = test_rng();
        let sk = SecretKey::random(&mut rng);
        let note = Note::new(500, 1, owner_hash(&sk).0, &mut rng);
        let nf = nullifier(&sk, &note.nonce);
        assert!(!nf.0.is_zero());
        // same inputs → same nullifier
        let nf2 = nullifier(&sk, &note.nonce);
        assert_eq!(nf.0, nf2.0);
    }

    #[test]
    fn transfer_proof_verifies() {
        let mut rng = test_rng();
        let sk_a = SecretKey::random(&mut rng);
        let owner_a = owner_hash(&sk_a);
        let sk_b = SecretKey::random(&mut rng);
        let owner_b = owner_hash(&sk_b);

        let note_a = Note::new(1000, 1, owner_a.0, &mut rng);

        let empty_fr = wallet::hex_to_fr(&merkle::empty_root_hex()).unwrap();
        let path = MerklePath {
            siblings: vec![empty_fr; MERKLE_DEPTH],
            indices: vec![false; MERKLE_DEPTH],
        };

        let out_b = Note::new(300, 1, owner_b.0, &mut rng);
        let out_change = Note::new(700, 1, owner_a.0, &mut rng);

        let (pk, vk) = r14_sdk::prove::setup(&mut StdRng::seed_from_u64(42));
        let (proof, pi) = r14_sdk::prove::prove(
            &pk,
            sk_a.0,
            note_a,
            path,
            [out_b, out_change],
            &mut rng,
        );

        assert!(
            r14_sdk::prove::verify_offchain(&vk, &proof, &pi),
            "proof must verify"
        );
    }

    #[test]
    fn transfer_balances_sum_correctly() {
        let deposit = 1000u64;
        let transfer = 300u64;
        let change = deposit - transfer;
        assert_eq!(change, 700);
        assert_eq!(transfer + change, deposit, "value must be conserved");
    }

    #[test]
    fn proof_serialization_non_empty() {
        let mut rng = test_rng();
        let sk = SecretKey::random(&mut rng);
        let owner = owner_hash(&sk);

        let note = Note::new(500, 1, owner.0, &mut rng);
        let empty_fr = wallet::hex_to_fr(&merkle::empty_root_hex()).unwrap();
        let path = MerklePath {
            siblings: vec![empty_fr; MERKLE_DEPTH],
            indices: vec![false; MERKLE_DEPTH],
        };
        let out_0 = Note::new(200, 1, owner.0, &mut rng);
        let out_1 = Note::new(300, 1, owner.0, &mut rng);

        let (pk, _vk) = r14_sdk::prove::setup(&mut StdRng::seed_from_u64(42));
        let (proof, pi) = r14_sdk::prove::prove(
            &pk, sk.0, note, path, [out_0, out_1], &mut rng,
        );
        let (sp, pub_inputs) =
            r14_sdk::prove::serialize_proof_for_soroban(&proof, &pi.to_vec());

        assert!(!sp.a.is_empty(), "proof.a must be non-empty");
        assert!(!sp.b.is_empty(), "proof.b must be non-empty");
        assert!(!sp.c.is_empty(), "proof.c must be non-empty");
        assert!(!pub_inputs.is_empty(), "public inputs must be non-empty");
    }

    #[test]
    fn hex_roundtrip() {
        let mut rng = test_rng();
        let sk = SecretKey::random(&mut rng);
        let owner = owner_hash(&sk);
        let hex = fr_to_hex(&owner.0);
        let recovered = wallet::hex_to_fr(&hex).unwrap();
        assert_eq!(owner.0, recovered);
    }
}
