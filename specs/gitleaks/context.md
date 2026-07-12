---
spec: gitleaks.spec.md
---

## Context

This Rust plugin replaces an archived custom scanner with a thin, auditable wrapper over the established gitleaks engine.

## Related Modules

- gitleaks CLI and configuration
- Git pre-commit hooks

## Design Decisions

- Delegate detection to gitleaks rather than maintain custom patterns.
- Mark and remove only plugin-owned hook content.
