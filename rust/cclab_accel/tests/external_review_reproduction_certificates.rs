use std::fs;
use std::path::{Path, PathBuf};

use cclab_accel::{
    active_obligation, is_sha256_descriptor, paper16_errc002_marker, paper16_errc003_marker,
    paper16_errc004_marker, paper16_errc005_marker, paper16_errc006_marker, paper16_errc007_marker,
    paper16_errc008_marker, paper16_skeleton_marker, ERRC001UpstreamBinding,
    ERRC002CertificateRecord, ERRC003ReviewerProtocolProvenance, ERRC004ArtifactEnvironmentHashes,
    ERRC005Paper15Compatibility, ERRC006StabilityAuditability, ERRC007NoHiddenClaimAudit,
    ERRC008FinalConditionalCertificate, Paper16SkeletonCertificate, PAPER15_FINAL_CERTIFICATE,
    PAPER15_FORMAL_ENDPOINT, PAPER15_FROZEN_COMMIT,
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
        "docs/external_review_reproduction_certificates_theorem.md",
        "docs/open_proof_obligations.md",
        "GPD/formal/FiniteCapacity/ExternalReviewReproductionCertificates.lean",
    ] {
        assert!(
            repo_root().join(path).exists(),
            "missing required file: {path}"
        );
    }
}

#[test]
fn errc001_consumes_closed_paper15_chain_without_success_claims() {
    let binding = ERRC001UpstreamBinding::canonical();
    assert!(binding.closes_errc001());
    assert_eq!(binding.paper15_frozen_commit, PAPER15_FROZEN_COMMIT);
    assert_eq!(binding.paper15_formal_endpoint, PAPER15_FORMAL_ENDPOINT);
    assert_eq!(binding.paper15_final_certificate, PAPER15_FINAL_CERTIFICATE);
    assert!(binding
        .claim_boundary
        .all_physical_review_and_success_claims_remain_false());
}

#[test]
fn initial_skeleton_keeps_paper16_theorem_open() {
    let skeleton = Paper16SkeletonCertificate::initial_errc001_only();
    assert!(skeleton.errc001_upstream_binding_closed);
    assert!(!skeleton.errc002_finite_certificate_record_closed);
    assert!(!skeleton.errc008_final_conditional_certificate_closed);
    assert!(!skeleton.closes_paper16_theorem());
    assert_eq!(
        paper16_skeleton_marker(),
        "paper16-external-review-reproduction-certificates-errc001-nonpromoting-skeleton"
    );
}

#[test]
fn errc002_defines_finite_nonpromoting_certificate_records() {
    let record = ERRC002CertificateRecord::canonical();
    assert!(record.closes_errc002());
    assert!(record.certificate_identifier.is_finite_bounded_label());
    assert!(record.reviewer_label.is_finite_bounded_label());
    assert!(record.protocol_label.is_finite_bounded_label());
    assert!(record.artifact_label.is_finite_bounded_label());
    assert!(record.environment_descriptor.is_finite_bounded_label());
    assert!(record.reproduction_status.avoids_success_claim());
    assert_eq!(
        record.paper15_protocol_reference.value,
        PAPER15_FORMAL_ENDPOINT
    );
    assert!(record.paper15_protocol_compatibility_referenced_only);
    assert!(record.no_review_acceptance_claim);
    assert!(record.no_reproduction_success_claim);
    assert!(record.no_physical_promotion_claim);
    assert!(record.no_physical_validation_claim);
    assert!(record.no_unified_field_theory_claim);
    assert_eq!(
        paper16_errc002_marker(),
        "paper16-external-review-reproduction-certificates-errc002-finite-records"
    );
}

#[test]
fn errc002_stage_keeps_later_rungs_open() {
    let record = ERRC002CertificateRecord::canonical();
    let certificate = Paper16SkeletonCertificate::from_errc002_record(&record);
    assert!(certificate.errc001_upstream_binding_closed);
    assert!(certificate.errc002_finite_certificate_record_closed);
    assert!(!certificate.errc003_reviewer_protocol_provenance_closed);
    assert!(!certificate.errc008_final_conditional_certificate_closed);
    assert!(!certificate.closes_paper16_theorem());
}

#[test]
fn errc003_defines_finite_reviewer_protocol_provenance_descriptors() {
    let descriptors = ERRC003ReviewerProtocolProvenance::canonical();
    assert!(descriptors.closes_errc003());
    assert!(descriptors.reviewer_label.is_finite_bounded_label());
    assert!(descriptors.reviewer_role.is_finite_bounded_label());
    assert!(descriptors.protocol_label.is_finite_bounded_label());
    assert!(descriptors.protocol_scope.is_finite_bounded_label());
    assert!(descriptors.provenance_source.is_finite_bounded_label());
    assert!(descriptors.provenance_timestamp.is_finite_bounded_label());
    assert!(descriptors.provenance_custodian.is_finite_bounded_label());
    assert!(descriptors.reviewer_descriptor_is_label_only);
    assert!(descriptors.protocol_descriptor_is_label_only);
    assert!(descriptors.provenance_descriptor_is_audit_only);
    assert!(descriptors.no_review_acceptance_claim);
    assert!(descriptors.no_reproduction_success_claim);
    assert!(descriptors.no_protocol_recovery_claim);
    assert!(descriptors.no_physical_validation_claim);
    assert!(descriptors.no_unified_field_theory_claim);
    assert_eq!(
        paper16_errc003_marker(),
        "paper16-external-review-reproduction-certificates-errc003-provenance-descriptors"
    );
}

#[test]
fn errc003_stage_keeps_later_rungs_open() {
    let record = ERRC002CertificateRecord::canonical();
    let descriptors = ERRC003ReviewerProtocolProvenance::canonical();
    let certificate = Paper16SkeletonCertificate::from_errc003_descriptors(&record, &descriptors);
    assert!(certificate.errc001_upstream_binding_closed);
    assert!(certificate.errc002_finite_certificate_record_closed);
    assert!(certificate.errc003_reviewer_protocol_provenance_closed);
    assert!(!certificate.errc004_reproduction_artifact_environment_closed);
    assert!(!certificate.errc008_final_conditional_certificate_closed);
    assert!(!certificate.closes_paper16_theorem());
}

#[test]
fn errc004_defines_finite_artifact_environment_hash_descriptors() {
    let artifacts = ERRC004ArtifactEnvironmentHashes::canonical();
    assert!(artifacts.closes_errc004());
    assert!(artifacts.artifact_label.is_finite_bounded_label());
    assert!(artifacts.environment_descriptor.is_finite_bounded_label());
    assert!(artifacts.artifact_hash.is_finite_bounded_label());
    assert!(artifacts.environment_hash.is_finite_bounded_label());
    assert!(is_sha256_descriptor(artifacts.artifact_hash.value));
    assert!(is_sha256_descriptor(artifacts.environment_hash.value));
    assert!(artifacts.reproduction_status.avoids_success_claim());
    assert!(artifacts.artifact_descriptor_is_audit_only);
    assert!(artifacts.hash_descriptors_are_integrity_only);
    assert!(artifacts.environment_descriptor_is_audit_only);
    assert!(artifacts.no_reproduction_success_claim);
    assert!(artifacts.no_physical_validation_claim);
    assert!(artifacts.no_empirical_adequacy_claim);
    assert!(artifacts.no_physical_promotion_claim);
    assert!(artifacts.no_unified_field_theory_claim);
    assert_eq!(
        paper16_errc004_marker(),
        "paper16-external-review-reproduction-certificates-errc004-artifact-environment-hashes"
    );
}

#[test]
fn errc004_stage_keeps_later_rungs_open() {
    let record = ERRC002CertificateRecord::canonical();
    let descriptors = ERRC003ReviewerProtocolProvenance::canonical();
    let artifacts = ERRC004ArtifactEnvironmentHashes::canonical();
    let certificate =
        Paper16SkeletonCertificate::from_errc004_artifacts(&record, &descriptors, &artifacts);
    assert!(certificate.errc001_upstream_binding_closed);
    assert!(certificate.errc002_finite_certificate_record_closed);
    assert!(certificate.errc003_reviewer_protocol_provenance_closed);
    assert!(certificate.errc004_reproduction_artifact_environment_closed);
    assert!(!certificate.errc005_paper15_protocol_compatibility_closed);
    assert!(!certificate.errc008_final_conditional_certificate_closed);
    assert!(!certificate.closes_paper16_theorem());
}

#[test]
fn errc005_defines_paper15_compatibility_without_success_imports() {
    let record = ERRC002CertificateRecord::canonical();
    let compatibility = ERRC005Paper15Compatibility::canonical();
    assert!(compatibility.closes_errc005(&record));
    assert_eq!(
        compatibility.paper15_endpoint_reference.value,
        PAPER15_FORMAL_ENDPOINT
    );
    assert_eq!(
        compatibility.paper15_final_certificate_reference.value,
        PAPER15_FINAL_CERTIFICATE
    );
    assert_eq!(
        compatibility.certificate_identifier.value,
        record.certificate_identifier.value
    );
    assert!(compatibility.paper15_endpoint_frozen);
    assert!(compatibility.paper15_final_certificate_frozen);
    assert!(compatibility.compatibility_is_reference_only);
    assert!(compatibility.no_certificate_recovery_claim);
    assert!(compatibility.no_protocol_recovery_claim);
    assert!(compatibility.no_review_acceptance_claim);
    assert!(compatibility.no_reproduction_success_claim);
    assert!(compatibility.no_prediction_success_claim);
    assert!(compatibility.no_falsification_success_claim);
    assert!(compatibility.no_physical_validation_claim);
    assert!(compatibility.no_physical_promotion_claim);
    assert_eq!(
        paper16_errc005_marker(),
        "paper16-external-review-reproduction-certificates-errc005-paper15-compatibility"
    );
}

#[test]
fn errc005_stage_keeps_later_rungs_open() {
    let record = ERRC002CertificateRecord::canonical();
    let descriptors = ERRC003ReviewerProtocolProvenance::canonical();
    let artifacts = ERRC004ArtifactEnvironmentHashes::canonical();
    let compatibility = ERRC005Paper15Compatibility::canonical();
    let certificate = Paper16SkeletonCertificate::from_errc005_paper15_compatibility(
        &record,
        &descriptors,
        &artifacts,
        &compatibility,
    );
    assert!(certificate.errc001_upstream_binding_closed);
    assert!(certificate.errc002_finite_certificate_record_closed);
    assert!(certificate.errc003_reviewer_protocol_provenance_closed);
    assert!(certificate.errc004_reproduction_artifact_environment_closed);
    assert!(certificate.errc005_paper15_protocol_compatibility_closed);
    assert!(!certificate.errc006_stability_auditability_closed);
    assert!(!certificate.errc008_final_conditional_certificate_closed);
    assert!(!certificate.closes_paper16_theorem());
}

#[test]
fn errc006_defines_stability_auditability_as_recheckability() {
    let record = ERRC002CertificateRecord::canonical();
    let descriptors = ERRC003ReviewerProtocolProvenance::canonical();
    let artifacts = ERRC004ArtifactEnvironmentHashes::canonical();
    let compatibility = ERRC005Paper15Compatibility::canonical();
    let stability = ERRC006StabilityAuditability::canonical();
    assert!(stability.closes_errc006(&record, &descriptors, &artifacts, &compatibility));
    assert!(stability.audit_snapshot_label.is_finite_bounded_label());
    assert!(stability.recheck_procedure_label.is_finite_bounded_label());
    assert!(stability.audit_status.is_finite_descriptor());
    assert!(stability.preserves_certificate_identifier);
    assert!(stability.preserves_reviewer_protocol_provenance);
    assert!(stability.preserves_artifact_environment_hashes);
    assert!(stability.preserves_paper15_compatibility_reference);
    assert!(stability.finite_recheckability_behavior);
    assert!(stability.audit_status_non_promoting);
    assert!(stability.reproducibility_is_recheckability_only);
    assert!(stability.no_reproduction_success_claim);
    assert!(stability.no_physical_validation_claim);
    assert!(stability.no_empirical_adequacy_claim);
    assert!(stability.no_physical_promotion_claim);
    assert_eq!(
        paper16_errc006_marker(),
        "paper16-external-review-reproduction-certificates-errc006-stability-auditability"
    );
}

#[test]
fn errc006_stage_keeps_later_rungs_open() {
    let record = ERRC002CertificateRecord::canonical();
    let descriptors = ERRC003ReviewerProtocolProvenance::canonical();
    let artifacts = ERRC004ArtifactEnvironmentHashes::canonical();
    let compatibility = ERRC005Paper15Compatibility::canonical();
    let stability = ERRC006StabilityAuditability::canonical();
    let certificate = Paper16SkeletonCertificate::from_errc006_stability(
        &record,
        &descriptors,
        &artifacts,
        &compatibility,
        &stability,
    );
    assert!(certificate.errc001_upstream_binding_closed);
    assert!(certificate.errc002_finite_certificate_record_closed);
    assert!(certificate.errc003_reviewer_protocol_provenance_closed);
    assert!(certificate.errc004_reproduction_artifact_environment_closed);
    assert!(certificate.errc005_paper15_protocol_compatibility_closed);
    assert!(certificate.errc006_stability_auditability_closed);
    assert!(!certificate.errc007_no_hidden_promotion_validation_acceptance_audit_closed);
    assert!(!certificate.errc008_final_conditional_certificate_closed);
    assert!(!certificate.closes_paper16_theorem());
}

#[test]
fn errc007_audits_all_hidden_claim_imports_fail_closed() {
    let record = ERRC002CertificateRecord::canonical();
    let descriptors = ERRC003ReviewerProtocolProvenance::canonical();
    let artifacts = ERRC004ArtifactEnvironmentHashes::canonical();
    let compatibility = ERRC005Paper15Compatibility::canonical();
    let stability = ERRC006StabilityAuditability::canonical();
    let audit = ERRC007NoHiddenClaimAudit::canonical();
    assert!(audit.closes_errc007(
        &record,
        &descriptors,
        &artifacts,
        &compatibility,
        &stability
    ));
    assert!(audit.audit_label.is_finite_bounded_label());
    assert!(audit.prior_rungs_remain_non_promoting);
    assert!(audit.rejects_certificate_recovery_import);
    assert!(audit.rejects_protocol_recovery_import);
    assert!(audit.rejects_review_acceptance_import);
    assert!(audit.rejects_reproduction_success_import);
    assert!(audit.rejects_benchmark_success_import);
    assert!(audit.rejects_prediction_success_import);
    assert!(audit.rejects_falsification_success_import);
    assert!(audit.rejects_physical_promotion_import);
    assert!(audit.rejects_physical_validation_import);
    assert!(audit.rejects_empirical_adequacy_import);
    assert!(audit.rejects_observed_catalog_recovery_import);
    assert!(audit.rejects_physical_standard_model_import);
    assert!(audit.rejects_physical_particle_excitation_import);
    assert!(audit.rejects_physical_matter_field_import);
    assert!(audit.rejects_physical_gauge_field_import);
    assert!(audit.rejects_physical_quantum_dynamics_import);
    assert!(audit.rejects_continuum_quantum_field_theory_import);
    assert!(audit.rejects_simulation_only_promotion_import);
    assert!(audit.rejects_fit_only_calibration_import);
    assert!(audit.rejects_physical_nature_import);
    assert!(audit.rejects_unified_field_theory_import);
    assert_eq!(
        paper16_errc007_marker(),
        "paper16-external-review-reproduction-certificates-errc007-no-hidden-claim-imports"
    );
}

#[test]
fn errc007_stage_keeps_final_certificate_open() {
    let record = ERRC002CertificateRecord::canonical();
    let descriptors = ERRC003ReviewerProtocolProvenance::canonical();
    let artifacts = ERRC004ArtifactEnvironmentHashes::canonical();
    let compatibility = ERRC005Paper15Compatibility::canonical();
    let stability = ERRC006StabilityAuditability::canonical();
    let audit = ERRC007NoHiddenClaimAudit::canonical();
    let certificate = Paper16SkeletonCertificate::from_errc007_no_hidden_claim_audit(
        &record,
        &descriptors,
        &artifacts,
        &compatibility,
        &stability,
        &audit,
    );
    assert!(certificate.errc001_upstream_binding_closed);
    assert!(certificate.errc002_finite_certificate_record_closed);
    assert!(certificate.errc003_reviewer_protocol_provenance_closed);
    assert!(certificate.errc004_reproduction_artifact_environment_closed);
    assert!(certificate.errc005_paper15_protocol_compatibility_closed);
    assert!(certificate.errc006_stability_auditability_closed);
    assert!(certificate.errc007_no_hidden_promotion_validation_acceptance_audit_closed);
    assert!(!certificate.errc008_final_conditional_certificate_closed);
    assert!(!certificate.closes_paper16_theorem());
}

#[test]
fn errc008_final_certificate_closes_paper16_theorem_conditionally() {
    let record = ERRC002CertificateRecord::canonical();
    let descriptors = ERRC003ReviewerProtocolProvenance::canonical();
    let artifacts = ERRC004ArtifactEnvironmentHashes::canonical();
    let compatibility = ERRC005Paper15Compatibility::canonical();
    let stability = ERRC006StabilityAuditability::canonical();
    let audit = ERRC007NoHiddenClaimAudit::canonical();
    let final_certificate = ERRC008FinalConditionalCertificate::canonical();
    assert!(final_certificate.closes_errc008(
        &record,
        &descriptors,
        &artifacts,
        &compatibility,
        &stability,
        &audit
    ));
    assert!(final_certificate
        .final_certificate_label
        .is_finite_bounded_label());
    assert!(final_certificate.final_certificate_assembled);
    assert!(final_certificate.no_certificate_recovery_claim);
    assert!(final_certificate.no_review_acceptance_claim);
    assert!(final_certificate.no_reproduction_success_claim);
    assert!(final_certificate.no_protocol_recovery_claim);
    assert!(final_certificate.no_prediction_success_claim);
    assert!(final_certificate.no_falsification_success_claim);
    assert!(final_certificate.no_physical_promotion_claim);
    assert!(final_certificate.no_physical_validation_claim);
    assert!(final_certificate.no_empirical_adequacy_claim);
    assert!(final_certificate.no_physical_nature_claim);
    assert!(final_certificate.no_unified_field_theory_claim);
    assert_eq!(
        paper16_errc008_marker(),
        "paper16-external-review-reproduction-certificates-errc008-final-conditional-certificate"
    );
}

#[test]
fn final_certificate_sets_all_rungs_closed_and_preserves_boundary() {
    let record = ERRC002CertificateRecord::canonical();
    let descriptors = ERRC003ReviewerProtocolProvenance::canonical();
    let artifacts = ERRC004ArtifactEnvironmentHashes::canonical();
    let compatibility = ERRC005Paper15Compatibility::canonical();
    let stability = ERRC006StabilityAuditability::canonical();
    let audit = ERRC007NoHiddenClaimAudit::canonical();
    let final_certificate = ERRC008FinalConditionalCertificate::canonical();
    let certificate = Paper16SkeletonCertificate::from_errc008_final_certificate(
        &record,
        &descriptors,
        &artifacts,
        &compatibility,
        &stability,
        &audit,
        &final_certificate,
    );
    assert!(certificate.errc001_upstream_binding_closed);
    assert!(certificate.errc002_finite_certificate_record_closed);
    assert!(certificate.errc003_reviewer_protocol_provenance_closed);
    assert!(certificate.errc004_reproduction_artifact_environment_closed);
    assert!(certificate.errc005_paper15_protocol_compatibility_closed);
    assert!(certificate.errc006_stability_auditability_closed);
    assert!(certificate.errc007_no_hidden_promotion_validation_acceptance_audit_closed);
    assert!(certificate.errc008_final_conditional_certificate_closed);
    assert!(certificate
        .claim_boundary
        .all_physical_review_and_success_claims_remain_false());
    assert!(certificate.closes_paper16_theorem());
}

#[test]
fn upstream_json_records_paper15_certificate_and_nonpromotion() {
    let upstream = read_repo_file("UPSTREAM-PAPERS.json");
    assert!(upstream.contains(PAPER15_FROZEN_COMMIT));
    assert!(upstream.contains(PAPER15_FORMAL_ENDPOINT));
    assert!(upstream.contains(PAPER15_FINAL_CERTIFICATE));
    assert!(upstream.contains(
        "\"paper15_conditional_prediction_falsification_protocols_theorem_closed\": true"
    ));
    assert!(upstream.contains("\"paper15_prediction_success_claim\": false"));
    assert!(upstream.contains("\"paper15_falsification_success_claim\": false"));
    assert!(upstream.contains("\"unified_field_theory_claim\": false"));
}

#[test]
fn docs_record_theorem_closed_and_success_claims_false() {
    let state = read_repo_file("GPD/state.json");
    let state_md = read_repo_file("GPD/STATE.md");
    let theorem = read_repo_file("docs/external_review_reproduction_certificates_theorem.md");

    assert_eq!(active_obligation(), "NONE");
    assert!(state.contains("\"active_obligation\": \"NONE\""));
    assert!(state.contains("\"errc002_finite_certificate_record_closed\": true"));
    assert!(state.contains("\"errc003_reviewer_protocol_provenance_closed\": true"));
    assert!(state.contains("\"errc004_reproduction_artifact_environment_closed\": true"));
    assert!(state.contains("\"errc005_paper15_protocol_compatibility_closed\": true"));
    assert!(state.contains("\"errc006_stability_auditability_closed\": true"));
    assert!(
        state.contains("\"errc007_no_hidden_promotion_validation_acceptance_audit_closed\": true")
    );
    assert!(state.contains("\"errc008_final_conditional_certificate_closed\": true"));
    assert!(state.contains("\"external_review_reproduction_certificates_theorem_closed\": true"));
    assert!(state.contains("\"certificate_recovery_claim\": false"));
    assert!(state.contains("\"review_acceptance_claim\": false"));
    assert!(state.contains("\"reproduction_success_claim\": false"));
    assert!(state.contains("\"prediction_success_claim\": false"));
    assert!(state.contains("\"falsification_success_claim\": false"));
    assert!(state.contains("\"physical_promotion_claim\": false"));
    assert!(state.contains("\"physical_validation_claim\": false"));
    assert!(state_md
        .contains("The local Paper 16 external review and reproduction certificates theorem is"));
    assert!(theorem.contains("ERRC-002"));
    assert!(theorem.contains("ERRC-008"));
    assert!(theorem.contains("The theorem is closed conditionally"));
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
