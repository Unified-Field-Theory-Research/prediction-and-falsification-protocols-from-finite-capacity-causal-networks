use std::fs;
use std::path::{Path, PathBuf};

use cclab_accel::{
    active_obligation, paper15_skeleton_marker, PFP001UpstreamBinding, Paper15SkeletonCertificate,
    PAPER14_FINAL_CERTIFICATE, PAPER14_FORMAL_ENDPOINT, PAPER14_FROZEN_COMMIT,
};

fn repo_root() -> PathBuf {
    Path::new(env!("CARGO_MANIFEST_DIR")).join("../..")
}

fn read_repo_file(path: &str) -> String {
    fs::read_to_string(repo_root().join(path)).unwrap_or_else(|err| {
        panic!("failed to read {path}: {err}");
    })
}

#[test]
fn required_scaffold_files_exist() {
    for path in [
        "README.md",
        "UPSTREAM-PAPERS.json",
        "GPD/STATE.md",
        "GPD/ROADMAP.md",
        "GPD/state.json",
        "docs/prediction_and_falsification_protocols_theorem.md",
        "docs/open_proof_obligations.md",
        "GPD/formal/FiniteCapacity/PredictionFalsificationProtocols.lean",
    ] {
        assert!(
            repo_root().join(path).exists(),
            "missing required file: {path}"
        );
    }
}

#[test]
fn pfp001_consumes_closed_paper14_chain_without_success_claims() {
    let binding = PFP001UpstreamBinding::canonical();
    assert!(binding.closes_pfp001());
    assert_eq!(binding.paper14_frozen_commit, PAPER14_FROZEN_COMMIT);
    assert_eq!(binding.paper14_formal_endpoint, PAPER14_FORMAL_ENDPOINT);
    assert_eq!(binding.paper14_final_certificate, PAPER14_FINAL_CERTIFICATE);
    assert!(binding
        .claim_boundary
        .all_physical_and_success_claims_remain_false());
}

#[test]
fn initial_skeleton_keeps_paper15_theorem_open() {
    let skeleton = Paper15SkeletonCertificate::initial_pfp001_only();
    assert!(skeleton.pfp001_upstream_binding_closed);
    assert!(!skeleton.pfp002_finite_protocol_record_closed);
    assert!(!skeleton.pfp008_final_conditional_certificate_closed);
    assert!(!skeleton.closes_paper15_theorem());
    assert_eq!(
        paper15_skeleton_marker(),
        "paper15-prediction-falsification-protocols-pfp001-nonpromoting-skeleton"
    );
}

#[test]
fn upstream_json_records_paper14_certificate_and_nonpromotion() {
    let upstream = read_repo_file("UPSTREAM-PAPERS.json");
    assert!(upstream.contains(PAPER14_FROZEN_COMMIT));
    assert!(upstream.contains(PAPER14_FORMAL_ENDPOINT));
    assert!(upstream.contains(PAPER14_FINAL_CERTIFICATE));
    assert!(
        upstream.contains("\"paper14_conditional_discriminating_benchmarks_theorem_closed\": true")
    );
    assert!(upstream.contains("\"paper14_prediction_success_claim\": false"));
    assert!(upstream.contains("\"paper14_falsification_success_claim\": false"));
    assert!(upstream.contains("\"unified_field_theory_claim\": false"));
}

#[test]
fn docs_keep_pfp002_active_and_success_claims_false() {
    let state = read_repo_file("GPD/state.json");
    let state_md = read_repo_file("GPD/STATE.md");
    let theorem = read_repo_file("docs/prediction_and_falsification_protocols_theorem.md");

    assert_eq!(active_obligation(), "PFP-002");
    assert!(state.contains("\"active_obligation\": \"PFP-002\""));
    assert!(state.contains("\"prediction_and_falsification_protocols_theorem_closed\": false"));
    assert!(state.contains("\"protocol_recovery_claim\": false"));
    assert!(state.contains("\"benchmark_success_claim\": false"));
    assert!(state.contains("\"prediction_success_claim\": false"));
    assert!(state.contains("\"falsification_success_claim\": false"));
    assert!(state.contains("\"physical_promotion_claim\": false"));
    assert!(state.contains("\"physical_validation_claim\": false"));
    assert!(state.contains("\"empirical_adequacy_claim\": false"));
    assert!(state_md
        .contains("The local Paper 15 prediction and falsification protocols theorem is not"));
    assert!(theorem.contains("PFP-002"));
    assert!(theorem.contains("no unified field theory claim"));
}

#[test]
fn repository_contains_no_python_artifacts() {
    let forbidden = [
        "pyproject.toml",
        "requirements.txt",
        "setup.py",
        "setup.cfg",
    ];
    for path in forbidden {
        assert!(
            !repo_root().join(path).exists(),
            "unexpected Python artifact: {path}"
        );
    }
}
