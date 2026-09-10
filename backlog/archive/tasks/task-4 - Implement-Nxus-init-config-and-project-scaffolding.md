---
id: TASK-4
title: Implement Nxus init config and project scaffolding
status: To Do
assignee: []
created_date: '2026-08-29 07:37'
labels:
  - cli
  - core
  - init
dependencies: []
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Add the `nxus init config` and `nxus init project [PATH]` command paths using the existing architecture. Refactor bootstrap only as needed so init runs as a pre-project command without constructing fake resolved configs. Reuse core abstractions for config defaults, path handling, and errors where practical. Scaffolding must be conservative around existing files and should create the canonical minimal Nxus project layout and config files expected by the current build flow.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 CLI parsing supports `nxus init config`, `nxus init project`, and `nxus init project <path>`
- [ ] #2 Init commands can run before project discovery and do not require an existing `nxus.toml`
- [ ] #3 Config initialization creates a minimal parseable `nxus.toml` plus project-owned config files or directories required by the current Nxus layout
- [ ] #4 Project initialization creates the minimal canonical project scaffold in a new or empty destination directory
- [ ] #5 Init refuses to overwrite conflicting existing files or destructively initialize unsafe destinations with useful errors
- [ ] #6 Automated tests cover parsing generated config loading safe initialization and refusal cases
<!-- AC:END -->
