use std::fs;
use std::path::{Path, PathBuf};

use cclab_accel::{
    active_obligation, paper16_errc002_marker, paper16_errc003_marker, paper16_skeleton_marker,
    ERRC001UpstreamBinding, ERRC002CertificateRecord, ERRC003ReviewerProtocolProvenance,
    Paper16SkeletonCertificate, PAPER15_FINAL_CERTIFICATE, PAPER15_FORMAL_ENDPOINT,
    PAPER15_FROZEN_COMMIT,
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
fn docs_keep_errc004_active_and_success_claims_false() {
    let state = read_repo_file("GPD/state.json");
    let state_md = read_repo_file("GPD/STATE.md");
    let theorem = read_repo_file("docs/external_review_reproduction_certificates_theorem.md");

    assert_eq!(active_obligation(), "ERRC-004");
    assert!(state.contains("\"active_obligation\": \"ERRC-004\""));
    assert!(state.contains("\"errc002_finite_certificate_record_closed\": true"));
    assert!(state.contains("\"errc003_reviewer_protocol_provenance_closed\": true"));
    assert!(state.contains("\"external_review_reproduction_certificates_theorem_closed\": false"));
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
    assert!(theorem.contains("ERRC-004"));
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
