---
change: CHG-0002-correct-the-migration-contract-and-portable-verification-guidance
artifact: design
---

# Design

Narrow the hook contract to the implementation's actual ownership boundary: unmanaged hooks are preserved through refusal, while a marker-bearing managed hook is removed as a complete file. Keep installation idempotent when the marker is already present.

Use the same full-input create-spec guidance for Claude, Cursor, and Gemini, and make Gemini create-change shell-escape the displayed raw arguments once. Replace `site/` with `docs/` in lifecycle policy and invoke the release help smoke through `cargo run` so Cargo selects the correct executable name on every supported runner.
