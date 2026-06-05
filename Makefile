.PHONY: test-fast lean-build

test-fast:
	cargo test --workspace --test prediction_falsification_protocols

lean-build:
	cd GPD/formal && lake build
