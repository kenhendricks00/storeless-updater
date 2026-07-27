# BinaryFerry contributor rules

## Stack and commands

- Rust 2021, minimum Rust 1.88
- Slint desktop UI
- Build: `cargo build --locked --release`
- Test: `cargo test --locked`
- Format: `cargo fmt --all -- --check`
- Lint: `cargo clippy --locked --all-targets -- -D warnings`
- Live catalog check: `cargo run --locked --release -- --test-fetch`

## Conventions

- Keep the launcher Windows x64 only until another architecture has an end-to-end test.
- Keep Microsoft Store protocol code isolated under `src/store/`.
- Treat every downloaded package, manifest value, ZIP path, and redirect as untrusted.
- Prefer small pure helpers with unit tests for selection, validation, and path logic.
- User-facing text must call the app ChatGPT and clearly state that the launcher is unofficial.

## Boundaries

- Always verify package trust and the expected `OpenAI.Codex` identity before extraction.
- Always extract into a partial directory and atomically promote it after validation.
- Never redistribute or commit OpenAI application packages or binaries.
- Never accept arbitrary download URLs from users.
- Never require the Microsoft Store client for the direct download path.
- Never commit build output, downloaded packages, credentials, or device-specific state.
