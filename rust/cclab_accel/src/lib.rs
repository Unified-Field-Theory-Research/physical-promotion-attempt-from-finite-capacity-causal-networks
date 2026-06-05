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
pub const PAPER15_FROZEN_COMMIT: &str = "b0fbcea9d936393586367e1ea9fb2586b4b4f7e3";
pub const PAPER16_FROZEN_COMMIT: &str = "8bb266493b1c53c776b2d39a47140794edc8935f";

pub const PAPER16_FORMAL_ENDPOINT: &str =
    "Paper16ExternalReviewReproductionCertificatesTheoremContract.closed";
pub const PAPER16_FINAL_CERTIFICATE: &str =
    "paper16_errc008_final_conditional_certificate_closes_external_review_reproduction_certificates_theorem";

pub const PAPER17_SKELETON_MARKER: &str =
    "paper17-physical-promotion-attempt-ppa001-nonpromoting-skeleton";

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

pub const UPSTREAM_CHAIN: [UpstreamPaper; 16] = [
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
    UpstreamPaper::new(15, PAPER15_FROZEN_COMMIT, true, false, false),
    UpstreamPaper::new(16, PAPER16_FROZEN_COMMIT, true, false, false),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Paper17ClaimBoundary {
    pub physical_promotion_attempt_success_claim: bool,
    pub physical_promotion_claim: bool,
    pub physical_validation_claim: bool,
    pub empirical_adequacy_claim: bool,
    pub certificate_recovery_claim: bool,
    pub review_acceptance_claim: bool,
    pub reproduction_success_claim: bool,
    pub protocol_recovery_claim: bool,
    pub benchmark_success_claim: bool,
    pub prediction_success_claim: bool,
    pub falsification_success_claim: bool,
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

impl Paper17ClaimBoundary {
    pub const fn non_promoting() -> Self {
        Self {
            physical_promotion_attempt_success_claim: false,
            physical_promotion_claim: false,
            physical_validation_claim: false,
            empirical_adequacy_claim: false,
            certificate_recovery_claim: false,
            review_acceptance_claim: false,
            reproduction_success_claim: false,
            protocol_recovery_claim: false,
            benchmark_success_claim: false,
            prediction_success_claim: false,
            falsification_success_claim: false,
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

    pub fn all_physical_promotion_and_success_claims_remain_false(&self) -> bool {
        !self.physical_promotion_attempt_success_claim
            && !self.physical_promotion_claim
            && !self.physical_validation_claim
            && !self.empirical_adequacy_claim
            && !self.certificate_recovery_claim
            && !self.review_acceptance_claim
            && !self.reproduction_success_claim
            && !self.protocol_recovery_claim
            && !self.benchmark_success_claim
            && !self.prediction_success_claim
            && !self.falsification_success_claim
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
pub struct PPA001UpstreamBinding {
    pub upstream_chain: &'static [UpstreamPaper],
    pub paper16_frozen_commit: &'static str,
    pub paper16_formal_endpoint: &'static str,
    pub paper16_final_certificate: &'static str,
    pub finite_capacity_boundary_preserved: bool,
    pub locality_boundary_preserved: bool,
    pub bounded_transfer_boundary_preserved: bool,
    pub no_physical_promotion_imported: bool,
    pub no_physical_validation_imported: bool,
    pub no_empirical_adequacy_imported: bool,
    pub no_review_acceptance_imported: bool,
    pub no_reproduction_success_imported: bool,
    pub no_benchmark_success_imported: bool,
    pub no_prediction_success_imported: bool,
    pub no_falsification_success_imported: bool,
    pub no_physical_nature_claim_imported: bool,
    pub no_unified_field_theory_claim_imported: bool,
    pub claim_boundary: Paper17ClaimBoundary,
}

impl PPA001UpstreamBinding {
    pub const fn canonical() -> Self {
        Self {
            upstream_chain: &UPSTREAM_CHAIN,
            paper16_frozen_commit: PAPER16_FROZEN_COMMIT,
            paper16_formal_endpoint: PAPER16_FORMAL_ENDPOINT,
            paper16_final_certificate: PAPER16_FINAL_CERTIFICATE,
            finite_capacity_boundary_preserved: true,
            locality_boundary_preserved: true,
            bounded_transfer_boundary_preserved: true,
            no_physical_promotion_imported: true,
            no_physical_validation_imported: true,
            no_empirical_adequacy_imported: true,
            no_review_acceptance_imported: true,
            no_reproduction_success_imported: true,
            no_benchmark_success_imported: true,
            no_prediction_success_imported: true,
            no_falsification_success_imported: true,
            no_physical_nature_claim_imported: true,
            no_unified_field_theory_claim_imported: true,
            claim_boundary: Paper17ClaimBoundary::non_promoting(),
        }
    }

    pub fn closes_ppa001(&self) -> bool {
        self.upstream_chain.len() == 16
            && self.upstream_chain.iter().enumerate().all(|(idx, paper)| {
                paper.paper as usize == idx + 1
                    && is_sha1_hex(paper.frozen_commit)
                    && paper.closes_internal_conditional_without_physical_claim()
            })
            && self.paper16_frozen_commit == PAPER16_FROZEN_COMMIT
            && self.paper16_formal_endpoint == PAPER16_FORMAL_ENDPOINT
            && self.paper16_final_certificate == PAPER16_FINAL_CERTIFICATE
            && self.finite_capacity_boundary_preserved
            && self.locality_boundary_preserved
            && self.bounded_transfer_boundary_preserved
            && self.no_physical_promotion_imported
            && self.no_physical_validation_imported
            && self.no_empirical_adequacy_imported
            && self.no_review_acceptance_imported
            && self.no_reproduction_success_imported
            && self.no_benchmark_success_imported
            && self.no_prediction_success_imported
            && self.no_falsification_success_imported
            && self.no_physical_nature_claim_imported
            && self.no_unified_field_theory_claim_imported
            && self
                .claim_boundary
                .all_physical_promotion_and_success_claims_remain_false()
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Paper17SkeletonCertificate {
    pub ppa001_upstream_binding_closed: bool,
    pub ppa002_finite_promotion_attempt_record_closed: bool,
    pub ppa003_eligibility_evidence_review_closed: bool,
    pub ppa004_decision_objection_risk_closed: bool,
    pub ppa005_paper16_certificate_compatibility_closed: bool,
    pub ppa006_stability_audit_rollback_closed: bool,
    pub ppa007_no_hidden_promotion_validation_nature_audit_closed: bool,
    pub ppa008_final_conditional_certificate_closed: bool,
    pub claim_boundary: Paper17ClaimBoundary,
}

impl Paper17SkeletonCertificate {
    pub const fn initial_ppa001_only() -> Self {
        Self {
            ppa001_upstream_binding_closed: true,
            ppa002_finite_promotion_attempt_record_closed: false,
            ppa003_eligibility_evidence_review_closed: false,
            ppa004_decision_objection_risk_closed: false,
            ppa005_paper16_certificate_compatibility_closed: false,
            ppa006_stability_audit_rollback_closed: false,
            ppa007_no_hidden_promotion_validation_nature_audit_closed: false,
            ppa008_final_conditional_certificate_closed: false,
            claim_boundary: Paper17ClaimBoundary::non_promoting(),
        }
    }

    pub fn closes_paper17_theorem(&self) -> bool {
        self.ppa001_upstream_binding_closed
            && self.ppa002_finite_promotion_attempt_record_closed
            && self.ppa003_eligibility_evidence_review_closed
            && self.ppa004_decision_objection_risk_closed
            && self.ppa005_paper16_certificate_compatibility_closed
            && self.ppa006_stability_audit_rollback_closed
            && self.ppa007_no_hidden_promotion_validation_nature_audit_closed
            && self.ppa008_final_conditional_certificate_closed
            && self
                .claim_boundary
                .all_physical_promotion_and_success_claims_remain_false()
    }
}

pub fn paper17_skeleton_marker() -> &'static str {
    PAPER17_SKELETON_MARKER
}

pub fn is_sha1_hex(value: &str) -> bool {
    value.len() == 40 && value.bytes().all(|byte| byte.is_ascii_hexdigit())
}

pub fn active_obligation() -> &'static str {
    "PPA-002"
}
