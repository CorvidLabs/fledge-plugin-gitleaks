## MODIFIED

### REQUIREMENT REQ-gitleaks-004

Hook installation and removal SHALL preserve unmanaged hooks by refusing to overwrite or delete any hook that lacks the plugin's managed marker.

Acceptance Criteria

- Installation creates the marker-bearing staged-check hook only when no hook exists, and repeated installation recognizes the managed marker.
- Installation refuses an existing hook without the managed marker.
- Removal deletes the complete hook file when the managed marker is present.
- Removal refuses a hook without the managed marker.
