---
id: CHG-0002-correct-the-migration-contract-and-portable-verification-guidance
state: accepted
type: bug_fix
base_commit: 7128abdcc34663554453babc14ca28e748223903
---

# Correct the migration contract and portable verification guidance

## Intent

Correct the migration contract and portable verification guidance

## Affected Canonical Specs

- `gitleaks`

## Acceptance Criteria

- The canonical hook contract matches the existing marker-based whole-file install and removal behavior without promising partial-content preservation; Claude
- Cursor
- and Gemini correctly preserve full SpecSync input; documentation changes under docs require the SDD lifecycle; the native smoke command is cross-platform; Fledge verification and strict SpecSync 100% coverage pass.

## No-spec Rationale

Not applicable
