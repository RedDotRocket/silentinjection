# Makefile for Hugging Face Usage Scanner in Rust

# Crate/binary name (should match Cargo.toml)
BINARY_NAME := hfscanner
BUILD_DIR := target/release

# Default target (release build)
all: release

# Optimized release build with stripped symbols
release:
	RUSTFLAGS="-C link-arg=-s" cargo build --release

# Rename binary if Cargo.toml crate name differs
# You can comment this out if binary is already named correctly
rename:
	@if [ -f $(BUILD_DIR)/scanner ]; then \
		mv $(BUILD_DIR)/scanner $(BUILD_DIR)/$(BINARY_NAME); \
	fi

# Clean previous build
clean:
	cargo clean

# Run scanner with summary report
run-summary:
	$(BUILD_DIR)/$(BINARY_NAME) $(DIR) --summary

# Run scanner with detailed project status
run-detailed:
	$(BUILD_DIR)/$(BINARY_NAME) $(DIR) --detailed

# Run scanner with detailed output and CSV export
run-csv:
	$(BUILD_DIR)/$(BINARY_NAME) $(DIR) --detailed --csv report.csv

# Print help
help:
	@echo "Usage: make [target] [DIR=/path/to/code]"
	@echo ""
	@echo "Targets:"
	@echo "  all / release         Build optimized binary"
	@echo "  rename                Rename binary (only needed if crate name is not hf_scanner)"
	@echo "  clean                 Remove build artifacts"
	@echo "  run-summary DIR=...   Run scanner with --summary"
	@echo "  run-detailed DIR=...  Run scanner with --detailed"
	@echo "  run-csv DIR=...       Run scanner and write report.csv"
	@echo "  help                  Show this message"
