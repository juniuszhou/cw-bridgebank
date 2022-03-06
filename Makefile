.PHONY: all ## Build release version
all:
	cargo build --release --target wasm32-unknown-unknown

.PHONY: fmtcheck ## cargo fmt check
fmtcheck:
	cargo fmt --all -- --check

.PHONY: fmt ## cargo fmt all
fmt:
	cargo fmt --all