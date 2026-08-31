---
id: TASK-5
title: Add init config and project scaffolding
status: Done
assignee:
  - OpenCode
created_date: '2026-08-29 07:38'
updated_date: '2026-08-29 07:56'
labels:
  - cli
  - core
  - init
dependencies: []
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implement the remaining `nxus init` command path as a subcommand group with `config` and `project [PATH]`. Refactor shell bootstrap so init is treated as a pre-project command that runs before discovery or resolved-config creation. Add conservative core scaffolding helpers that create the minimal canonical Nxus project structure and supporting config files without overwriting user data.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 CLI parsing supports `nxus init config`, `nxus init project`, and `nxus init project <path>`
- [x] #2 Init commands work without requiring an existing `nxus.toml` to be discoverable
- [x] #3 Config initialization writes a minimal parseable `nxus.toml` and the project-owned config files or directories required by the current Nxus layout
- [x] #4 Project initialization creates the minimal canonical project scaffold in a new or empty destination directory
- [x] #5 Init refuses to overwrite conflicting files or destructively initialize unsafe destinations with useful errors
- [x] #6 Automated tests cover parsing generated config loading safe initialization and refusal cases
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Refactor `nxus-shell` CLI parsing and dispatch so `init` is a pre-project command that runs before config discovery and resolved-config creation.
2. Inspect the current Nxus project layout assumptions from path helpers and workspace/config generation, then add focused `nxus-core` initialization helpers that create the minimal canonical config-only and full-project scaffolds.
3. Preserve Nxus defaults by generating a minimal `nxus.toml` with explicit project-owned values that matter, especially workspace revisions, while refusing to overwrite conflicting existing files or unsafe non-empty destinations.
4. Add shell/core tests for init parsing, generated config loading, safe directory handling, and overwrite refusal cases.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented pre-project CLI dispatch for `init`, added conservative `nxus-core` scaffolding helpers for config-only and full-project initialization, and aligned the scaffold with the current app-root-under-project layout used by discovery and config path resolution.

Added shell, core, and integration tests covering CLI parsing, generated config loading, current-directory config init, project scaffolding into new and empty destinations, and refusal to overwrite or initialize unsafe directories.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Implemented `nxus init` as a pre-project command group with `config` and `project [PATH]` subcommands. The shell bootstrap now dispatches `init` before config discovery, while the rest of the CLI continues to use the normal discovery/load/resolve path. In `nxus-core`, added conservative scaffolding helpers that write a minimal parseable `nxus.toml`, default config overlays, and the canonical app-root project scaffold without overwriting existing files or non-empty destinations. Added shell/core/integration coverage for parsing, safe filesystem behavior, and generated-config loading.
<!-- SECTION:FINAL_SUMMARY:END -->
