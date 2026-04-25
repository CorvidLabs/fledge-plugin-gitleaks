# fledge-plugin-gitleaks

A thin fledge wrapper around [`gitleaks`](https://github.com/gitleaks/gitleaks) — pretty-printed scans + a one-command pre-commit hook installer.

A plugin for [fledge](https://github.com/CorvidLabs/fledge).

> Looking for a self-contained Kotlin scanner with built-in pattern catalog and entropy detection? See [`CorvidLabs/fledge-plugin-secrets`](https://github.com/CorvidLabs/fledge-plugin-secrets). This plugin is the gitleaks-backed alternative — pick whichever fits your stack.

## Prerequisites

Install gitleaks:

```bash
brew install gitleaks               # macOS
# or: https://github.com/gitleaks/gitleaks/releases
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

The plugin honors any `.gitleaks.toml` you have in the repo. To allowlist false positives, edit that file (see [gitleaks docs](https://github.com/gitleaks/gitleaks#configuration)).

## License

MIT
