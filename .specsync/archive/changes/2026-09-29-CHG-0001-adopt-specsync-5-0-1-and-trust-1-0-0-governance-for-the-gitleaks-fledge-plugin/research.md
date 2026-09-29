---
change: CHG-0001-adopt-specsync-5-0-1-and-trust-1-0-0-governance-for-the-gitleaks-fledge-plugin
artifact: research
---

# Research

The existing CI runs tests and release builds on Linux, macOS, and Windows, plus blocking Rust formatting and Clippy checks on Linux. The release workflow packages multiple architectures and must remain independent.

All implementation is in `src/main.rs`, so SpecSync can enforce 100% companion coverage. Pages remains the repository's independent publication path, so Trust-managed Atlas publication stays disabled.
