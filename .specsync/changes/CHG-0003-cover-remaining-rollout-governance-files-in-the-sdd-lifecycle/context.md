---
change: CHG-0003-cover-remaining-rollout-governance-files-in-the-sdd-lifecycle
artifact: context
---

# Context

Expanding `meaningful_paths` made the migration's committed Attest, Augur, and generated agent files subject to change coverage. Those files were already part of the rollout branch, but CHG-0001 did not enumerate each path, so hosted SpecSync correctly rejected the pull-request range as uncovered.

This workspace supplies portable coverage metadata for those already-migrated governance files. It does not modify their contents or Gitleaks runtime behavior.
