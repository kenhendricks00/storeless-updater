# Implementation plan: Storeless Updater

## Architecture decisions

- Adapt the proven MIT-licensed Rust resolver and native launcher instead of creating a second undocumented Store protocol implementation.
- Keep direct Microsoft catalog/CDN resolution as the default and WinGet only as an explicit fallback.
- Derive the launch target from the validated package manifest, not a fixed `Codex.exe` filename.
- Keep portable mode simple and rollback-friendly with versioned directories.

## Phase 1: Foundation

### Task 1: Establish the derivative project

Acceptance criteria:

- Upstream MIT notices and history attribution are preserved.
- Product metadata and documentation describe Storeless Updater accurately.
- The untouched baseline builds from its lockfile.

Verification: `cargo build --locked --release`.

Dependencies: none.

### Task 2: Prove current package compatibility with tests

Acceptance criteria:

- Tests fail against the old `Codex.exe` assumption.
- Manifest parsing accepts the current `OpenAI.Codex` x64 identity and resolves `app/ChatGPT.exe`.
- Wrong identity, publisher, architecture, and unsafe paths are rejected.

Verification: focused tests, followed by `cargo test --locked`.

Dependencies: Task 1.

## Checkpoint: foundation

- Tests pass and the current package layout is represented by fixtures.

## Phase 2: Secure update path

### Task 3: Validate downloads before promotion

Acceptance criteria:

- Redirects and final download URLs are limited to expected Microsoft hosts.
- Windows package trust verification runs before extraction.
- Manifest identity and executable path are checked before version promotion.

Verification: unit tests plus a live package download to a temporary directory.

Dependencies: Task 2.

### Task 4: Launch and update current ChatGPT builds

Acceptance criteria:

- Proxy, shortcut, process detection, and updater flows use the resolved `ChatGPT.exe` target.
- Offline launch uses the newest valid installed version.
- Two versions are retained and partial updates are ignored.

Verification: `cargo test --locked` and a temporary end-to-end extraction.

Dependencies: Task 3.

## Checkpoint: core flow

- A current official package resolves, validates, extracts, and produces a runnable target without touching the Store client.

## Phase 3: Public-facing polish

### Task 5: Rebrand and simplify the native UI

Acceptance criteria:

- User-facing text consistently says Storeless Updater.
- The UI clearly labels the project unofficial and retains required Slint attribution.
- Loading, success, error, update, offline, and rollback states are understandable and keyboard accessible.

Verification: build, keyboard walkthrough, and screenshot review.

Dependencies: Task 4.

### Task 6: Harden release documentation and automation

Acceptance criteria:

- README explains usage, limitations, trust model, build commands, and affiliation disclaimer.
- CI runs format, test, Clippy, and release build checks.
- Build artifacts and downloaded packages remain ignored.

Verification: all documented commands pass and Git status is clean after commit.

Dependencies: Task 5.

## Risks and mitigations

| Risk | Impact | Mitigation |
| --- | --- | --- |
| Microsoft changes private catalog protocol | High | Isolate resolver, provide WinGet/manual-package fallback, return actionable errors |
| OpenAI changes package layout again | High | Parse and validate the manifest instead of hard-coding an executable name |
| Unsigned public launcher triggers SmartScreen | Medium | Publish hashes and GitHub build provenance; document that the launcher itself is not code-signed |
| Store-only capabilities behave differently unpackaged | Medium | Document limitation and keep the official packaged payload unmodified |
| Trademark confusion | Medium | Use an explicit unofficial/non-affiliation notice and no copied OpenAI artwork in the launcher |

## Approval

The user delegated decisions with “Do whatever you think.” This plan implements the narrow goal stated afterward: give people a way to use the official ChatGPT desktop app without relying on the Windows Store client.
