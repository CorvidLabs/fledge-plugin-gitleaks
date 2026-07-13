---
id: CHG-0003-cover-remaining-rollout-governance-files-in-the-sdd-lifecycle
state: accepted
type: migration
base_commit: 0ae5e3a2cd3f8da88210c4d9dd79749e8223b3b1
---

# Cover remaining rollout governance files in the SDD lifecycle

## Intent

Cover remaining rollout governance files in the SDD lifecycle

## Affected Canonical Specs

- None

## Acceptance Criteria

- SpecSync recognizes the Attest
- Augur
- Claude
- Cursor
- and Gemini rollout files as covered delivery inputs; strict validation retains real 1/1 file and 326/326 LOC coverage; the native Fledge lane and hosted Trust contract gate pass.

## No-spec Rationale

Declare the already-migrated Attest, Augur, and generated agent governance files as delivery inputs so the pull-request range is fully covered without changing Gitleaks requirements or Rust behavior.
