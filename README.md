# fledge-plugin-gitleaks

[![CI](https://github.com/CorvidLabs/fledge-plugin-gitleaks/actions/workflows/ci.yml/badge.svg)](https://github.com/CorvidLabs/fledge-plugin-gitleaks/actions/workflows/ci.yml)

A thin fledge wrapper around [`gitleaks`](https://github.com/gitleaks/gitleaks) — pretty-printed scans + a one-command pre-commit hook installer.

A plugin for [fledge](https://github.com/CorvidLabs/fledge).

> The earlier Kotlin scanner [`CorvidLabs/fledge-plugin-secrets`](https://github.com/CorvidLabs/fledge-plugin-secrets) is now archived; this gitleaks-backed plugin is its successor.

## Prerequisites

- [fledge](https://github.com/CorvidLabs/fledge) installed and available in your PATH
- [gitleaks](https://github.com/gitleaks/gitleaks) installed:

```bash
brew install gitleaks               # macOS
sudo apt-get install gitleaks       # Debian/Ubuntu (if available)
# or grab a binary: https://github.com/gitleaks/gitleaks/releases
```

## Install

```bash
fledge plugins install CorvidLabs/fledge-plugin-gitleaks
```

## Usage

```bash
fledge gitleaks scan               # scan working tree, pretty-print findings
fledge gitleaks scan --staged      # scan only staged changes (fast, pre-commit)
fledge gitleaks check              # exits non-zero if any findings (CI mode)
fledge gitleaks install-hook       # install a pre-commit hook calling `check --staged`
fledge gitleaks uninstall-hook     # remove the pre-commit hook
```

### CI Integration

Use `fledge gitleaks check` in your CI pipeline. It exits non-zero when secrets are detected:

```yaml
- run: fledge gitleaks check
```

### Configuration

The plugin honors any `.gitleaks.toml` you have in the repo. To allowlist false positives, edit that file (see [gitleaks docs](https://github.com/gitleaks/gitleaks#configuration)).

## Development

```bash
cargo build --release
cargo test
cargo clippy -- -D warnings
cargo fmt --check
```

The built binary lives at `target/release/fledge-gitleaks`.

## License

MIT
