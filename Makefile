.PHONY: test-fast lean-build

test-fast:
	cargo test --workspace --test physical_promotion_attempt

lean-build:
	cd GPD/formal && lake build
