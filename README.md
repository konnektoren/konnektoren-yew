# Konnektoren-Yew

[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![Component Catalog](https://img.shields.io/badge/Component%20Catalog-📄-blue)](https://konnektoren.github.io/konnektoren-yew/catalog.html)

**Konnektoren-Yew** is the interactive web frontend for the Konnektoren language learning platform. Built with Rust and Yew, it provides a performant and type-safe user interface for engaging grammar challenges, vocabulary exercises, and learning tools. This repository focuses on the UI components and their integration with the `konnektoren-core` logic.

## ✨ Features

*   **Interactive Challenges:** Multiple choice, gap fill, ordering, and contextual choice exercises.
*   **Dynamic UI:** Components for profiles, progress, achievements, and game maps.
*   **Internationalization (i18n):** Full support for multiple languages with a flexible translation system.
*   **Theming & Design Modes:** Switch between light/dark themes and desktop/mobile layouts, with automatic mobile/desktop detection.
*   **Component Preview:** Integrated `yew-preview` for isolated component development and testing.
*   **Component Catalog:** Auto-generated static HTML catalog of all components — browsable offline and linkable in PRs.
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

### Component Catalog

Generate a self-contained HTML file showing every component with all its variants and test cases:

```bash
# Full build (compiles CSS via trunk, then generates catalog)
just catalog

# Fast path — reuse CSS already in dist/ from a previous build
just catalog-quick
```

The catalog is written to `dist/catalog.html` and is also deployed automatically to GitHub Pages on every push to `main`:

🔗 **[konnektoren.github.io/konnektoren-yew/catalog.html](https://konnektoren.github.io/konnektoren-yew/catalog.html)**

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
│   ├── bin/                 # Executable binaries (server, ssg, catalog)
│   │   ├── catalog.rs       # Generates static HTML component catalog
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

## Design Provider

`DesignProvider` exposes two related pieces of state:

*   `use_design()` returns the effective `Design` used by components such as navigation.
*   `use_design_mode()` returns the configuration: `Auto` or `Fixed(Design)`.

The provider resolves the effective design in this order:

1.  `forced` prop on `DesignProvider`, for app-level hard overrides.
2.  Explicit `<body class="design-*">` from HTML or an SSG template.
3.  Persisted user setting from `localStorage` key `konnektoren.design.mode`.
4.  Automatic detection.

Automatic detection uses the `mobile_breakpoint` prop, defaulting to `768`, and listens to media-query changes, window resize, and orientation changes. It also checks the physical screen width so mobile devices still get `Design::Mobile` if a page forgets the standard viewport meta tag.

`SelectDesign` cycles through `Auto`, `Desktop`, and `Mobile` by default. In `Auto`, the label shows the current effective design, for example `Auto (Mobile Design)`. Provider-written body classes are marked as managed, so a provider remount does not accidentally turn auto detection into a fixed body configuration.

Apps should include:

```html
<meta name="viewport" content="width=device-width, initial-scale=1.0" />
```

### Component Example

Use `use_design()` when the component structure differs between desktop and mobile. Keep the shared root class stable, and add a BEM modifier for design-specific styling:

```rust
use crate::model::Design;
use crate::providers::use_design;
use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct LessonToolbarProps {
    pub title: String,
    pub on_back: Callback<MouseEvent>,
}

#[function_component(LessonToolbar)]
pub fn lesson_toolbar(props: &LessonToolbarProps) -> Html {
    let design = use_design();
    let is_mobile = matches!(*design, Design::Mobile);

    let root_class = classes!(
        "lesson-toolbar",
        is_mobile.then_some("lesson-toolbar--mobile"),
        (!is_mobile).then_some("lesson-toolbar--desktop"),
    );

    if is_mobile {
        html! {
            <header class={root_class}>
                <button class="lesson-toolbar__icon-button" onclick={props.on_back.clone()}>
                    <i class="fas fa-arrow-left lesson-toolbar__icon"></i>
                </button>
                <h1 class="lesson-toolbar__title">{ &props.title }</h1>
            </header>
        }
    } else {
        html! {
            <header class={root_class}>
                <button class="lesson-toolbar__button" onclick={props.on_back.clone()}>
                    <i class="fas fa-arrow-left lesson-toolbar__icon"></i>
                    <span>{ "Back" }</span>
                </button>
                <h1 class="lesson-toolbar__title">{ &props.title }</h1>
                <div class="lesson-toolbar__actions"></div>
            </header>
        }
    }
}
```

```css
.lesson-toolbar {
    @apply flex items-center gap-3 bg-base-100 text-base-content border-b border-base-200;
}

.lesson-toolbar--desktop {
    @apply min-h-16 px-6;
}

.lesson-toolbar--mobile {
    @apply min-h-14 px-3;
}

.lesson-toolbar__title {
    @apply text-adaptive-base font-semibold truncate;
}

.lesson-toolbar--desktop .lesson-toolbar__title {
    @apply flex-1;
}

.lesson-toolbar--mobile .lesson-toolbar__title {
    @apply flex-1 text-center;
}

.lesson-toolbar__button {
    @apply btn btn-ghost gap-2;
}

.lesson-toolbar__icon-button {
    @apply btn btn-ghost btn-square;
}

.lesson-toolbar__icon {
    @apply transition-transform duration-300;
}

.lesson-toolbar__button:hover .lesson-toolbar__icon,
.lesson-toolbar__icon-button:hover .lesson-toolbar__icon {
    @apply scale-110;
}

.lesson-toolbar__actions {
    @apply flex min-w-24 justify-end gap-2;
}
```
