use std::fs;
use std::path::{Path, PathBuf};

use cclab_accel::{
    active_obligation, is_bounded_protocol_label, paper15_skeleton_marker, PFP001UpstreamBinding,
    PFP002FiniteProtocolRecord, PFP003PredictionTargetRegimeDescriptor,
    PFP004FalsificationThresholdDescriptor, PFP005Paper14BenchmarkCompatibility,
    PFP006StabilityReproducibility, PFP007NoHiddenPromotionAudit,
    PFP008FinalConditionalCertificate, Paper15ClaimBoundary, Paper15SkeletonCertificate,
    PAPER14_FINAL_CERTIFICATE, PAPER14_FORMAL_ENDPOINT, PAPER14_FROZEN_COMMIT,
    PAPER15_PFP002_MARKER, PAPER15_PFP003_MARKER, PAPER15_PFP004_MARKER, PAPER15_PFP005_MARKER,
    PAPER15_PFP006_MARKER, PAPER15_PFP007_MARKER, PAPER15_PFP008_MARKER,
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
fn pfp002_defines_finite_protocol_record_without_success_claims() {
    let record = PFP002FiniteProtocolRecord::canonical();
    assert!(record.closes_pfp002());
    assert!(record.all_protocol_labels_are_bounded());
    assert_eq!(
        record.paper14_benchmark_compatibility_reference,
        PAPER14_FORMAL_ENDPOINT
    );
    assert!(record.benchmark_compatibility_only_referenced);
    assert!(record.auditable_interface_row);
    assert!(record
        .claim_boundary
        .all_physical_and_success_claims_remain_false());
    assert_eq!(
        PAPER15_PFP002_MARKER,
        "paper15-prediction-falsification-protocols-pfp002-finite-record"
    );
}

#[test]
fn pfp002_rejects_unbounded_or_promoting_protocol_rows() {
    assert!(is_bounded_protocol_label("finite-target-label"));
    assert!(!is_bounded_protocol_label(""));
    assert!(!is_bounded_protocol_label(
        "protocol-label-that-is-intentionally-longer-than-sixty-four-bytes"
    ));
    assert!(!is_bounded_protocol_label("contains whitespace"));

    let mut promoting_record = PFP002FiniteProtocolRecord::canonical();
    promoting_record.claim_boundary = Paper15ClaimBoundary {
        prediction_success_claim: true,
        ..Paper15ClaimBoundary::non_promoting()
    };
    assert!(!promoting_record.closes_pfp002());

    let mut benchmark_success_record = PFP002FiniteProtocolRecord::canonical();
    benchmark_success_record.claim_boundary = Paper15ClaimBoundary {
        benchmark_success_claim: true,
        ..Paper15ClaimBoundary::non_promoting()
    };
    assert!(!benchmark_success_record.closes_pfp002());
}

#[test]
fn pfp002_skeleton_still_keeps_final_theorem_open() {
    let skeleton = Paper15SkeletonCertificate::pfp002_record_only();
    assert!(skeleton.pfp001_upstream_binding_closed);
    assert!(skeleton.pfp002_finite_protocol_record_closed);
    assert!(!skeleton.pfp003_prediction_target_regime_closed);
    assert!(!skeleton.pfp008_final_conditional_certificate_closed);
    assert!(!skeleton.closes_paper15_theorem());
}

#[test]
fn pfp003_defines_target_observable_regime_descriptors_without_recovery() {
    let descriptors = PFP003PredictionTargetRegimeDescriptor::canonical();
    assert!(descriptors.closes_pfp003());
    assert!(descriptors.protocol_record.closes_pfp002());
    assert!(descriptors.all_descriptor_labels_are_bounded());
    assert!(descriptors.descriptors_linked_to_protocol_row);
    assert!(descriptors.descriptors_are_selection_criteria_only);
    assert!(
        !descriptors
            .claim_boundary
            .observed_particle_catalog_recovery_claim
    );
    assert_eq!(
        PAPER15_PFP003_MARKER,
        "paper15-prediction-falsification-protocols-pfp003-descriptors"
    );
}

#[test]
fn pfp003_rejects_unlinked_catalog_recovery_or_prediction_success() {
    let mut unlinked = PFP003PredictionTargetRegimeDescriptor::canonical();
    unlinked.descriptors_linked_to_protocol_row = false;
    assert!(!unlinked.closes_pfp003());

    let mut recovery = PFP003PredictionTargetRegimeDescriptor::canonical();
    recovery.claim_boundary = Paper15ClaimBoundary {
        observed_particle_catalog_recovery_claim: true,
        ..Paper15ClaimBoundary::non_promoting()
    };
    assert!(!recovery.closes_pfp003());

    let mut prediction_success = PFP003PredictionTargetRegimeDescriptor::canonical();
    prediction_success.claim_boundary = Paper15ClaimBoundary {
        prediction_success_claim: true,
        ..Paper15ClaimBoundary::non_promoting()
    };
    assert!(!prediction_success.closes_pfp003());
}

#[test]
fn pfp003_skeleton_still_keeps_final_theorem_open() {
    let skeleton = Paper15SkeletonCertificate::pfp003_descriptor_only();
    assert!(skeleton.pfp001_upstream_binding_closed);
    assert!(skeleton.pfp002_finite_protocol_record_closed);
    assert!(skeleton.pfp003_prediction_target_regime_closed);
    assert!(!skeleton.pfp004_falsification_threshold_closed);
    assert!(!skeleton.closes_paper15_theorem());
}

#[test]
fn pfp004_defines_threshold_and_rejection_descriptors_without_success() {
    let threshold = PFP004FalsificationThresholdDescriptor::canonical();
    assert!(threshold.closes_pfp004());
    assert!(threshold.descriptor_record.closes_pfp003());
    assert!(threshold.all_threshold_labels_are_bounded());
    assert!(threshold.threshold_linked_to_protocol_row);
    assert!(threshold.rejection_condition_linked_to_protocol_row);
    assert!(threshold.threshold_is_finite_criterion_only);
    assert!(threshold.rejection_condition_predeclared);
    assert_eq!(
        PAPER15_PFP004_MARKER,
        "paper15-prediction-falsification-protocols-pfp004-thresholds"
    );
}

#[test]
fn pfp004_rejects_falsification_success_or_fit_only_shortcut() {
    let mut falsification_success = PFP004FalsificationThresholdDescriptor::canonical();
    falsification_success.claim_boundary = Paper15ClaimBoundary {
        falsification_success_claim: true,
        ..Paper15ClaimBoundary::non_promoting()
    };
    assert!(!falsification_success.closes_pfp004());

    let mut fit_only = PFP004FalsificationThresholdDescriptor::canonical();
    fit_only.claim_boundary = Paper15ClaimBoundary {
        fit_only_calibration_claim: true,
        ..Paper15ClaimBoundary::non_promoting()
    };
    assert!(!fit_only.closes_pfp004());

    let mut not_predeclared = PFP004FalsificationThresholdDescriptor::canonical();
    not_predeclared.rejection_condition_predeclared = false;
    assert!(!not_predeclared.closes_pfp004());
}

#[test]
fn pfp004_skeleton_still_keeps_final_theorem_open() {
    let skeleton = Paper15SkeletonCertificate::pfp004_threshold_only();
    assert!(skeleton.pfp001_upstream_binding_closed);
    assert!(skeleton.pfp002_finite_protocol_record_closed);
    assert!(skeleton.pfp003_prediction_target_regime_closed);
    assert!(skeleton.pfp004_falsification_threshold_closed);
    assert!(!skeleton.pfp005_paper14_benchmark_compatibility_closed);
    assert!(!skeleton.closes_paper15_theorem());
}

#[test]
fn pfp005_defines_paper14_compatibility_without_benchmark_success() {
    let compatibility = PFP005Paper14BenchmarkCompatibility::canonical();
    assert!(compatibility.closes_pfp005());
    assert!(compatibility.threshold_record.closes_pfp004());
    assert!(compatibility.references_frozen_paper14_certificate());
    assert_eq!(compatibility.paper14_frozen_commit, PAPER14_FROZEN_COMMIT);
    assert_eq!(
        compatibility.paper14_formal_endpoint,
        PAPER14_FORMAL_ENDPOINT
    );
    assert_eq!(
        compatibility.paper14_final_certificate,
        PAPER14_FINAL_CERTIFICATE
    );
    assert!(compatibility.compatibility_is_schema_alignment_only);
    assert_eq!(
        PAPER15_PFP005_MARKER,
        "paper15-prediction-falsification-protocols-pfp005-paper14-compat"
    );
}

#[test]
fn pfp005_rejects_benchmark_success_or_bad_paper14_binding() {
    let mut benchmark_success = PFP005Paper14BenchmarkCompatibility::canonical();
    benchmark_success.claim_boundary = Paper15ClaimBoundary {
        benchmark_success_claim: true,
        ..Paper15ClaimBoundary::non_promoting()
    };
    assert!(!benchmark_success.closes_pfp005());

    let mut prediction_success = PFP005Paper14BenchmarkCompatibility::canonical();
    prediction_success.claim_boundary = Paper15ClaimBoundary {
        prediction_success_claim: true,
        ..Paper15ClaimBoundary::non_promoting()
    };
    assert!(!prediction_success.closes_pfp005());

    let mut bad_binding = PFP005Paper14BenchmarkCompatibility::canonical();
    bad_binding.paper14_frozen_commit = "not-a-frozen-paper14-commit";
    assert!(!bad_binding.closes_pfp005());
}

#[test]
fn pfp005_skeleton_still_keeps_final_theorem_open() {
    let skeleton = Paper15SkeletonCertificate::pfp005_benchmark_compatibility_only();
    assert!(skeleton.pfp001_upstream_binding_closed);
    assert!(skeleton.pfp002_finite_protocol_record_closed);
    assert!(skeleton.pfp003_prediction_target_regime_closed);
    assert!(skeleton.pfp004_falsification_threshold_closed);
    assert!(skeleton.pfp005_paper14_benchmark_compatibility_closed);
    assert!(!skeleton.pfp006_stability_reproducibility_closed);
    assert!(!skeleton.closes_paper15_theorem());
}

#[test]
fn pfp006_defines_stability_reproducibility_without_empirical_success() {
    let stability = PFP006StabilityReproducibility::canonical();
    assert!(stability.closes_pfp006());
    assert!(stability.compatibility_record.closes_pfp005());
    assert!(stability.all_stability_labels_are_bounded());
    assert!(stability.stability_preserves_protocol_labels);
    assert!(stability.coarse_graining_preserves_audit_boundary);
    assert!(stability.reproducibility_preserves_claim_boundary);
    assert!(stability.reproducibility_is_finite_record_compatibility);
    assert_eq!(
        PAPER15_PFP006_MARKER,
        "paper15-prediction-falsification-protocols-pfp006-stability"
    );
}

#[test]
fn pfp006_rejects_reproduced_success_validation_or_boundary_loss() {
    let mut validation = PFP006StabilityReproducibility::canonical();
    validation.claim_boundary = Paper15ClaimBoundary {
        physical_validation_claim: true,
        ..Paper15ClaimBoundary::non_promoting()
    };
    assert!(!validation.closes_pfp006());

    let mut empirical = PFP006StabilityReproducibility::canonical();
    empirical.claim_boundary = Paper15ClaimBoundary {
        empirical_adequacy_claim: true,
        ..Paper15ClaimBoundary::non_promoting()
    };
    assert!(!empirical.closes_pfp006());

    let mut boundary_loss = PFP006StabilityReproducibility::canonical();
    boundary_loss.reproducibility_preserves_claim_boundary = false;
    assert!(!boundary_loss.closes_pfp006());
}

#[test]
fn pfp006_skeleton_still_keeps_final_theorem_open() {
    let skeleton = Paper15SkeletonCertificate::pfp006_stability_only();
    assert!(skeleton.pfp001_upstream_binding_closed);
    assert!(skeleton.pfp002_finite_protocol_record_closed);
    assert!(skeleton.pfp003_prediction_target_regime_closed);
    assert!(skeleton.pfp004_falsification_threshold_closed);
    assert!(skeleton.pfp005_paper14_benchmark_compatibility_closed);
    assert!(skeleton.pfp006_stability_reproducibility_closed);
    assert!(!skeleton.pfp007_no_hidden_promotion_validation_success_audit_closed);
    assert!(!skeleton.closes_paper15_theorem());
}

#[test]
fn pfp007_audits_hidden_promotion_validation_success_routes() {
    let audit = PFP007NoHiddenPromotionAudit::canonical();
    assert!(audit.closes_pfp007());
    assert!(audit.stability_record.closes_pfp006());
    assert!(audit.finite_audit_row);
    assert!(audit.fail_closed_audit);
    assert!(audit.all_protocol_routes_audited);
    assert!(audit.all_hidden_routes_blocked());
    assert_eq!(
        PAPER15_PFP007_MARKER,
        "paper15-prediction-falsification-protocols-pfp007-hidden-audit"
    );
}

#[test]
fn pfp007_rejects_unblocked_hidden_routes_or_promoting_boundary() {
    let mut hidden_prediction = PFP007NoHiddenPromotionAudit::canonical();
    hidden_prediction.no_hidden_prediction_success_import = false;
    assert!(!hidden_prediction.closes_pfp007());

    let mut not_fail_closed = PFP007NoHiddenPromotionAudit::canonical();
    not_fail_closed.fail_closed_audit = false;
    assert!(!not_fail_closed.closes_pfp007());

    let mut physical_nature = PFP007NoHiddenPromotionAudit::canonical();
    physical_nature.claim_boundary = Paper15ClaimBoundary {
        physical_nature_claim: true,
        ..Paper15ClaimBoundary::non_promoting()
    };
    assert!(!physical_nature.closes_pfp007());
}

#[test]
fn pfp007_skeleton_still_keeps_final_theorem_open() {
    let skeleton = Paper15SkeletonCertificate::pfp007_hidden_audit_only();
    assert!(skeleton.pfp001_upstream_binding_closed);
    assert!(skeleton.pfp002_finite_protocol_record_closed);
    assert!(skeleton.pfp003_prediction_target_regime_closed);
    assert!(skeleton.pfp004_falsification_threshold_closed);
    assert!(skeleton.pfp005_paper14_benchmark_compatibility_closed);
    assert!(skeleton.pfp006_stability_reproducibility_closed);
    assert!(skeleton.pfp007_no_hidden_promotion_validation_success_audit_closed);
    assert!(!skeleton.pfp008_final_conditional_certificate_closed);
    assert!(!skeleton.closes_paper15_theorem());
}

#[test]
fn pfp008_final_certificate_closes_conditional_theorem_only() {
    let certificate = PFP008FinalConditionalCertificate::canonical();
    let theorem = Paper15SkeletonCertificate::final_conditional_certificate();

    assert!(certificate.closes_pfp008());
    assert!(certificate.upstream_binding.closes_pfp001());
    assert!(certificate.hidden_audit.closes_pfp007());
    assert!(certificate.finite_protocol_interface_constructed);
    assert!(certificate.final_certificate_conditional);
    assert!(certificate.all_claim_boundaries_preserved);
    assert!(theorem.closes_paper15_theorem());
    assert!(theorem
        .claim_boundary
        .all_physical_and_success_claims_remain_false());
    assert_eq!(
        PAPER15_PFP008_MARKER,
        "paper15-prediction-falsification-protocols-pfp008-final-certificate"
    );
}

#[test]
fn pfp008_rejects_nonconditional_or_promoting_final_certificate() {
    let mut nonconditional = PFP008FinalConditionalCertificate::canonical();
    nonconditional.final_certificate_conditional = false;
    assert!(!nonconditional.closes_pfp008());

    let mut boundary_not_preserved = PFP008FinalConditionalCertificate::canonical();
    boundary_not_preserved.all_claim_boundaries_preserved = false;
    assert!(!boundary_not_preserved.closes_pfp008());

    let mut promoting = PFP008FinalConditionalCertificate::canonical();
    promoting.claim_boundary = Paper15ClaimBoundary {
        unified_field_theory_claim: true,
        ..Paper15ClaimBoundary::non_promoting()
    };
    assert!(!promoting.closes_pfp008());
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
fn docs_record_no_active_obligation_and_success_claims_false() {
    let state = read_repo_file("GPD/state.json");
    let state_md = read_repo_file("GPD/STATE.md");
    let theorem = read_repo_file("docs/prediction_and_falsification_protocols_theorem.md");

    assert_eq!(active_obligation(), "NONE");
    assert!(state.contains("\"active_obligation\": \"NONE\""));
    assert!(state.contains("\"prediction_and_falsification_protocols_theorem_closed\": true"));
    assert!(state.contains("\"pfp002_finite_protocol_record_closed\": true"));
    assert!(state.contains("\"pfp003_prediction_target_regime_closed\": true"));
    assert!(state.contains("\"pfp004_falsification_threshold_closed\": true"));
    assert!(state.contains("\"pfp005_paper14_benchmark_compatibility_closed\": true"));
    assert!(state.contains("\"pfp006_stability_reproducibility_closed\": true"));
    assert!(state.contains("\"pfp007_no_hidden_promotion_validation_success_audit_closed\": true"));
    assert!(state.contains("\"pfp008_final_conditional_certificate_closed\": true"));
    assert!(state.contains("\"protocol_recovery_claim\": false"));
    assert!(state.contains("\"benchmark_success_claim\": false"));
    assert!(state.contains("\"prediction_success_claim\": false"));
    assert!(state.contains("\"falsification_success_claim\": false"));
    assert!(state.contains("\"physical_promotion_claim\": false"));
    assert!(state.contains("\"physical_validation_claim\": false"));
    assert!(state.contains("\"empirical_adequacy_claim\": false"));
    assert!(state_md
        .contains("The local Paper 15 prediction and falsification protocols theorem is closed"));
    assert!(theorem.contains("No active rungs remain"));
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
