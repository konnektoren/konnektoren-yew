# justfile

# Import styles justfile
styles := "scss"

# Set default values for environment variables
export BUILD_DIR := env_var_or_default("BUILD_DIR", "dist")
export REPORTS_DIR := env_var_or_default("REPORTS_DIR", "reports")

# Default recipe to display help information
default:
    @just --list

# Setup everything
setup: setup-rust setup-styles

# Setup Rust tools
setup-rust:
    cargo install trunk
    cargo install wasm-pack
    rustup target add wasm32-unknown-unknown

# Setup styles
setup-styles:
    cd {{styles}} && just setup-vendors

# Start development server
serve:
    trunk serve --features=yew-preview

# Build the project for release
build: styles-check sbom
    #!/usr/bin/env bash
    set -euo pipefail
    echo "Building with BUILD_DIR=${BUILD_DIR}"

    # Create build directory if it doesn't exist
    mkdir -p ${BUILD_DIR}

    # Main build
    trunk build --release

# Run all tests
test: test-cargo test-wasm test-i18n

# Run cargo tests
test-cargo:
    cargo test

# Run i18n completeness check
test-i18n:
    #!/usr/bin/env bash
    chmod +x ./scripts/i18n_report.sh
    ./scripts/i18n_report.sh

# Generate i18n report
i18n-report:
    #!/usr/bin/env bash
    ./scripts/i18n_report.sh

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
update: update-rust update-styles

# Update Rust dependencies
update-rust:
    cargo update

# Check styles before build
styles-check:
    cd {{styles}} && just vendor-status

# Update style dependencies
update-styles:
    cd {{styles}} && just update-vendors

# Show styles status
styles-status:
    cd {{styles}} && just vendor-status

lint-style:
    npx stylelint "scss/**/*.{css,scss}" --config scss/.stylelintrc.json --ignore-path scss/.stylelintignore

# You might want to update your existing lint command to include style linting
lint: lint-style
    cargo clippy -- -D warnings

# Generate SBOM
sbom:
    cargo install cargo-cyclonedx
    cargo cyclonedx --format json

server:
    cargo run --bin konnektoren-yew-server --features server,yew-preview
