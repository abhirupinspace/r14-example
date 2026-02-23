//! zkTLS — prove private web2 credentials on-chain without revealing them.
//!
//! Demonstrates: TLS oracle → Poseidon commitment → range proof → verify.
//! The range proof circuit doesn't exist in r14-sdk yet, so we use the
//! existing Poseidon primitives for commitment and simulate verification.
//!
//! Run: cargo run -p zktls

use ark_ff::UniformRand;
use r14_sdk::wallet::{crypto_rng, fr_to_hex};
use r14_sdk::hash2;

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

    // ── 1. Mock TLS oracle ────────────────────────────────────────────
    println!("=== 1. TLS Oracle: Fetch Private Data ===");
    let oracle = mock_tls_oracle();
    println!("source:     {}", oracle.source);
    println!("field:      {}", oracle.field);
    println!("session:    {}", oracle.tls_session_id);
    println!("value:      [REDACTED — known only to prover]");

    // ── 2. Poseidon commitment ────────────────────────────────────────
    println!("\n=== 2. Poseidon Commitment ===");
    let value_fr = Fr::from(oracle.value);
    let blinding = Fr::rand(&mut rng);
    let commit = hash2(value_fr, blinding);
    println!("commitment: {}", fr_to_hex(&commit));
    println!("blinding:   [SECRET]");
    println!("(commit = Poseidon(value, blinding) — hiding + binding)");

    // ── 3. Range proof: balance > 10000 ───────────────────────────────
    println!("\n=== 3. Range Proof: balance > 10000 ===");
    let threshold = 10000u64;

    // r14-sdk only has a transfer circuit — no range proof circuit yet.
    // We demonstrate what the flow *would* look like and verify the
    // claim locally to show the concept.
    let claim_holds = oracle.value > threshold;

    println!("claim:      \"balance > {threshold}\"");
    println!("circuit:    [simulated — range proof circuit not yet in r14-sdk]");
    println!("status:     {}", if claim_holds { "PASS" } else { "FAIL" });
    println!();
    println!("  In production, the prover would:");
    println!("  1. Open the commitment inside a Groth16 circuit");
    println!("  2. Constrain: committed_value > threshold");
    println!("  3. Output: (commitment, threshold, proof)");
    println!("  4. Verifier checks proof without learning the value");

    // ── 4. Verification summary ───────────────────────────────────────
    println!("\n=== 4. Verification Summary ===");
    println!("┌─────────────────┬────────────────────────────────────────┐");
    println!("│ Claim           │ balance > {:<29}│", format!("{threshold}"));
    println!("│ Data source     │ {:<39}│", oracle.source);
    println!("│ TLS session     │ {:<39}│", oracle.tls_session_id);
    println!("│ Commitment      │ {}..  │", &fr_to_hex(&commit)[..38]);
    println!("│ Value revealed  │ {:<39}│", "NOTHING");
    println!("│ Proof valid     │ {:<39}│", claim_holds);
    println!("└─────────────────┴────────────────────────────────────────┘");

    println!("\nzkTLS concept: web2 credentials → on-chain proof, zero data leaked.");
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
        // commitment should not equal the raw value — Poseidon mixes it
        let mut rng = test_rng();
        let value = Fr::from(15000u64);
        let blinding = Fr::rand(&mut rng);
        let commit = hash2(value, blinding);
        assert_ne!(commit, value, "commitment must not equal raw value");
        assert_ne!(commit, blinding, "commitment must not equal blinding");
    }

    #[test]
    fn range_check_logic() {
        let oracle_value = 15000u64;
        let threshold = 10000u64;
        assert!(oracle_value > threshold, "15000 > 10000 must hold");
        assert!(!(5000u64 > threshold), "5000 > 10000 must not hold");
    }

    #[test]
    fn mock_oracle_returns_expected() {
        let oracle = super::mock_tls_oracle();
        assert_eq!(oracle.value, 15000);
        assert_eq!(oracle.source, "api.examplebank.com/balance");
        assert_eq!(oracle.field, "account_balance_usd");
    }
}
