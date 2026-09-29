---
change: CHG-0002-correct-the-migration-contract-and-portable-verification-guidance
artifact: context
---

# Context

Review found that the migration spec promised partial managed-hook removal, while the existing Rust implementation owns the complete marker-bearing hook file and removes that file as a unit. The same review identified generated agent prompts that mishandle natural-language input, a Gemini variable that is not interpolated, an SDD policy that tracks a nonexistent `site/` instead of published `docs/`, and a Unix-only release-binary smoke path.

This correction changes governance, tests, and the canonical description of current behavior. It does not modify Rust implementation code.
