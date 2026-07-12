---
change: CHG-0001-adopt-specsync-5-0-1-and-trust-1-0-0-governance-for-the-gitleaks-fledge-plugin
artifact: testing
---

# Testing

Local acceptance requires:

- `fledge lanes run verify`
- `specsync check --strict --require-coverage 100 --force`
- `specsync agents status`
- `fledge trust doctor`
- `git diff --check`

Hosted acceptance requires the new `trust` job plus every existing CI, release-relevant, security, and compatibility check to remain green on the repository's normal runners.
