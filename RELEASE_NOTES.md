# Storeless Updater v0.2.1

Storeless Updater is an independent, unofficial launcher and updater for the ChatGPT desktop app on Windows. It provides a Store-free installation path while continuing to obtain the application package from Microsoft's own delivery service.

## Download reliability

- Fixes Direct downloads after XML entity references caused Microsoft's signed URL to be truncated at its first query parameter. The complete signed URL is now preserved, avoiding a 403 response and unnecessary Winget fallback.
- Allows long package transfers to finish by replacing the previous two-minute total download timeout with a two-hour transfer window and a separate 30-second connection timeout.
- Keeps the current security boundaries: redirects must remain on Microsoft's delivery domain, and the completed package still must pass signature, identity, publisher, architecture, and archive validation before extraction.

## Highlights

- Portable mode requires no administrator rights, background service, or Microsoft Store client.
- Discovers the current official package and checks for application updates on each launch.
- Validates the Windows package signature, exact OpenAI package identity, publisher, architecture, manifest launch target, and archive paths before use.
- Keeps versioned application directories and retains the previous version for rollback.
- Can update Storeless Updater itself from its public GitHub releases.

## Requirements and caveats

- Windows 10 build 19041 or newer, x64 only.
- The initial official application download is roughly 667 MB. Its transfer speed depends on Microsoft's CDN and the user's network route.
- The launcher is not code-signed, so Windows SmartScreen may display a warning for a newly downloaded release.
- Some features that depend on Store package registration or OS-managed capabilities may behave differently when the application is run unpackaged.

Storeless Updater does not include or redistribute ChatGPT, alter the official payload, bypass sign-in, or remove OpenAI's account requirements or terms. It is not affiliated with, endorsed by, or supported by OpenAI or Microsoft.

## Verify this build

This release is built from its Git tag by GitHub Actions and receives a Sigstore build-provenance attestation. The published SHA-256 is included in `storeless-updater.exe.sha256`.

```powershell
gh attestation verify storeless-updater.exe --repo kenhendricks00/storeless-updater
```
