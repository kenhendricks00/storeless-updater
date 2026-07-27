# Spec: ChatGPT Portable

## Objective

Build a small, unofficial Windows launcher that lets people download, update, and run the official ChatGPT desktop application without opening or depending on the Microsoft Store client. The launcher downloads the free x64 package from Microsoft's catalog and CDN at runtime, validates it, extracts only the desktop payload, keeps two versions for rollback, and launches the newest valid version.

The launcher does not redistribute ChatGPT, bypass authentication, alter OpenAI code, or make the application data portable between computers. Users still authenticate normally and remain subject to OpenAI's terms.

## Assumptions approved for implementation

1. Windows 10 version 2004 (build 19041) or newer, x64 only.
2. Portable mode is the primary experience and needs no administrator rights, registry entry, background service, or Store client.
3. Updates are checked when the launcher runs, with a user-visible choice before replacing the active version.
4. Microsoft DisplayCatalog, Windows Update FE3, and the allowlisted Microsoft delivery CDN are the only package sources. The CDN may use the signed HTTP URL currently returned by FE3; package authenticity is enforced with Windows signature verification before extraction.
5. Product ID `9PLM9XGG6VKS`, package identity `OpenAI.Codex`, publisher `CN=50BDFD77-8903-4850-9FFE-6E8522F64D5B`, and x64 architecture are fixed trust constraints.
6. The executable path is read from `AppxManifest.xml`; the current manifest points to `app/ChatGPT.exe`.
7. User data remains in the application's normal `%APPDATA%` and `%LOCALAPPDATA%` locations so credentials are not copied between machines.

## Tech stack

- Rust 2021 with Rust 1.88 minimum
- Slint for the native Windows wizard
- `reqwest` with native-root TLS for Microsoft HTTPS calls
- `quick-xml` and `serde_json` for catalog responses
- `zip` for MSIX extraction with traversal-safe paths
- Windows trust APIs for package signature verification

The Store protocol templates and resolver are adapted from the MIT-licensed `vaportail/codex-windows-updater`, which in turn credits StoreDev/StoreLib. Required notices remain in the repository and UI.

## Commands

```powershell
cargo build --locked --release
cargo test --locked
cargo fmt --all -- --check
cargo clippy --locked --all-targets -- -D warnings
cargo run --locked --release -- --test-fetch
```

## Project structure

```text
src/                 Rust application source
src/store/           Isolated Microsoft catalog and delivery resolver
src/store/templates/ SOAP request templates
ui/                  Slint interface
docs/                Product spec, plan, and security notes
.github/workflows/   Reproducible CI and release builds
```

Runtime layout:

```text
ChatGPTPortable/
  chatgpt-portable.exe
  updater.json
  downloads/
  versions/
    26.721.4979.0/
      ChatGPT.exe
      resources/
```

## Code style

Use explicit error context and small boundary validators:

```rust
fn validate_identity(identity: &PackageIdentity) -> anyhow::Result<()> {
    anyhow::ensure!(identity.name == EXPECTED_PACKAGE_NAME, "unexpected package identity");
    anyhow::ensure!(identity.architecture == "x64", "unsupported package architecture");
    Ok(())
}
```

Run `cargo fmt`; prefer descriptive names, `Result` propagation with context, and comments explaining protocol or security constraints rather than restating code.

## Testing strategy

- Unit tests cover version ordering, package selection, manifest identity parsing, executable-path normalization, safe ZIP joining, host allowlisting, and update-policy decisions.
- Integration-style tests create temporary synthetic MSIX archives and prove extraction rejects wrong identities and traversal paths.
- A live, read-only catalog smoke test resolves the current Microsoft response without installing anything.
- A release verification downloads the current package to a temporary location, validates trust and identity, extracts it, and confirms the resolved executable exists. It must not launch or modify an installed Store copy during automated tests.

## Threat model

- Spoofing/tampering: accept only the fixed product and package identity, verify Windows package trust, and validate the manifest before launch.
- Malicious redirects/SSRF: the product ID is compiled in; only Microsoft catalog, FE3, and delivery hosts are allowed.
- ZIP traversal: reject absolute, prefixed, rooted, and parent-directory entries.
- Partial/corrupt updates: download and extract to temporary paths, then atomically promote only after validation.
- Destructive cleanup: prune only numeric version directories inside the validated launcher root, keeping at least two versions.
- Credential exposure: never copy, inspect, or package ChatGPT profile data.

## Boundaries

- Always: validate downloaded content, preserve rollback, show useful errors, run tests before commits.
- Ask first: add another package source, support ARM64, change identity constraints, or add background update services.
- Never: redistribute ChatGPT, bypass licensing or authentication, disable certificate validation, execute from a partial directory, or delete user-created files.

## Success criteria

1. A clean x64 Windows 10/11 machine can run one launcher executable without opening the Microsoft Store.
2. First run downloads the current official package from Microsoft, verifies trust and identity, extracts it, and launches `ChatGPT.exe`.
3. Later launches detect a newer version, update safely, preserve the prior version, and launch the newest valid version.
4. Offline launches use the newest already-extracted valid version.
5. Corrupt, unsigned, wrong-identity, wrong-architecture, or traversal-containing packages are rejected before launch.
6. The repository builds, tests, formats, and lints cleanly from the lockfile.
7. Documentation clearly states the unofficial status, Microsoft network dependency, OpenAI ownership, and lack of affiliation.

## Open questions

None. The user delegated product decisions and approved proceeding with the recommended defaults.
