# justfile
# Set default values for environment variables

export BUILD_DIR := env_var_or_default("BUILD_DIR", "dist")
export REPORTS_DIR := env_var_or_default("REPORTS_DIR", "reports")

# Default recipe to display help information
default:
    @just --list

# Setup everything
setup: setup-rust

# Setup Rust tools
setup-rust:
    cargo install trunk
    cargo install wasm-pack
    rustup target add wasm32-unknown-unknown

# Start development server
serve:
    trunk serve --features=csr,yew-preview,chat

# Build the project for release
build: sbom
    #!/usr/bin/env bash
    set -euo pipefail
    echo "Building with BUILD_DIR=${BUILD_DIR}"

    # Create build directory if it doesn't exist
    mkdir -p ${BUILD_DIR}

    # Main build
    trunk build --release --features=csr,yew-preview

    # Static Site Generate
    cargo run --bin ssg --features ssg,yew-preview

# Run all tests
test: test-cargo test-wasm test-i18n

# Run cargo tests
test-cargo:
    cargo test

# Run i18n completeness check
test-i18n:
    cargo test --test i18n_completeness

# Generate i18n report
i18n-report:
    I18N_SRC_DIR=${SRC_DIR:-src} I18N_REPORTS_DIR=${REPORTS_DIR:-reports} cargo run --bin i18n-report --features tools

# CI-specific settings
ci-test-i18n:
    #!/usr/bin/env bash
    set -euo pipefail
    just i18n-report
    if [ -f "${REPORTS_DIR}/i18n_summary.md" ]; then
        cat "${REPORTS_DIR}/i18n_summary.md"
    fi

# Run wasm tests in Firefox
test-wasm:
    wasm-pack test --headless --firefox

# Clean build artifacts and reports
clean:
    rm -rf ${BUILD_DIR}
    rm -rf reports
    cargo clean

# Format code
fmt:
    cargo fmt

# Check code formatting
fmt-check:
    cargo fmt --check

# Show current configuration
config:
    @echo "Current configuration:"
    @echo "BUILD_DIR: ${BUILD_DIR}"

# Update all dependencies
update: update-rust

# Update Rust dependencies
update-rust:
    cargo update

lint:
    cargo clippy -- -D warnings

# Generate SBOM
sbom:
    cargo install cargo-cyclonedx
    cargo cyclonedx --format json

server:
    cargo run --bin konnektoren-yew-server --features server,yew-preview

# Generate static HTML component catalog (builds trunk first to get compiled CSS)
catalog:
    #!/usr/bin/env bash
    set -euo pipefail
    mkdir -p dist
    trunk build --features=csr,yew-preview
    find dist -name "*.css" | sort | xargs cat > dist/.catalog-combined.css
    OUTPUT=dist/catalog.html CSS_FILE=dist/.catalog-combined.css cargo run --bin catalog --features catalog
    rm -f dist/.catalog-combined.css

# Generate catalog using already-built CSS in dist/ (fast, requires prior trunk build)
catalog-quick:
    #!/usr/bin/env bash
    set -euo pipefail
    if ! ls dist/*.css 1>/dev/null 2>&1; then
        echo "No CSS found in dist/ — run 'just catalog' or 'just build' first."
        exit 1
    fi
    find dist -name "*.css" | sort | xargs cat > dist/.catalog-combined.css
    OUTPUT=dist/catalog.html CSS_FILE=dist/.catalog-combined.css cargo run --bin catalog --features catalog
    rm -f dist/.catalog-combined.css
