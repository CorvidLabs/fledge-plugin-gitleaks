---
spec: gitleaks.spec.md
---

## Test Plan

### Integration Tests

- `cargo fmt --check`
- `cargo clippy -- -D warnings`
- `cargo test`
- `cargo build --release`
- Verify root and all subcommand help surfaces.
