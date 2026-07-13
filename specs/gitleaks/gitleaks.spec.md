---
module: gitleaks
version: 2
status: active
files:
  - src/main.rs

db_tables: []
depends_on: []
---

# Gitleaks

## Purpose

Wrap the installed gitleaks binary for working-tree and staged secret scanning, provide human/CI result modes, and install or remove a managed pre-commit invocation without destroying unrelated hook content.

## Public API

| Command | Behavior |
|---------|----------|
| scan | Run gitleaks against the working tree or staged changes and pretty-print findings. |
| check | Run CI-style detection and return non-zero when secrets are found or scanning fails. |
| install-hook | Add the managed staged-check invocation to the repository pre-commit hook. |
| uninstall-hook | Remove only the plugin-managed hook content and preserve unrelated commands. |

## Invariants

1. The plugin delegates secret detection and `.gitleaks.toml` semantics to the installed gitleaks binary.
2. Staged mode scans only staged changes and is the mode installed into pre-commit.
3. Check exits non-zero when findings exist or gitleaks cannot complete successfully.
4. Hook installation is idempotent and does not duplicate the managed invocation.
5. Hook removal deletes only managed content and retains unrelated user hook commands.
6. Hook files remain executable after installation or partial removal.
7. Missing gitleaks produces installation guidance rather than a false pass.

## Behavioral Examples

```
Given a pre-commit hook containing user commands
When install-hook and then uninstall-hook run
Then the staged gitleaks check is added once and later removed while the user commands remain
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
| 2026-07-13 | CHG-0001-adopt-specsync-5-0-1-and-trust-1-0-0-governance-for-the-gitleaks-fledge-plugin: Adopt SpecSync 5.0.1 and Trust 1.0.0 governance for the Gitleaks Fledge plugin |
