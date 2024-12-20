# Konnektoren-Yew

This is the repository for the Konnektoren Yew UI.

## Setup

```bash
just setup
```

## Build

```bash
cargo build
```

## Serve

Serve the demo page with

```bash
trunk serve
```

then visit http://localhost:8080

## Test

```bash
cargo test
wasm-pack test --firefox --headless
```

To test the components in isolation, run the following command:

```bash
trunk serve --features=yew-preview
```
