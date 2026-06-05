pub const PAPER1_FROZEN_COMMIT: &str = "3a9637c65f783ca35e77118f83560290f42f3085";
pub const PAPER2_FROZEN_COMMIT: &str = "053842ef5e1a50282df9d884266e87428ee07f60";
pub const PAPER3_FROZEN_COMMIT: &str = "6067360758108f799fa604855f5513545019492e";
pub const PAPER4_FROZEN_COMMIT: &str = "5a1ac95700786b697a0f25ddecb393fdeaaa166e";
pub const PAPER5_FROZEN_COMMIT: &str = "8db1a334b0c0ca934ccd3628add72c6e3f1ebfcb";
pub const PAPER6_FROZEN_COMMIT: &str = "20df751a0ceb2b4eb33a80dd15dd2795a1ea529a";
pub const PAPER7_FROZEN_COMMIT: &str = "4f52d9980f62977016ef5ee5da9e88a32dce70e5";
pub const PAPER8_FROZEN_COMMIT: &str = "d3c58356cdbe89d9a8b7a79784c7b6eaf4023b33";
pub const PAPER9_FROZEN_COMMIT: &str = "be6e37e43cfa63319d097f70d84de6a24c5b31fd";
pub const PAPER10_FROZEN_COMMIT: &str = "9d9063fa99a69cae3699f892891dde29e2c32d83";
pub const PAPER11_FROZEN_COMMIT: &str = "0e171b833d19216785f7e24c8cddb6e6fe5d39d0";
pub const PAPER12_FROZEN_COMMIT: &str = "42899acf2a84748e713b5f14cfb5e10c38e4bb3b";
pub const PAPER13_FROZEN_COMMIT: &str = "e3c2aaf67fc546c636d7901679ff0c3a4dc5a4ee";
pub const PAPER14_FROZEN_COMMIT: &str = "ad4f876a1699874cd6efd7fe73d318e64f5bbe19";

pub const PAPER14_FORMAL_ENDPOINT: &str = "Paper14DiscriminatingBenchmarksTheoremContract.closed";
pub const PAPER14_FINAL_CERTIFICATE: &str =
    "paper14_dbm008_final_conditional_certificate_closes_discriminating_benchmarks_theorem";

pub const PAPER15_SKELETON_MARKER: &str =
    "paper15-prediction-falsification-protocols-pfp001-nonpromoting-skeleton";
pub const PAPER15_PFP002_MARKER: &str =
    "paper15-prediction-falsification-protocols-pfp002-finite-record";
pub const PAPER15_PFP003_MARKER: &str =
    "paper15-prediction-falsification-protocols-pfp003-descriptors";
pub const PAPER15_PFP004_MARKER: &str =
    "paper15-prediction-falsification-protocols-pfp004-thresholds";
pub const PAPER15_PFP005_MARKER: &str =
    "paper15-prediction-falsification-protocols-pfp005-paper14-compat";
pub const FINITE_PROTOCOL_LABEL_MAX_BYTES: usize = 64;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UpstreamPaper {
    pub paper: u8,
    pub frozen_commit: &'static str,
    pub theorem_closed: bool,
    pub physical_nature_claim: bool,
    pub unified_field_theory_claim: bool,
}

impl UpstreamPaper {
    pub const fn new(
        paper: u8,
        frozen_commit: &'static str,
        theorem_closed: bool,
        physical_nature_claim: bool,
        unified_field_theory_claim: bool,
    ) -> Self {
        Self {
            paper,
            frozen_commit,
            theorem_closed,
            physical_nature_claim,
            unified_field_theory_claim,
        }
    }

    pub fn closes_internal_conditional_without_physical_claim(&self) -> bool {
        self.theorem_closed && !self.physical_nature_claim && !self.unified_field_theory_claim
    }
}

pub const UPSTREAM_CHAIN: [UpstreamPaper; 14] = [
    UpstreamPaper::new(1, PAPER1_FROZEN_COMMIT, true, false, false),
    UpstreamPaper::new(2, PAPER2_FROZEN_COMMIT, true, false, false),
    UpstreamPaper::new(3, PAPER3_FROZEN_COMMIT, true, false, false),
    UpstreamPaper::new(4, PAPER4_FROZEN_COMMIT, true, false, false),
    UpstreamPaper::new(5, PAPER5_FROZEN_COMMIT, true, false, false),
    UpstreamPaper::new(6, PAPER6_FROZEN_COMMIT, true, false, false),
    UpstreamPaper::new(7, PAPER7_FROZEN_COMMIT, true, false, false),
    UpstreamPaper::new(8, PAPER8_FROZEN_COMMIT, true, false, false),
    UpstreamPaper::new(9, PAPER9_FROZEN_COMMIT, true, false, false),
    UpstreamPaper::new(10, PAPER10_FROZEN_COMMIT, true, false, false),
    UpstreamPaper::new(11, PAPER11_FROZEN_COMMIT, true, false, false),
    UpstreamPaper::new(12, PAPER12_FROZEN_COMMIT, true, false, false),
    UpstreamPaper::new(13, PAPER13_FROZEN_COMMIT, true, false, false),
    UpstreamPaper::new(14, PAPER14_FROZEN_COMMIT, true, false, false),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Paper15ClaimBoundary {
    pub protocol_recovery_claim: bool,
    pub benchmark_success_claim: bool,
    pub prediction_success_claim: bool,
    pub falsification_success_claim: bool,
    pub physical_promotion_claim: bool,
    pub physical_validation_claim: bool,
    pub empirical_adequacy_claim: bool,
    pub observed_particle_catalog_recovery_claim: bool,
    pub physical_standard_model_claim: bool,
    pub physical_particle_excitation_claim: bool,
    pub physical_matter_field_claim: bool,
    pub physical_gauge_field_claim: bool,
    pub physical_quantum_dynamics_claim: bool,
    pub continuum_quantum_field_theory_claim: bool,
    pub simulation_only_promotion: bool,
    pub fit_only_calibration_claim: bool,
    pub physical_nature_claim: bool,
    pub unified_field_theory_claim: bool,
}

impl Paper15ClaimBoundary {
    pub const fn non_promoting() -> Self {
        Self {
            protocol_recovery_claim: false,
            benchmark_success_claim: false,
            prediction_success_claim: false,
            falsification_success_claim: false,
            physical_promotion_claim: false,
            physical_validation_claim: false,
            empirical_adequacy_claim: false,
            observed_particle_catalog_recovery_claim: false,
            physical_standard_model_claim: false,
            physical_particle_excitation_claim: false,
            physical_matter_field_claim: false,
            physical_gauge_field_claim: false,
            physical_quantum_dynamics_claim: false,
            continuum_quantum_field_theory_claim: false,
            simulation_only_promotion: false,
            fit_only_calibration_claim: false,
            physical_nature_claim: false,
            unified_field_theory_claim: false,
        }
    }

    pub fn all_physical_and_success_claims_remain_false(&self) -> bool {
        !self.protocol_recovery_claim
            && !self.benchmark_success_claim
            && !self.prediction_success_claim
            && !self.falsification_success_claim
            && !self.physical_promotion_claim
            && !self.physical_validation_claim
            && !self.empirical_adequacy_claim
            && !self.observed_particle_catalog_recovery_claim
            && !self.physical_standard_model_claim
            && !self.physical_particle_excitation_claim
            && !self.physical_matter_field_claim
            && !self.physical_gauge_field_claim
            && !self.physical_quantum_dynamics_claim
            && !self.continuum_quantum_field_theory_claim
            && !self.simulation_only_promotion
            && !self.fit_only_calibration_claim
            && !self.physical_nature_claim
            && !self.unified_field_theory_claim
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PFP001UpstreamBinding {
    pub upstream_chain: &'static [UpstreamPaper],
    pub paper14_frozen_commit: &'static str,
    pub paper14_formal_endpoint: &'static str,
    pub paper14_final_certificate: &'static str,
    pub finite_capacity_boundary_preserved: bool,
    pub locality_boundary_preserved: bool,
    pub bounded_transfer_boundary_preserved: bool,
    pub no_physical_promotion_imported: bool,
    pub no_physical_validation_imported: bool,
    pub no_empirical_adequacy_imported: bool,
    pub no_benchmark_success_imported: bool,
    pub no_prediction_success_imported: bool,
    pub no_falsification_success_imported: bool,
    pub no_observed_catalog_recovery_imported: bool,
    pub no_simulation_only_promotion_imported: bool,
    pub no_fit_only_calibration_imported: bool,
    pub no_unified_field_theory_claim_imported: bool,
    pub claim_boundary: Paper15ClaimBoundary,
}

impl PFP001UpstreamBinding {
    pub const fn canonical() -> Self {
        Self {
            upstream_chain: &UPSTREAM_CHAIN,
            paper14_frozen_commit: PAPER14_FROZEN_COMMIT,
            paper14_formal_endpoint: PAPER14_FORMAL_ENDPOINT,
            paper14_final_certificate: PAPER14_FINAL_CERTIFICATE,
            finite_capacity_boundary_preserved: true,
            locality_boundary_preserved: true,
            bounded_transfer_boundary_preserved: true,
            no_physical_promotion_imported: true,
            no_physical_validation_imported: true,
            no_empirical_adequacy_imported: true,
            no_benchmark_success_imported: true,
            no_prediction_success_imported: true,
            no_falsification_success_imported: true,
            no_observed_catalog_recovery_imported: true,
            no_simulation_only_promotion_imported: true,
            no_fit_only_calibration_imported: true,
            no_unified_field_theory_claim_imported: true,
            claim_boundary: Paper15ClaimBoundary::non_promoting(),
        }
    }

    pub fn closes_pfp001(&self) -> bool {
        self.upstream_chain.len() == 14
            && self.upstream_chain.iter().enumerate().all(|(idx, paper)| {
                paper.paper as usize == idx + 1
                    && is_sha1_hex(paper.frozen_commit)
                    && paper.closes_internal_conditional_without_physical_claim()
            })
            && self.paper14_frozen_commit == PAPER14_FROZEN_COMMIT
            && self.paper14_formal_endpoint == PAPER14_FORMAL_ENDPOINT
            && self.paper14_final_certificate == PAPER14_FINAL_CERTIFICATE
            && self.finite_capacity_boundary_preserved
            && self.locality_boundary_preserved
            && self.bounded_transfer_boundary_preserved
            && self.no_physical_promotion_imported
            && self.no_physical_validation_imported
            && self.no_empirical_adequacy_imported
            && self.no_benchmark_success_imported
            && self.no_prediction_success_imported
            && self.no_falsification_success_imported
            && self.no_observed_catalog_recovery_imported
            && self.no_simulation_only_promotion_imported
            && self.no_fit_only_calibration_imported
            && self.no_unified_field_theory_claim_imported
            && self
                .claim_boundary
                .all_physical_and_success_claims_remain_false()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PFP002FiniteProtocolRecord {
    pub protocol_identifier: &'static str,
    pub prediction_target_label: &'static str,
    pub observable_label: &'static str,
    pub regime_label: &'static str,
    pub threshold_label: &'static str,
    pub rejection_condition_label: &'static str,
    pub audit_status_descriptor: &'static str,
    pub paper14_benchmark_compatibility_reference: &'static str,
    pub benchmark_compatibility_only_referenced: bool,
    pub auditable_interface_row: bool,
    pub claim_boundary: Paper15ClaimBoundary,
}

impl PFP002FiniteProtocolRecord {
    pub const fn canonical() -> Self {
        Self {
            protocol_identifier: "pfp002-protocol-record",
            prediction_target_label: "finite-target-label",
            observable_label: "finite-observable-label",
            regime_label: "finite-regime-label",
            threshold_label: "finite-threshold-label",
            rejection_condition_label: "finite-rejection-condition",
            audit_status_descriptor: "definition-only-audit-status",
            paper14_benchmark_compatibility_reference: PAPER14_FORMAL_ENDPOINT,
            benchmark_compatibility_only_referenced: true,
            auditable_interface_row: true,
            claim_boundary: Paper15ClaimBoundary::non_promoting(),
        }
    }

    pub fn protocol_labels(&self) -> [&'static str; 7] {
        [
            self.protocol_identifier,
            self.prediction_target_label,
            self.observable_label,
            self.regime_label,
            self.threshold_label,
            self.rejection_condition_label,
            self.audit_status_descriptor,
        ]
    }

    pub fn all_protocol_labels_are_bounded(&self) -> bool {
        self.protocol_labels()
            .into_iter()
            .all(is_bounded_protocol_label)
    }

    pub fn closes_pfp002(&self) -> bool {
        self.all_protocol_labels_are_bounded()
            && self.paper14_benchmark_compatibility_reference == PAPER14_FORMAL_ENDPOINT
            && self.benchmark_compatibility_only_referenced
            && self.auditable_interface_row
            && self
                .claim_boundary
                .all_physical_and_success_claims_remain_false()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PFP003PredictionTargetRegimeDescriptor {
    pub protocol_record: PFP002FiniteProtocolRecord,
    pub prediction_target_descriptor: &'static str,
    pub observable_descriptor: &'static str,
    pub regime_descriptor: &'static str,
    pub descriptors_linked_to_protocol_row: bool,
    pub descriptors_are_selection_criteria_only: bool,
    pub claim_boundary: Paper15ClaimBoundary,
}

impl PFP003PredictionTargetRegimeDescriptor {
    pub const fn canonical() -> Self {
        Self {
            protocol_record: PFP002FiniteProtocolRecord::canonical(),
            prediction_target_descriptor: "target-selection-descriptor",
            observable_descriptor: "observable-selection-descriptor",
            regime_descriptor: "regime-selection-descriptor",
            descriptors_linked_to_protocol_row: true,
            descriptors_are_selection_criteria_only: true,
            claim_boundary: Paper15ClaimBoundary::non_promoting(),
        }
    }

    pub fn descriptor_labels(&self) -> [&'static str; 3] {
        [
            self.prediction_target_descriptor,
            self.observable_descriptor,
            self.regime_descriptor,
        ]
    }

    pub fn all_descriptor_labels_are_bounded(&self) -> bool {
        self.descriptor_labels()
            .into_iter()
            .all(is_bounded_protocol_label)
    }

    pub fn closes_pfp003(&self) -> bool {
        self.protocol_record.closes_pfp002()
            && self.all_descriptor_labels_are_bounded()
            && self.descriptors_linked_to_protocol_row
            && self.descriptors_are_selection_criteria_only
            && !self.claim_boundary.observed_particle_catalog_recovery_claim
            && self
                .claim_boundary
                .all_physical_and_success_claims_remain_false()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PFP004FalsificationThresholdDescriptor {
    pub descriptor_record: PFP003PredictionTargetRegimeDescriptor,
    pub threshold_descriptor: &'static str,
    pub rejection_condition_descriptor: &'static str,
    pub threshold_linked_to_protocol_row: bool,
    pub rejection_condition_linked_to_protocol_row: bool,
    pub threshold_is_finite_criterion_only: bool,
    pub rejection_condition_predeclared: bool,
    pub claim_boundary: Paper15ClaimBoundary,
}

impl PFP004FalsificationThresholdDescriptor {
    pub const fn canonical() -> Self {
        Self {
            descriptor_record: PFP003PredictionTargetRegimeDescriptor::canonical(),
            threshold_descriptor: "finite-threshold-criterion",
            rejection_condition_descriptor: "predeclared-rejection-rule",
            threshold_linked_to_protocol_row: true,
            rejection_condition_linked_to_protocol_row: true,
            threshold_is_finite_criterion_only: true,
            rejection_condition_predeclared: true,
            claim_boundary: Paper15ClaimBoundary::non_promoting(),
        }
    }

    pub fn threshold_labels(&self) -> [&'static str; 2] {
        [
            self.threshold_descriptor,
            self.rejection_condition_descriptor,
        ]
    }

    pub fn all_threshold_labels_are_bounded(&self) -> bool {
        self.threshold_labels()
            .into_iter()
            .all(is_bounded_protocol_label)
    }

    pub fn closes_pfp004(&self) -> bool {
        self.descriptor_record.closes_pfp003()
            && self.all_threshold_labels_are_bounded()
            && self.threshold_linked_to_protocol_row
            && self.rejection_condition_linked_to_protocol_row
            && self.threshold_is_finite_criterion_only
            && self.rejection_condition_predeclared
            && !self.claim_boundary.falsification_success_claim
            && !self.claim_boundary.simulation_only_promotion
            && !self.claim_boundary.fit_only_calibration_claim
            && self
                .claim_boundary
                .all_physical_and_success_claims_remain_false()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PFP005Paper14BenchmarkCompatibility {
    pub threshold_record: PFP004FalsificationThresholdDescriptor,
    pub paper14_frozen_commit: &'static str,
    pub paper14_formal_endpoint: &'static str,
    pub paper14_final_certificate: &'static str,
    pub finite_compatibility_row: bool,
    pub compatibility_linked_to_threshold_record: bool,
    pub compatibility_is_schema_alignment_only: bool,
    pub claim_boundary: Paper15ClaimBoundary,
}

impl PFP005Paper14BenchmarkCompatibility {
    pub const fn canonical() -> Self {
        Self {
            threshold_record: PFP004FalsificationThresholdDescriptor::canonical(),
            paper14_frozen_commit: PAPER14_FROZEN_COMMIT,
            paper14_formal_endpoint: PAPER14_FORMAL_ENDPOINT,
            paper14_final_certificate: PAPER14_FINAL_CERTIFICATE,
            finite_compatibility_row: true,
            compatibility_linked_to_threshold_record: true,
            compatibility_is_schema_alignment_only: true,
            claim_boundary: Paper15ClaimBoundary::non_promoting(),
        }
    }

    pub fn references_frozen_paper14_certificate(&self) -> bool {
        self.paper14_frozen_commit == PAPER14_FROZEN_COMMIT
            && is_sha1_hex(self.paper14_frozen_commit)
            && self.paper14_formal_endpoint == PAPER14_FORMAL_ENDPOINT
            && self.paper14_final_certificate == PAPER14_FINAL_CERTIFICATE
    }

    pub fn closes_pfp005(&self) -> bool {
        self.threshold_record.closes_pfp004()
            && self.references_frozen_paper14_certificate()
            && self.finite_compatibility_row
            && self.compatibility_linked_to_threshold_record
            && self.compatibility_is_schema_alignment_only
            && !self.claim_boundary.protocol_recovery_claim
            && !self.claim_boundary.benchmark_success_claim
            && !self.claim_boundary.prediction_success_claim
            && !self.claim_boundary.falsification_success_claim
            && self
                .claim_boundary
                .all_physical_and_success_claims_remain_false()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Paper15SkeletonCertificate {
    pub pfp001_upstream_binding_closed: bool,
    pub pfp002_finite_protocol_record_closed: bool,
    pub pfp003_prediction_target_regime_closed: bool,
    pub pfp004_falsification_threshold_closed: bool,
    pub pfp005_paper14_benchmark_compatibility_closed: bool,
    pub pfp006_stability_reproducibility_closed: bool,
    pub pfp007_no_hidden_promotion_validation_success_audit_closed: bool,
    pub pfp008_final_conditional_certificate_closed: bool,
    pub claim_boundary: Paper15ClaimBoundary,
}

impl Paper15SkeletonCertificate {
    pub const fn initial_pfp001_only() -> Self {
        Self {
            pfp001_upstream_binding_closed: true,
            pfp002_finite_protocol_record_closed: false,
            pfp003_prediction_target_regime_closed: false,
            pfp004_falsification_threshold_closed: false,
            pfp005_paper14_benchmark_compatibility_closed: false,
            pfp006_stability_reproducibility_closed: false,
            pfp007_no_hidden_promotion_validation_success_audit_closed: false,
            pfp008_final_conditional_certificate_closed: false,
            claim_boundary: Paper15ClaimBoundary::non_promoting(),
        }
    }

    pub const fn pfp002_record_only() -> Self {
        Self {
            pfp001_upstream_binding_closed: true,
            pfp002_finite_protocol_record_closed: true,
            pfp003_prediction_target_regime_closed: false,
            pfp004_falsification_threshold_closed: false,
            pfp005_paper14_benchmark_compatibility_closed: false,
            pfp006_stability_reproducibility_closed: false,
            pfp007_no_hidden_promotion_validation_success_audit_closed: false,
            pfp008_final_conditional_certificate_closed: false,
            claim_boundary: Paper15ClaimBoundary::non_promoting(),
        }
    }

    pub const fn pfp003_descriptor_only() -> Self {
        Self {
            pfp001_upstream_binding_closed: true,
            pfp002_finite_protocol_record_closed: true,
            pfp003_prediction_target_regime_closed: true,
            pfp004_falsification_threshold_closed: false,
            pfp005_paper14_benchmark_compatibility_closed: false,
            pfp006_stability_reproducibility_closed: false,
            pfp007_no_hidden_promotion_validation_success_audit_closed: false,
            pfp008_final_conditional_certificate_closed: false,
            claim_boundary: Paper15ClaimBoundary::non_promoting(),
        }
    }

    pub const fn pfp004_threshold_only() -> Self {
        Self {
            pfp001_upstream_binding_closed: true,
            pfp002_finite_protocol_record_closed: true,
            pfp003_prediction_target_regime_closed: true,
            pfp004_falsification_threshold_closed: true,
            pfp005_paper14_benchmark_compatibility_closed: false,
            pfp006_stability_reproducibility_closed: false,
            pfp007_no_hidden_promotion_validation_success_audit_closed: false,
            pfp008_final_conditional_certificate_closed: false,
            claim_boundary: Paper15ClaimBoundary::non_promoting(),
        }
    }

    pub const fn pfp005_benchmark_compatibility_only() -> Self {
        Self {
            pfp001_upstream_binding_closed: true,
            pfp002_finite_protocol_record_closed: true,
            pfp003_prediction_target_regime_closed: true,
            pfp004_falsification_threshold_closed: true,
            pfp005_paper14_benchmark_compatibility_closed: true,
            pfp006_stability_reproducibility_closed: false,
            pfp007_no_hidden_promotion_validation_success_audit_closed: false,
            pfp008_final_conditional_certificate_closed: false,
            claim_boundary: Paper15ClaimBoundary::non_promoting(),
        }
    }

    pub fn closes_paper15_theorem(&self) -> bool {
        self.pfp001_upstream_binding_closed
            && self.pfp002_finite_protocol_record_closed
            && self.pfp003_prediction_target_regime_closed
            && self.pfp004_falsification_threshold_closed
            && self.pfp005_paper14_benchmark_compatibility_closed
            && self.pfp006_stability_reproducibility_closed
            && self.pfp007_no_hidden_promotion_validation_success_audit_closed
            && self.pfp008_final_conditional_certificate_closed
            && self
                .claim_boundary
                .all_physical_and_success_claims_remain_false()
    }
}

pub fn paper15_skeleton_marker() -> &'static str {
    PAPER15_SKELETON_MARKER
}

pub fn is_sha1_hex(value: &str) -> bool {
    value.len() == 40 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

pub fn is_bounded_protocol_label(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= FINITE_PROTOCOL_LABEL_MAX_BYTES
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b':' | b'.'))
}

pub fn active_obligation() -> &'static str {
    "PFP-006"
}
