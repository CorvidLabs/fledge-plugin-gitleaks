---
change: CHG-0001-adopt-specsync-5-0-1-and-trust-1-0-0-governance-for-the-gitleaks-fledge-plugin
artifact: design
---

# Design

Add one active `gitleaks` module specification with stable requirement IDs mapped to `src/main.rs`. Configure the SDD lifecycle at version 5.0.1 and install the four supported agent integrations.

The local `verify` lane runs formatting, Clippy with warnings denied, unit tests, a release build, and the CLI help smoke. Trust uses that lane for lifecycle evidence, blocks high risk, enforces 100% contract coverage, verifies provenance progressively, and leaves Atlas disabled.

The new GitHub `trust` job uses the immutable Trust 1.0.0 commit and full Git history. Existing CI, release, and Pages workflows remain unchanged.
