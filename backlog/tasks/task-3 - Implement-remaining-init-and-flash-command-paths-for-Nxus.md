---
id: TASK-3
title: Implement remaining init and flash command paths for Nxus
status: Done
assignee: []
created_date: '2026-08-29 07:37'
updated_date: '2026-08-29 07:56'
labels:
  - cli
  - core
  - init
  - flash
dependencies: []
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Finish the currently stubbed `nxus init` and `nxus flash` command paths by following the existing `nxus-shell` and `nxus-core` architecture. This includes supporting pre-project init bootstrap behavior, conservative project/config scaffolding, profile-configured flash execution with placeholder expansion, automated tests, documentation updates, and final validation with the repository's standard `just` workflows.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 `nxus init` is implemented as a subcommand group with `config` and `project [PATH]` flows that work without requiring an existing `nxus.toml`
- [x] #2 `nxus flash` executes a profile-configured structured command with documented Nxus placeholder expansion and actionable errors
- [x] #3 Behavioral tests cover init safety rules and flash resolution or execution semantics without requiring external hardware
- [x] #4 User-facing help text and relevant documentation describe the new init and flash behavior
- [x] #5 Repository formatting linting and test workflows pass including `just ci`
<!-- AC:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Completed the remaining `init` and `flash` command paths for Nxus. `init` is now a pre-project command group with conservative config-only and full-project scaffolding, `flash` is now driven by per-profile structured command configuration with placeholder expansion and artifact checks, the CLI help and README were updated, and the repository validation gates passed including `just ci` and >90% line coverage overall.
<!-- SECTION:FINAL_SUMMARY:END -->
