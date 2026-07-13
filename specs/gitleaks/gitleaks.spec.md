---
module: gitleaks
version: 3
status: active
files:
  - src/main.rs

db_tables: []
depends_on: []
---

# Gitleaks

## Purpose

Wrap the installed gitleaks binary for working-tree and staged secret scanning, provide human/CI result modes, and install or remove a marker-owned pre-commit hook without overwriting an unmanaged existing hook.

## Public API

| Command | Behavior |
|---------|----------|
| scan | Run gitleaks against the working tree or staged changes and pretty-print findings. |
| check | Run CI-style detection and return non-zero when secrets are found or scanning fails. |
| install-hook | Create the managed staged-check pre-commit hook, or refuse an unmanaged existing hook. |
| uninstall-hook | Remove the complete marker-bearing managed hook, or refuse an unmanaged hook. |

## Invariants

1. The plugin delegates secret detection and `.gitleaks.toml` semantics to the installed gitleaks binary.
2. Staged mode scans only staged changes and is the mode installed into pre-commit.
3. Check exits non-zero when findings exist or gitleaks cannot complete successfully.
4. Hook installation is idempotent for a marker-bearing hook and refuses to overwrite an unmanaged existing hook.
5. Hook removal deletes the complete marker-bearing hook and refuses to delete a hook without the managed marker.
6. A newly installed hook is executable on Unix platforms.
7. Missing gitleaks produces installation guidance rather than a false pass.

## Behavioral Examples

```
Given a pre-commit hook without the plugin's managed marker
When install-hook or uninstall-hook runs
Then the command refuses to overwrite or remove that unmanaged hook
```

## Error Cases

| Error | When | Behavior |
|-------|------|----------|
| Gitleaks unavailable | Scan/check starts without the binary | Report prerequisite guidance and exit non-zero. |
| Secret detected | Gitleaks reports findings | Pretty-print findings and make check fail. |
| Not a Git repository | Staged or hook operation has no Git context | Report the repository precondition and exit non-zero. |
| Hook write failure | Hook directory/file cannot be updated | Preserve existing content and surface the I/O failure. |
| Unknown subcommand/options | CLI input is unsupported | Print scoped help and exit non-zero. |

## Dependencies

- Rust 1.89 or later
- installed gitleaks binary
- Git repository for staged scans and hooks
- `clap`, `serde`, `serde_json`, and `anyhow`

## Change Log

| Version | Date | Changes |
|---------|------|---------|
| 1 | 2026-07-12 | Document existing gitleaks scan, check, and managed-hook behavior for SpecSync 5 adoption. |
| 2 | 2026-07-13 | CHG-0001-adopt-specsync-5-0-1-and-trust-1-0-0-governance-for-the-gitleaks-fledge-plugin: Adopt SpecSync 5.0.1 and Trust 1.0.0 governance for the Gitleaks Fledge plugin |
| 3 | 2026-07-13 | CHG-0002-correct-the-migration-contract-and-portable-verification-guidance: Correct the migration contract and portable verification guidance |
