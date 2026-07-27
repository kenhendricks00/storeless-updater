# Security model

ChatGPT Portable retrieves executable content, so downloaded bytes are treated as untrusted until every validation boundary succeeds.

## Package acceptance

A package is accepted only when all of the following are true:

- It was resolved for the compiled-in Microsoft product ID `9PLM9XGG6VKS`.
- The delivery host is `delivery.mp.microsoft.com` or a subdomain of it, with standard HTTP or HTTPS ports only. Microsoft currently returns an HTTP CDN URL; the mandatory Windows signature and fixed identity checks provide payload authenticity and integrity before any extracted file can run.
- Windows `WinVerifyTrust` reports a valid signature.
- `AppxManifest.xml` declares package name `OpenAI.Codex`.
- The manifest publisher is `CN=50BDFD77-8903-4850-9FFE-6E8522F64D5B`.
- The package architecture is x64.
- The application executable is a relative path below `app/`.
- Every extracted archive entry remains below the temporary extraction root.

The temporary version is promoted only after extraction and manifest validation complete. Existing valid versions are not deleted until the new version is ready, and at least two versions are retained by default.

## Network boundaries

The catalog resolver contacts Microsoft DisplayCatalog and Windows Update FE3 endpoints. Redirected package content is restricted to Microsoft's delivery domain. No custom package URL is accepted by the normal installer flow.

## Local data boundaries

The launcher manages only its selected install root, its own configuration and logs, optional shortcut, and optional uninstall registry entry. It does not inspect or delete ChatGPT account data. Uninstallation leaves ChatGPT sign-in state, settings, and logs in place.

## Out of scope

- Vulnerabilities in the official ChatGPT application or Microsoft services.
- Compromise of a correctly trusted OpenAI package-signing identity.
- Code signing of locally built launcher executables.
- Store-only Windows integrations unavailable to an unpackaged application.

## Reporting

Do not include account tokens, cookies, credentials, or private ChatGPT logs in a public report. A report should include the launcher version, Windows version, exact error text, and reproduction steps using non-sensitive data.
