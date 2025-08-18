# Konnektoren-Yew

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

**Konnektoren-Yew** is the interactive web frontend for the Konnektoren language learning platform. Built with Rust and Yew, it provides a performant and type-safe user interface for engaging grammar challenges, vocabulary exercises, and learning tools. This repository focuses on the UI components and their integration with the `konnektoren-core` logic.

## ✨ Features

*   **Interactive Challenges:** Multiple choice, gap fill, ordering, and contextual choice exercises.
*   **Dynamic UI:** Components for profiles, progress, achievements, and game maps.
*   **Internationalization (i18n):** Full support for multiple languages with a flexible translation system.
*   **Theming & Design Modes:** Switch between light/dark themes and desktop/mobile layouts.
*   **Component Preview:** Integrated `yew-preview` for isolated component development and testing.
*   **Progress Persistence:** Uses local storage for saving user progress and settings.
*   **Server-Side Rendering (SSR):** Supports SSR for improved SEO and initial load performance.
*   **Build-time SBOM:** Generates a Software Bill of Materials for enhanced supply chain security.
*   **Optional Integrations:**
    *   **Chat:** Real-time chat functionality.
    *   **Marketplace:** Web3 wallet integration (TON, Solana) for in-app purchases.
    *   **Google Drive Backup:** User data backup and restore.
    *   **Text-to-Speech:** Read aloud functionality for challenge content.

## 🚀 Getting Started

To get the project up and running for development or to build for production.

### Prerequisites

*   [Rust toolchain](https://www.rust-lang.org/tools/install) (latest stable recommended)
*   [Just](https://github.com/casey/just) (a command runner, similar to Make)

### Setup

Install the necessary `cargo` tools and Rust target:

```bash
just setup
```

### Development Server

Start a local development server with hot-reloading. This will enable client-side rendering (CSR) and the `yew-preview` feature, useful for developing individual components.

```bash
just serve
```

Then, open your browser to `http://localhost:8080`.

### Building for Release

To create a production-ready static build, including Static Site Generation (SSG) for SEO:

```bash
just build
```
The output will be generated in the `dist/` directory.

### Running the SSR Server (Optional)

You can run a local server that demonstrates server-side rendering of individual components:

```bash
just server
```
Visit `http://localhost:3000` in your browser.

## 🧪 Testing

Run all types of tests:

```bash
just test
```

### Unit & Integration Tests

Run Rust unit and integration tests:

```bash
just test-cargo
```

### WebAssembly Tests

Run headless browser tests for WebAssembly code (requires Firefox):

```bash
just test-wasm
```

### Internationalization Completeness Check

Check for missing translations across supported languages:

```bash
just test-i18n
```

## 🧹 Maintenance

*   **Clean build artifacts:** `just clean`
*   **Format code:** `just fmt`
*   **Check formatting:** `just fmt-check`
*   **Lint code:** `just lint` (runs `clippy` with warnings as errors)
*   **Update dependencies:** `just update`

## 📦 Project Structure

```
konnektoren-yew/
├── assets/                  # Static assets (images, i18n files, custom challenge resources)
├── src/
│   ├── app.rs               # Main Yew application for Client-Side Rendering (CSR)
│   ├── app_ssr.rs           # Main Yew application for Server-Side Rendering (SSR)
│   ├── bin/                 # Executable binaries (server, ssg)
│   │   ├── server.rs        # Axum server for SSR demonstration
│   │   └── ssg.rs           # Static Site Generation tool
│   ├── components/          # Reusable Yew UI components (structured by domain)
│   │   ├── ads/
│   │   ├── analytics/
│   │   ├── challenge/
│   │   ├── certificates/
│   │   ├── marketplace/
│   │   ├── map/
│   │   ├── navigation/
│   │   ├── profile/
│   │   ├── settings/
│   │   └── ...              # Other generic components
│   ├── effects/             # UI-specific effects (e.g., animations, text-to-speech)
│   ├── i18n/                # Internationalization logic and utilities
│   ├── managers/            # Components for managing global state and interacting with providers
│   ├── model/               # Application-specific data structures
│   ├── providers/           # Yew Context Providers for global state and dependency injection
│   ├── repository/          # Data persistence layer (local storage, GDrive, traits)
│   ├── tools/               # Utility functions (e.g., HTTP tracing)
│   └── lib.rs               # Crate root, defines public modules and prelude
├── build.rs                 # Custom build script (versioning, SBOM)
├── Cargo.toml               # Project dependencies and features
├── justfile                 # Task automation scripts
└── README.md                # This file
```

## 🏗️ Architecture Highlights

Konnektoren-Yew is designed with a layered architecture:

*   **Presentation Layer (`components/`):** Yew components responsible for rendering the UI and handling user interactions. They receive data via props or consume contexts from providers.
*   **Application Layer (`managers/`, `providers/`, `model/`, `i18n/`):** Manages application-specific logic, state, and services. Providers inject dependencies and share state across the component tree. Managers interact with these providers to coordinate more complex UI behaviors.
*   **Domain/Core Layer (`konnektoren-core` crate - external):** Contains the core business logic, game mechanics, challenge definitions, and domain models, independent of the UI framework.
*   **Infrastructure Layer (`repository/`, `tools/`):** Handles external concerns like data persistence (local storage, cloud backups), network communication, and utility functions.

This separation promotes maintainability, testability, and scalability.
