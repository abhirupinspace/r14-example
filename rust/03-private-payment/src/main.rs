//! Private Payment — full deposit → transfer → balance flow.
//!
//! Set env vars for live testnet, or run offline by default.
//!   R14_INDEXER_URL, R14_STELLAR_SECRET, R14_CORE_CONTRACT, R14_TRANSFER_CONTRACT
//!
//! Run: cargo run -p private-payment

use r14_sdk::wallet::{self, crypto_rng, fr_to_hex, NoteEntry};
use r14_sdk::{commitment, merkle, owner_hash, MerklePath, Note, SecretKey, MERKLE_DEPTH};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let indexer = std::env::var("R14_INDEXER_URL").unwrap_or("http://localhost:3000".into());
    let secret = std::env::var("R14_STELLAR_SECRET").unwrap_or("PLACEHOLDER".into());
    let core_id = std::env::var("R14_CORE_CONTRACT").unwrap_or("PLACEHOLDER".into());
    let xfer_id = std::env::var("R14_TRANSFER_CONTRACT").unwrap_or("PLACEHOLDER".into());
    let live = secret != "PLACEHOLDER" && xfer_id != "PLACEHOLDER";

    let mut rng = crypto_rng();

    // ── 1. keygen ───────────────────────────────────────────────────
    println!("=== 1. Keygen ===");
    let sk = SecretKey::random(&mut rng);
    let owner = owner_hash(&sk);
    println!("owner_hash: {}", fr_to_hex(&owner.0));

    // ── 2. deposit (create note) ────────────────────────────────────
    println!("\n=== 2. Deposit ===");
    let note = Note::new(1000, 1, owner.0, &mut rng);
    let cm = commitment(&note);
    let mut entry = NoteEntry {
        value: note.value,
        app_tag: note.app_tag,
        owner: fr_to_hex(&note.owner),
        nonce: fr_to_hex(&note.nonce),
        commitment: fr_to_hex(&cm),
        index: None,
        spent: false,
    };
    println!("note: value=1000  commitment={}...", &fr_to_hex(&cm)[..18]);

    if live {
        let contracts = r14_sdk::R14Contracts { core: core_id, transfer: xfer_id };
        let client = r14_sdk::R14Client::new(&indexer, contracts, &secret, "testnet")?;
        let res = client.deposit(1000, 1, &owner.0).await?;
        entry = res.note_entry;
        println!("on-chain tx: {}", res.tx_result);
    } else {
        entry.index = Some(0); // simulate on-chain for proof demo
        println!("(offline mode — skipping on-chain deposit)");
    }

    // ── 3. transfer (ZK proof) ──────────────────────────────────────
    println!("\n=== 3. Transfer ===");
    let recipient_sk = SecretKey::random(&mut rng);
    let recipient = owner_hash(&recipient_sk);
    println!("recipient: {}...", &fr_to_hex(&recipient.0)[..18]);

    let transfer_value = 300u64;
    let change_value = entry.value - transfer_value;

    // reconstruct consumed note
    let consumed = Note::with_nonce(
        entry.value,
        entry.app_tag,
        wallet::hex_to_fr(&entry.owner)?,
        wallet::hex_to_fr(&entry.nonce)?,
    );

    // build single-leaf merkle path (offline demo)
    let empty_fr = wallet::hex_to_fr(&merkle::empty_root_hex())?;
    let path = MerklePath {
        siblings: vec![empty_fr; MERKLE_DEPTH],
        indices: vec![false; MERKLE_DEPTH],
    };

    // output notes
    let out_0 = Note::new(transfer_value, 1, recipient.0, &mut rng);
    let out_1 = Note::new(change_value, 1, owner.0, &mut rng);

    // generate proof
    println!("generating Groth16 proof...");
    use ark_std::rand::{rngs::StdRng, SeedableRng};
    let (pk, _vk) = r14_sdk::prove::setup(&mut StdRng::seed_from_u64(42));
    let (proof, pi) = r14_sdk::prove::prove(
        &pk,
        sk.0,
        consumed,
        path,
        [out_0, out_1],
        &mut rng,
    );
    let (sp, _) = r14_sdk::prove::serialize_proof_for_soroban(&proof, &pi.to_vec());
    println!("proof generated! a={}...", &sp.a[..20]);
    println!("nullifier: {}...", &fr_to_hex(&pi.nullifier)[..18]);
    println!("transferred {transfer_value} to recipient, change {change_value} to self");

    // ── 4. balance ──────────────────────────────────────────────────
    println!("\n=== 4. Balance ===");
    println!("total: {} (before transfer)", entry.value);
    println!("after transfer: {} (change note)", change_value);

    println!("\ndone. set R14_* env vars to run live on Stellar testnet.");
    Ok(())
}
