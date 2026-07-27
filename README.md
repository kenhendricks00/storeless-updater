# BinaryFerry

BinaryFerry is an independent, unofficial Windows launcher and updater for OpenAI's ChatGPT desktop app. It downloads Microsoft's current signed package directly, validates it, extracts it into a versioned folder, and runs it without installing through or opening the Microsoft Store client.

It does not redistribute ChatGPT, modify the ChatGPT payload, bypass sign-in, or remove OpenAI's normal terms and account requirements.

## Quick start

1. Download or build `binaryferry.exe`.
2. Run it and keep the recommended **Portable** mode selected.
3. Choose an install folder and click **Install**.
4. The first download is large, so allow several minutes.
5. Keep launching ChatGPT through `binaryferry.exe`. It checks for an official update on each launch and asks before applying one.

Portable mode creates a self-contained application folder and does not require administrator rights, registry entries, a background service, or the Microsoft Store app. Optional per-user and system installation modes are also available.

## What "portable" means

The launcher and extracted application files are portable. ChatGPT still stores sign-in state, settings, and logs in its normal Windows profile locations. This is deliberate: the launcher never reads, copies, or bundles credentials. Moving the application folder to another computer does not move your ChatGPT account session.

## How it works

1. Resolves product `9PLM9XGG6VKS` through Microsoft's DisplayCatalog and Windows Update FE3 services.
2. Accepts package downloads only from Microsoft's delivery domain. Microsoft currently returns a signed HTTP CDN URL, so authenticity and integrity are enforced by the mandatory Windows package-signature and identity checks before extraction.
3. Verifies the MSIX with Windows `WinVerifyTrust` before extraction.
4. Requires the exact `OpenAI.Codex` package identity, OpenAI publisher certificate identity, and x64 architecture.
5. Reads the launch target from the validated `AppxManifest.xml`. The current package uses `app/ChatGPT.exe`, so the launcher does not depend on an obsolete hard-coded filename.
6. Extracts with path-traversal protection, promotes only a complete validated version, and retains the previous version for rollback.
7. Starts the newest valid version. If the catalog is temporarily unavailable, an already installed version can still launch.

The Microsoft Store **client** is not required. Microsoft's catalog, update, and package-delivery servers are still required to discover and download official releases.

## Security model

The package source, allowlisted Microsoft delivery host, Windows signature, package identity, publisher, architecture, manifest executable path, and archive paths are validated before a downloaded version can be promoted or launched. See [docs/SECURITY.md](docs/SECURITY.md) for the detailed boundaries.

The launcher itself is an independent open-source project and is not signed with an OpenAI or Microsoft certificate. Windows SmartScreen may warn about a newly downloaded build. Public releases should include a SHA-256 file and GitHub build-provenance attestation.

## Limitations

- Windows 10 build 19041 or newer, x64 only.
- The Microsoft catalog protocol is not a public application API and can change.
- Features that require Store package registration, protocol registration, or OS-managed capabilities may behave differently when the payload is run unpackaged.
- ChatGPT updates are automatic only while using this launcher. There is no background updater.
- The official package is large and multiple retained versions require additional disk space.
- BinaryFerry checks its own public GitHub releases so future launcher security and compatibility updates can be installed from the same interface.

## Build from source

Install the stable Rust toolchain on x64 Windows, then run:

```powershell
cargo build --locked --release
```

The executable is written to `target\release\binaryferry.exe`.

Before publishing a change, run:

```powershell
cargo fmt --all -- --check
cargo test --locked --all-targets
cargo clippy --locked --all-targets -- -D warnings
cargo build --locked --release
```

To test only the live catalog resolver without installing or launching ChatGPT:

```powershell
cargo run --locked --release -- --test-fetch
```

## Project status and provenance

This project is derived from the MIT-licensed [`vaportail/codex-windows-updater`](https://github.com/vaportail/codex-windows-updater). Its direct Microsoft package resolver is adapted from that work, which credits StoreDev/StoreLib for the FE3 request templates. See [THIRD_PARTY_NOTICES.md](THIRD_PARTY_NOTICES.md).

BinaryFerry is the project's own name. "ChatGPT" is used only to identify the official application with which it interoperates. ChatGPT and OpenAI are trademarks of OpenAI. Windows and Microsoft Store are trademarks of Microsoft. This project is not affiliated with, endorsed by, or supported by OpenAI or Microsoft.

The project name was selected through a documented [preliminary name screen](docs/NAME_CLEARANCE.md). That screen is not legal advice or a guarantee of trademark availability.

## License

MIT. See [LICENSE](LICENSE).
