---
change: CHG-0001-adopt-specsync-5-0-1-and-trust-1-0-0-governance-for-the-gitleaks-fledge-plugin
artifact: context
---

# Context

The plugin already ships a Rust CLI, multi-platform release artifacts, tests, formatting, linting, and Pages documentation. The organization rollout adds the shared SpecSync 5.0.1 lifecycle and Trust 1.0.0 gate without replacing those specialized workflows.

The canonical specification records the existing scan, check, and managed-hook behavior. This migration does not change the CLI contract.
