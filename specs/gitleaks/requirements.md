---
spec: gitleaks.spec.md
---

## User Stories

- As a developer, I want working-tree and staged secret scans through fledge.
- As a repository owner, I want a managed pre-commit check without losing existing hook behavior.

## Acceptance Criteria

### REQ-gitleaks-001

The plugin SHALL run working-tree and staged gitleaks scans and honor repository configuration.

### REQ-gitleaks-002

The CI check SHALL exit non-zero for findings, missing prerequisites, or scan failures.

Acceptance Criteria
- The native Fledge verification lane passes formatting, clippy, tests, release build, and CLI help smoke.
- Hosted Trust and existing CI remain required after the owner-approved lifecycle is recorded.

### REQ-gitleaks-003

Hook installation SHALL be idempotent and invoke the staged check.

### REQ-gitleaks-004

Hook removal SHALL preserve all non-managed user content and required executable permissions.

## Constraints

- Requires gitleaks; staged and hook operations require Git repository context.

## Out of Scope

- Reimplementing secret detection, installing gitleaks, and modifying unrelated hooks.
