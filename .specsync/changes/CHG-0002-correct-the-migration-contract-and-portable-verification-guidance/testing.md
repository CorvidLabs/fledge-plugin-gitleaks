---
change: CHG-0002-correct-the-migration-contract-and-portable-verification-guidance
artifact: testing
---

# Testing

- Run `fledge lanes run verify` for formatting, clippy, four Rust unit tests, release build, and platform-neutral CLI help.
- Run `specsync check --strict --require-coverage 100 --force` and require real Rust file and LOC coverage.
- Confirm the canonical hook requirement matches `install_hook` and `uninstall_hook` without claiming partial-content preservation.
- `REQ-gitleaks-004` evidence is direct source inspection of `HOOK_MARKER`, `install_hook`, and `uninstall_hook`; this migration does not alter their Rust behavior.
- Require the hosted Linux, macOS, Windows, Trust, and CodeQL jobs before promotion.
