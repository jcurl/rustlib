define CRATE_template
$(1)_format: .PHONY
	@echo "Format check $(1)..."
	@(cargo fmt --all --check \
	  && { exit 0; } \
	  || { echo "Automatically Formatting $(1)..."; cargo fmt && exit 1; })

$(1)_lint: .PHONY
	@echo "Lint $(1)..."
	@cargo clippy

$(1)_build: .PHONY
	@echo "Build $(1)..."
	@cargo build

$(1)_release: .PHONY
	@echo "Release $(1)..."
	@cargo build --release

$(1)_test: .PHONY
	@echo "Test $(1)..."
	@cargo test

$(1)_cov: .PHONY
	@echo "Coverage $(1)..."
	@cargo llvm-cov --open

$(1)_clean: .PHONY
	@echo "Clean $(1)..."
	@cargo clean

.PHONY:

endef
