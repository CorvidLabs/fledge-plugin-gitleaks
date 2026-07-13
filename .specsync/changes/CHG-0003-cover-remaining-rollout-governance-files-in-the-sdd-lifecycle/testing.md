---
change: CHG-0003-cover-remaining-rollout-governance-files-in-the-sdd-lifecycle
artifact: testing
---

# Testing

- Run `fledge lanes run verify` for formatting, clippy, four Rust tests, release build, and CLI help.
- Run `specsync check --strict --require-coverage 100 --force` and require 1/1 file plus 326/326 LOC coverage.
- Confirm the hosted Trust contract gate no longer reports the seven rollout governance paths as uncovered.
