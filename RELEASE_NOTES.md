# BinaryFerry v0.1.0

BinaryFerry is an independent, unofficial Windows launcher and updater for the official ChatGPT desktop app. It provides a Store-free installation path while continuing to obtain the application package from Microsoft's own delivery service.

## Highlights

- Portable mode requires no administrator rights, background service, or Microsoft Store client.
- Discovers the current official package and checks for application updates on each launch.
- Validates the Windows package signature, exact OpenAI package identity, publisher, architecture, manifest launch target, and archive paths before use.
- Keeps versioned application directories and retains the previous version for rollback.
- Can update BinaryFerry itself from its public GitHub releases.

## Requirements and caveats

- Windows 10 build 19041 or newer, x64 only.
- The initial official application download is large and may take several minutes.
- The launcher is not code-signed, so Windows SmartScreen may display a warning for a newly downloaded release.
- Some features that depend on Store package registration or OS-managed capabilities may behave differently when the application is run unpackaged.

BinaryFerry does not include or redistribute ChatGPT, alter the official payload, bypass sign-in, or remove OpenAI's account requirements or terms. It is not affiliated with, endorsed by, or supported by OpenAI or Microsoft.

## Verify this build

This release is built from its Git tag by GitHub Actions and receives a Sigstore build-provenance attestation. The published SHA-256 is included in `binaryferry.exe.sha256`.

```powershell
gh attestation verify binaryferry.exe --repo kenhendricks00/binaryferry
```
