# fledge-plugin-secrets

Scan your repo for committed secrets — wraps [`gitleaks`](https://github.com/gitleaks/gitleaks) with a friendlier interface and a one-command pre-commit hook installer.

A plugin for [fledge](https://github.com/CorvidLabs/fledge).

## Prerequisites

Install gitleaks:

```bash
brew install gitleaks               # macOS
# or: https://github.com/gitleaks/gitleaks/releases
```

## Install

```bash
fledge plugins install CorvidLabs/fledge-plugin-secrets
```

## Usage

```bash
fledge secrets scan               # scan working tree, pretty-print findings
fledge secrets scan --staged      # scan only staged changes (fast pre-commit)
fledge secrets check              # exits non-zero if any findings (CI mode)
fledge secrets install-hook       # install a pre-commit hook calling `check --staged`
fledge secrets uninstall-hook     # remove the pre-commit hook
```

The plugin honors any `.gitleaks.toml` you have in the repo. To allowlist false positives, edit that file (see [gitleaks docs](https://github.com/gitleaks/gitleaks#configuration)).

## License

MIT
