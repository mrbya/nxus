---
id: TASK-12
title: Make overlay_root affect generated configuration
status: To Do
assignee: []
created_date: '2026-09-09 05:57'
labels:
  - bug
  - config
  - documentation
dependencies: []
references:
  - crates/nxus-core/src/config/resolution.rs
  - crates/nxus-core/src/paths.rs
  - crates/nxus-core/src/workspace.rs
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The project configuration schema resolves `overlay_root`, but configuration generation currently reads common and profile overlay files from `config/` under the invocation working directory. Clarify the intended behavior and either apply `overlay_root` consistently or remove the unsupported setting.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 The configured overlay-root behavior is implemented consistently or the unsupported option is removed
- [ ] #2 User-facing configuration documentation reflects the resulting behavior
- [ ] #3 Relevant tests cover configured overlay location behavior
<!-- AC:END -->
