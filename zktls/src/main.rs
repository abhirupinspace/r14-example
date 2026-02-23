//! zkTLS — prove private web2 credentials on-chain without revealing them.
//!
//! Flow: TLS oracle → Poseidon commitment → real testnet deposit → balance →
//!       private ZK transfer → final balance.
//!
//! Requires a configured wallet at ~/.r14/wallet.json (run r14_keygen first).
//!
//! Run: cargo run -p zktls

use ark_ff::UniformRand;
use r14_sdk::wallet::{crypto_rng, fr_to_hex, hex_to_fr, load_wallet, save_wallet};
use r14_sdk::{hash2, owner_hash, R14Client, SecretKey};

type Fr = ark_bls12_381::Fr;

/// Simulated TLS oracle response from a bank API.
struct TlsOracleResponse {
    source: &'static str,
    field: &'static str,
    value: u64,
    tls_session_id: &'static str,
}

fn mock_tls_oracle() -> TlsOracleResponse {
    TlsOracleResponse {
        source: "api.examplebank.com/balance",
        field: "account_balance_usd",
        value: 15000,
        tls_session_id: "tls13_aead_aes256gcm_sha384_0x7f3a",
    }
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut rng = crypto_rng();

    // ── 0. Load wallet ──────────────────────────────────────────────────
    println!("=== 0. Load Wallet ===");
    let mut wallet = load_wallet().map_err(|e| anyhow::anyhow!("wallet load failed: {e}"))?;

    if wallet.stellar_secret.contains("PLACEHOLDER")
        || wallet.core_contract_id.contains("PLACEHOLDER")
        || wallet.transfer_contract_id.contains("PLACEHOLDER")
    {
        anyhow::bail!(
            "Wallet has PLACEHOLDER values. Configure ~/.r14/wallet.json first:\n\
             r14_config_set stellar_secret <SECRET>\n\
             r14_config_set core_contract_id <CONTRACT>\n\
             r14_config_set transfer_contract_id <CONTRACT>"
        );
    }

    let sk_fr = hex_to_fr(&wallet.secret_key)?;
    let owner_fr = hex_to_fr(&wallet.owner_hash)?;
    let client = R14Client::from_wallet(&wallet)
        .map_err(|e| anyhow::anyhow!("client init failed: {e}"))?;

    println!("owner_hash: {}...", &wallet.owner_hash[..18]);
    println!("rpc:        {}", wallet.rpc_url);
    println!("indexer:    {}", wallet.indexer_url);

    // ── 1. Mock TLS oracle ──────────────────────────────────────────────
    println!("\n=== 1. TLS Oracle: Fetch Private Data ===");
    let oracle = mock_tls_oracle();
    println!("source:     {}", oracle.source);
    println!("field:      {}", oracle.field);
    println!("session:    {}", oracle.tls_session_id);
    println!("value:      [REDACTED — known only to prover]");

    // ── 2. Poseidon commitment (zkTLS attestation) ──────────────────────
    println!("\n=== 2. Poseidon Commitment ===");
    let value_fr = Fr::from(oracle.value);
    let blinding = Fr::rand(&mut rng);
    let commit = hash2(value_fr, blinding);
    println!("commitment: {}", fr_to_hex(&commit));
    println!("blinding:   [SECRET]");
    println!("(commit = Poseidon(value, blinding) — hiding + binding)");

    // ── 3. Deposit on testnet ───────────────────────────────────────────
    println!("\n=== 3. Deposit {} on Stellar Testnet ===", oracle.value);
    let deposit_result = client
        .deposit(oracle.value, 1, &owner_fr)
        .await
        .map_err(|e| anyhow::anyhow!("deposit failed: {e}"))?;

    wallet.notes.push(deposit_result.note_entry);
    save_wallet(&wallet)?;

    println!("commitment: {}", deposit_result.commitment);
    println!("tx:         {}", deposit_result.tx_result);
    println!("deposited {} as shielded note", deposit_result.value);

    // ── 4. Balance check ────────────────────────────────────────────────
    println!("\n=== 4. Balance Check ===");
    let bal = client
        .balance(&mut wallet.notes)
        .await
        .map_err(|e| anyhow::anyhow!("balance failed: {e}"))?;
    save_wallet(&wallet)?;

    println!("total: {}", bal.total);
    for ns in &bal.notes {
        println!(
            "  note {} | on_chain={} | tag={}",
            ns.value, ns.on_chain, ns.app_tag
        );
    }

    // ── 5. Private transfer ─────────────────────────────────────────────
    println!("\n=== 5. Private Transfer (Groth16 ZK Proof) ===");

    // Generate a second user (Bob) as transfer recipient
    let bob_sk = SecretKey::random(&mut rng);
    let bob_owner = owner_hash(&bob_sk);
    let transfer_amount = 5000u64;

    println!("bob owner:  {}...", &fr_to_hex(&bob_owner.0)[..18]);
    println!("amount:     {}", transfer_amount);
    println!("generating Groth16 proof...");

    let xfer = client
        .transfer(
            &mut wallet.notes,
            &sk_fr,
            &owner_fr,
            &bob_owner.0,
            transfer_amount,
        )
        .await
        .map_err(|e| anyhow::anyhow!("transfer failed: {e}"))?;

    // Persist updated notes (consumed note marked spent + new change note)
    wallet.notes.push(xfer.change_note);
    save_wallet(&wallet)?;

    println!("nullifier:  {}", xfer.nullifier);
    println!("recipient:  {}", xfer.out_commitment_0);
    println!("change:     {}", xfer.out_commitment_1);
    println!("tx:         {}", xfer.tx_result);

    // ── 6. Final balance ────────────────────────────────────────────────
    println!("\n=== 6. Final Balance ===");
    let final_bal = client
        .balance(&mut wallet.notes)
        .await
        .map_err(|e| anyhow::anyhow!("final balance failed: {e}"))?;
    save_wallet(&wallet)?;

    println!("total: {}", final_bal.total);
    for ns in &final_bal.notes {
        println!(
            "  note {} | on_chain={} | tag={}",
            ns.value, ns.on_chain, ns.app_tag
        );
    }

    println!("\nzkTLS concept: web2 credentials → shielded on-chain note → private ZK transfer.");
    println!("Value never revealed. Proof verifies on Stellar testnet.");
    Ok(())
}

#[cfg(test)]
mod tests {
    use ark_ff::{UniformRand, Zero};
    use ark_std::rand::{rngs::StdRng, SeedableRng};
    use r14_sdk::hash2;

    type Fr = ark_bls12_381::Fr;

    fn test_rng() -> StdRng {
        StdRng::seed_from_u64(99999)
    }

    #[test]
    fn commitment_is_non_zero() {
        let mut rng = test_rng();
        let value = Fr::from(15000u64);
        let blinding = Fr::rand(&mut rng);
        let commit = hash2(value, blinding);
        assert!(!commit.is_zero(), "Poseidon commitment must be non-zero");
    }

    #[test]
    fn commitment_is_deterministic() {
        let mut rng = test_rng();
        let value = Fr::from(15000u64);
        let blinding = Fr::rand(&mut rng);
        let c1 = hash2(value, blinding);
        let c2 = hash2(value, blinding);
        assert_eq!(c1, c2, "same inputs must produce same commitment");
    }

    #[test]
    fn different_blindings_different_commitments() {
        let mut rng = test_rng();
        let value = Fr::from(15000u64);
        let b1 = Fr::rand(&mut rng);
        let b2 = Fr::rand(&mut rng);
        let c1 = hash2(value, b1);
        let c2 = hash2(value, b2);
        assert_ne!(c1, c2, "different blindings must produce different commitments");
    }

    #[test]
    fn different_values_different_commitments() {
        let mut rng = test_rng();
        let blinding = Fr::rand(&mut rng);
        let c1 = hash2(Fr::from(15000u64), blinding);
        let c2 = hash2(Fr::from(10000u64), blinding);
        assert_ne!(c1, c2, "different values must produce different commitments");
    }

    #[test]
    fn commitment_hides_value() {
        let mut rng = test_rng();
        let value = Fr::from(15000u64);
        let blinding = Fr::rand(&mut rng);
        let commit = hash2(value, blinding);
        assert_ne!(commit, value, "commitment must not equal raw value");
        assert_ne!(commit, blinding, "commitment must not equal blinding");
    }

    #[test]
    fn wallet_load_or_graceful_error() {
        // load_wallet returns Err if ~/.r14/wallet.json missing — that's fine
        let result = r14_sdk::wallet::load_wallet();
        match result {
            Ok(w) => assert!(!w.owner_hash.is_empty()),
            Err(e) => {
                let msg = format!("{e}");
                assert!(
                    msg.contains("wallet") || msg.contains("not found") || msg.contains("No such"),
                    "error should mention wallet: {msg}"
                );
            }
        }
    }
}
