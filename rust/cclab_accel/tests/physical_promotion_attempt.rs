use std::fs;
use std::path::{Path, PathBuf};

use cclab_accel::{
    active_obligation, bounded_descriptor, paper17_ppa002_marker, paper17_ppa003_marker,
    paper17_ppa004_marker, paper17_ppa005_marker, paper17_ppa006_marker, paper17_ppa007_marker,
    paper17_ppa008_marker, paper17_skeleton_marker, PPA001UpstreamBinding,
    PPA002FinitePromotionAttemptRecord, PPA003EligibilityEvidenceReviewDescriptors,
    PPA004DecisionObjectionRiskDescriptors, PPA005Paper16CertificateCompatibility,
    PPA006StabilityAuditRollback, PPA007NoHiddenImportAudit, PPA008FinalConditionalCertificate,
    Paper17SkeletonCertificate, PAPER16_FINAL_CERTIFICATE, PAPER16_FORMAL_ENDPOINT,
    PAPER16_FROZEN_COMMIT,
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
        "docs/physical_promotion_attempt_theorem.md",
        "docs/open_proof_obligations.md",
        "GPD/formal/FiniteCapacity/PhysicalPromotionAttempt.lean",
    ] {
        assert!(
            repo_root().join(path).exists(),
            "missing required file: {path}"
        );
    }
}

#[test]
fn ppa001_consumes_closed_paper16_chain_without_promotion_claims() {
    let binding = PPA001UpstreamBinding::canonical();
    assert!(binding.closes_ppa001());
    assert_eq!(binding.paper16_frozen_commit, PAPER16_FROZEN_COMMIT);
    assert_eq!(binding.paper16_formal_endpoint, PAPER16_FORMAL_ENDPOINT);
    assert_eq!(binding.paper16_final_certificate, PAPER16_FINAL_CERTIFICATE);
    assert!(binding
        .claim_boundary
        .all_physical_promotion_and_success_claims_remain_false());
}

#[test]
fn initial_skeleton_keeps_paper17_theorem_open() {
    let skeleton = Paper17SkeletonCertificate::initial_ppa001_only();
    assert!(skeleton.ppa001_upstream_binding_closed);
    assert!(!skeleton.ppa002_finite_promotion_attempt_record_closed);
    assert!(!skeleton.ppa008_final_conditional_certificate_closed);
    assert!(!skeleton.closes_paper17_theorem());
    assert_eq!(
        paper17_skeleton_marker(),
        "paper17-physical-promotion-attempt-ppa001-nonpromoting-skeleton"
    );
}

#[test]
fn ppa002_defines_bounded_nonpromoting_attempt_record() {
    let record = PPA002FinitePromotionAttemptRecord::canonical();
    assert!(record.closes_ppa002());
    assert!(bounded_descriptor(record.attempt_identifier));
    assert!(bounded_descriptor(record.eligibility_label));
    assert!(bounded_descriptor(record.evidence_bundle_label));
    assert!(bounded_descriptor(record.review_certificate_reference));
    assert!(bounded_descriptor(record.decision_label));
    assert!(bounded_descriptor(record.objection_label));
    assert!(bounded_descriptor(record.residual_risk_label));
    assert!(bounded_descriptor(record.audit_status_descriptor));
    assert_eq!(
        record.review_certificate_reference,
        PAPER16_FINAL_CERTIFICATE
    );
    assert!(record.paper16_certificate_referenced_only);
    assert!(record
        .claim_boundary
        .all_physical_promotion_and_success_claims_remain_false());
    assert_eq!(
        paper17_ppa002_marker(),
        "paper17-physical-promotion-attempt-ppa002-finite-record"
    );
}

#[test]
fn ppa002_skeleton_closes_record_rung_but_not_theorem() {
    let skeleton = Paper17SkeletonCertificate::ppa002_record_closed();
    assert!(skeleton.ppa001_upstream_binding_closed);
    assert!(skeleton.ppa002_finite_promotion_attempt_record_closed);
    assert!(!skeleton.ppa003_eligibility_evidence_review_closed);
    assert!(!skeleton.ppa008_final_conditional_certificate_closed);
    assert!(!skeleton.closes_paper17_theorem());
}

#[test]
fn ppa003_defines_descriptor_references_without_success_claims() {
    let attempt_record = PPA002FinitePromotionAttemptRecord::canonical();
    let descriptors = PPA003EligibilityEvidenceReviewDescriptors::canonical();
    assert!(descriptors.closes_ppa003(&attempt_record));
    assert!(bounded_descriptor(descriptors.eligibility_descriptor));
    assert!(bounded_descriptor(descriptors.evidence_bundle_descriptor));
    assert!(bounded_descriptor(descriptors.review_reference_descriptor));
    assert!(bounded_descriptor(
        descriptors.paper16_certificate_reference
    ));
    assert!(descriptors.eligibility_descriptor_is_not_promotion_finding);
    assert!(descriptors.evidence_descriptor_is_not_sufficiency_claim);
    assert!(descriptors.review_reference_is_not_acceptance_claim);
    assert_eq!(
        descriptors.paper16_certificate_reference,
        PAPER16_FINAL_CERTIFICATE
    );
    assert!(descriptors
        .claim_boundary
        .all_physical_promotion_and_success_claims_remain_false());
    assert_eq!(
        paper17_ppa003_marker(),
        "paper17-physical-promotion-attempt-ppa003-descriptors"
    );
}

#[test]
fn ppa003_skeleton_closes_descriptor_rung_but_not_theorem() {
    let skeleton = Paper17SkeletonCertificate::ppa003_descriptors_closed();
    assert!(skeleton.ppa001_upstream_binding_closed);
    assert!(skeleton.ppa002_finite_promotion_attempt_record_closed);
    assert!(skeleton.ppa003_eligibility_evidence_review_closed);
    assert!(!skeleton.ppa004_decision_objection_risk_closed);
    assert!(!skeleton.ppa008_final_conditional_certificate_closed);
    assert!(!skeleton.closes_paper17_theorem());
}

#[test]
fn ppa004_defines_decision_objection_risk_without_resolution_claims() {
    let attempt_record = PPA002FinitePromotionAttemptRecord::canonical();
    let descriptors = PPA003EligibilityEvidenceReviewDescriptors::canonical();
    let decision_row = PPA004DecisionObjectionRiskDescriptors::canonical();
    assert!(decision_row.closes_ppa004(&attempt_record, &descriptors));
    assert!(bounded_descriptor(decision_row.decision_descriptor));
    assert!(bounded_descriptor(decision_row.objection_descriptor));
    assert!(bounded_descriptor(decision_row.residual_risk_descriptor));
    assert!(decision_row.decision_descriptor_is_not_promotion_decision);
    assert!(decision_row.objection_descriptor_is_not_review_resolution);
    assert!(decision_row.residual_risk_descriptor_is_not_risk_discharge);
    assert!(decision_row
        .claim_boundary
        .all_physical_promotion_and_success_claims_remain_false());
    assert_eq!(
        paper17_ppa004_marker(),
        "paper17-physical-promotion-attempt-ppa004-decision-objection-risk"
    );
}

#[test]
fn ppa004_skeleton_closes_decision_rung_but_not_theorem() {
    let skeleton = Paper17SkeletonCertificate::ppa004_decision_objection_risk_closed();
    assert!(skeleton.ppa001_upstream_binding_closed);
    assert!(skeleton.ppa002_finite_promotion_attempt_record_closed);
    assert!(skeleton.ppa003_eligibility_evidence_review_closed);
    assert!(skeleton.ppa004_decision_objection_risk_closed);
    assert!(!skeleton.ppa005_paper16_certificate_compatibility_closed);
    assert!(!skeleton.ppa008_final_conditional_certificate_closed);
    assert!(!skeleton.closes_paper17_theorem());
}

#[test]
fn ppa005_references_paper16_certificates_without_acceptance_or_success() {
    let attempt_record = PPA002FinitePromotionAttemptRecord::canonical();
    let descriptors = PPA003EligibilityEvidenceReviewDescriptors::canonical();
    let decision_row = PPA004DecisionObjectionRiskDescriptors::canonical();
    let compatibility = PPA005Paper16CertificateCompatibility::canonical();
    assert!(compatibility.closes_ppa005(&attempt_record, &descriptors, &decision_row));
    assert_eq!(
        compatibility.paper16_endpoint_reference,
        PAPER16_FORMAL_ENDPOINT
    );
    assert_eq!(
        compatibility.paper16_final_certificate_reference,
        PAPER16_FINAL_CERTIFICATE
    );
    assert!(bounded_descriptor(
        compatibility.review_certificate_compatibility_descriptor
    ));
    assert!(bounded_descriptor(
        compatibility.reproduction_certificate_compatibility_descriptor
    ));
    assert!(compatibility.review_certificate_reference_is_not_acceptance);
    assert!(compatibility.reproduction_certificate_reference_is_not_success);
    assert!(compatibility.validation_compatibility_is_not_validation_success);
    assert!(compatibility.benchmark_compatibility_is_not_benchmark_success);
    assert!(compatibility.prediction_compatibility_is_not_prediction_success);
    assert!(compatibility.falsification_compatibility_is_not_falsification_success);
    assert!(compatibility.evidence_intake_compatibility_is_not_promotion);
    assert!(compatibility
        .claim_boundary
        .all_physical_promotion_and_success_claims_remain_false());
    assert_eq!(
        paper17_ppa005_marker(),
        "paper17-physical-promotion-attempt-ppa005-paper16-compatibility"
    );
}

#[test]
fn ppa005_skeleton_closes_paper16_compatibility_but_not_theorem() {
    let skeleton = Paper17SkeletonCertificate::ppa005_paper16_compatibility_closed();
    assert!(skeleton.ppa001_upstream_binding_closed);
    assert!(skeleton.ppa002_finite_promotion_attempt_record_closed);
    assert!(skeleton.ppa003_eligibility_evidence_review_closed);
    assert!(skeleton.ppa004_decision_objection_risk_closed);
    assert!(skeleton.ppa005_paper16_certificate_compatibility_closed);
    assert!(!skeleton.ppa006_stability_audit_rollback_closed);
    assert!(!skeleton.ppa008_final_conditional_certificate_closed);
    assert!(!skeleton.closes_paper17_theorem());
}

#[test]
fn ppa006_defines_stability_audit_rollback_without_failure_claims() {
    let attempt_record = PPA002FinitePromotionAttemptRecord::canonical();
    let descriptors = PPA003EligibilityEvidenceReviewDescriptors::canonical();
    let decision_row = PPA004DecisionObjectionRiskDescriptors::canonical();
    let compatibility = PPA005Paper16CertificateCompatibility::canonical();
    let rollback = PPA006StabilityAuditRollback::canonical();
    assert!(rollback.closes_ppa006(&attempt_record, &descriptors, &decision_row, &compatibility));
    assert!(bounded_descriptor(rollback.stability_descriptor));
    assert!(bounded_descriptor(rollback.audit_descriptor));
    assert!(bounded_descriptor(rollback.rollback_descriptor));
    assert!(rollback.interface_stability_is_not_empirical_stability);
    assert!(rollback.audit_replayable_from_finite_rows);
    assert!(rollback.rollback_local_to_attempt_interface);
    assert!(rollback.rollback_is_not_falsification_success);
    assert!(rollback.rollback_is_not_review_failure);
    assert!(rollback.rollback_is_not_physical_promotion_failure);
    assert!(rollback.rollback_is_not_validation_failure);
    assert!(rollback
        .claim_boundary
        .all_physical_promotion_and_success_claims_remain_false());
    assert_eq!(
        paper17_ppa006_marker(),
        "paper17-physical-promotion-attempt-ppa006-stability-audit-rollback"
    );
}

#[test]
fn ppa006_skeleton_closes_stability_rung_but_not_theorem() {
    let skeleton = Paper17SkeletonCertificate::ppa006_stability_audit_rollback_closed();
    assert!(skeleton.ppa001_upstream_binding_closed);
    assert!(skeleton.ppa002_finite_promotion_attempt_record_closed);
    assert!(skeleton.ppa003_eligibility_evidence_review_closed);
    assert!(skeleton.ppa004_decision_objection_risk_closed);
    assert!(skeleton.ppa005_paper16_certificate_compatibility_closed);
    assert!(skeleton.ppa006_stability_audit_rollback_closed);
    assert!(!skeleton.ppa007_no_hidden_promotion_validation_nature_audit_closed);
    assert!(!skeleton.ppa008_final_conditional_certificate_closed);
    assert!(!skeleton.closes_paper17_theorem());
}

#[test]
fn ppa007_rejects_hidden_promotion_validation_nature_and_uft_imports() {
    let attempt_record = PPA002FinitePromotionAttemptRecord::canonical();
    let descriptors = PPA003EligibilityEvidenceReviewDescriptors::canonical();
    let decision_row = PPA004DecisionObjectionRiskDescriptors::canonical();
    let compatibility = PPA005Paper16CertificateCompatibility::canonical();
    let rollback = PPA006StabilityAuditRollback::canonical();
    let audit = PPA007NoHiddenImportAudit::canonical();
    assert!(audit.closes_ppa007(
        &attempt_record,
        &descriptors,
        &decision_row,
        &compatibility,
        &rollback
    ));
    assert!(bounded_descriptor(audit.hidden_import_audit_descriptor));
    assert!(audit.rejects_hidden_promotion_attempt_success);
    assert!(audit.rejects_hidden_physical_promotion);
    assert!(audit.rejects_hidden_physical_validation);
    assert!(audit.rejects_hidden_empirical_adequacy);
    assert!(audit.rejects_hidden_review_acceptance);
    assert!(audit.rejects_hidden_reproduction_success);
    assert!(audit.rejects_hidden_benchmark_success);
    assert!(audit.rejects_hidden_prediction_success);
    assert!(audit.rejects_hidden_falsification_success);
    assert!(audit.rejects_simulation_only_promotion);
    assert!(audit.rejects_fit_only_calibration);
    assert!(audit.rejects_physical_nature_import);
    assert!(audit.rejects_unified_field_theory_import);
    assert!(audit.fail_closed_audit_row);
    assert!(audit.auditable_audit_row);
    assert!(audit
        .claim_boundary
        .all_physical_promotion_and_success_claims_remain_false());
    assert_eq!(
        paper17_ppa007_marker(),
        "paper17-physical-promotion-attempt-ppa007-no-hidden-imports"
    );
}

#[test]
fn ppa007_skeleton_closes_hidden_import_audit_but_not_theorem() {
    let skeleton = Paper17SkeletonCertificate::ppa007_no_hidden_import_audit_closed();
    assert!(skeleton.ppa001_upstream_binding_closed);
    assert!(skeleton.ppa002_finite_promotion_attempt_record_closed);
    assert!(skeleton.ppa003_eligibility_evidence_review_closed);
    assert!(skeleton.ppa004_decision_objection_risk_closed);
    assert!(skeleton.ppa005_paper16_certificate_compatibility_closed);
    assert!(skeleton.ppa006_stability_audit_rollback_closed);
    assert!(skeleton.ppa007_no_hidden_promotion_validation_nature_audit_closed);
    assert!(!skeleton.ppa008_final_conditional_certificate_closed);
    assert!(!skeleton.closes_paper17_theorem());
}

#[test]
fn ppa008_final_certificate_closes_conditional_theorem_only() {
    let attempt_record = PPA002FinitePromotionAttemptRecord::canonical();
    let descriptors = PPA003EligibilityEvidenceReviewDescriptors::canonical();
    let decision_row = PPA004DecisionObjectionRiskDescriptors::canonical();
    let compatibility = PPA005Paper16CertificateCompatibility::canonical();
    let rollback = PPA006StabilityAuditRollback::canonical();
    let audit = PPA007NoHiddenImportAudit::canonical();
    let final_certificate = PPA008FinalConditionalCertificate::canonical();
    assert!(final_certificate.closes_ppa008(
        &attempt_record,
        &descriptors,
        &decision_row,
        &compatibility,
        &rollback,
        &audit
    ));
    assert!(bounded_descriptor(
        final_certificate.final_certificate_descriptor
    ));
    assert!(final_certificate.conditional_interface_theorem_only);
    assert!(final_certificate.preserves_finite_capacity);
    assert!(final_certificate.preserves_locality);
    assert!(final_certificate.preserves_bounded_transfer);
    assert!(final_certificate.preserves_no_hidden_import_audit);
    assert!(final_certificate
        .claim_boundary
        .all_physical_promotion_and_success_claims_remain_false());
    assert_eq!(
        paper17_ppa008_marker(),
        "paper17-physical-promotion-attempt-ppa008-final-conditional-certificate"
    );
}

#[test]
fn ppa008_skeleton_closes_paper17_theorem_contract() {
    let skeleton = Paper17SkeletonCertificate::final_conditional_certificate_closed();
    assert!(skeleton.ppa001_upstream_binding_closed);
    assert!(skeleton.ppa002_finite_promotion_attempt_record_closed);
    assert!(skeleton.ppa003_eligibility_evidence_review_closed);
    assert!(skeleton.ppa004_decision_objection_risk_closed);
    assert!(skeleton.ppa005_paper16_certificate_compatibility_closed);
    assert!(skeleton.ppa006_stability_audit_rollback_closed);
    assert!(skeleton.ppa007_no_hidden_promotion_validation_nature_audit_closed);
    assert!(skeleton.ppa008_final_conditional_certificate_closed);
    assert!(skeleton.closes_paper17_theorem());
    assert!(skeleton
        .claim_boundary
        .all_physical_promotion_and_success_claims_remain_false());
}

#[test]
fn upstream_json_records_paper16_certificate_and_nonpromotion() {
    let upstream = read_repo_file("UPSTREAM-PAPERS.json");
    assert!(upstream.contains(PAPER16_FROZEN_COMMIT));
    assert!(upstream.contains(PAPER16_FORMAL_ENDPOINT));
    assert!(upstream.contains(PAPER16_FINAL_CERTIFICATE));
    assert!(upstream.contains(
        "\"paper16_conditional_external_review_reproduction_certificates_theorem_closed\": true"
    ));
    assert!(upstream.contains("\"paper16_review_acceptance_claim\": false"));
    assert!(upstream.contains("\"paper16_reproduction_success_claim\": false"));
    assert!(upstream.contains("\"unified_field_theory_claim\": false"));
}

#[test]
fn docs_mark_theorem_complete_and_promotion_claims_false() {
    let state = read_repo_file("GPD/state.json");
    let state_md = read_repo_file("GPD/STATE.md");
    let normalized_state_md = state_md.split_whitespace().collect::<Vec<_>>().join(" ");
    let theorem = read_repo_file("docs/physical_promotion_attempt_theorem.md");

    assert_eq!(active_obligation(), "COMPLETE");
    assert!(state.contains("\"active_obligation\": \"COMPLETE\""));
    assert!(state.contains("\"ppa002_finite_promotion_attempt_record_closed\": true"));
    assert!(state.contains("\"ppa003_eligibility_evidence_review_closed\": true"));
    assert!(state.contains("\"ppa004_decision_objection_risk_closed\": true"));
    assert!(state.contains("\"ppa005_paper16_certificate_compatibility_closed\": true"));
    assert!(state.contains("\"ppa006_stability_audit_rollback_closed\": true"));
    assert!(state.contains("\"ppa007_no_hidden_promotion_validation_nature_audit_closed\": true"));
    assert!(state.contains("\"ppa008_final_conditional_certificate_closed\": true"));
    assert!(state.contains("\"physical_promotion_attempt_theorem_closed\": true"));
    assert!(state.contains("\"physical_promotion_attempt_success_claim\": false"));
    assert!(state.contains("\"physical_promotion_claim\": false"));
    assert!(state.contains("\"physical_validation_claim\": false"));
    assert!(state.contains("\"empirical_adequacy_claim\": false"));
    assert!(state.contains("\"physical_nature_claim\": false"));
    assert!(normalized_state_md.contains("theorem is closed"));
    assert!(theorem.contains("The theorem is closed"));
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
