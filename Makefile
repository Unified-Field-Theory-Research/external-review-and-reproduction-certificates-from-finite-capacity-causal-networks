.PHONY: test-fast lean-build

test-fast:
	cargo test --workspace --test external_review_reproduction_certificates

lean-build:
	cd GPD/formal && lake build
