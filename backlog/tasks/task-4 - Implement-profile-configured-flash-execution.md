---
id: TASK-4
title: Implement profile-configured flash execution
status: Done
assignee: []
created_date: '2026-08-29 07:37'
updated_date: '2026-08-29 07:56'
labels:
  - cli
  - core
  - flash
dependencies:
  - TASK-5
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implement the remaining `nxus flash` command path using a structured flash command configured per profile in `nxus.toml`. Extend the core configuration schema, resolve the selected profile's flash command, expand a limited set of Nxus-managed placeholders for project and build artifacts, reuse the existing runner abstraction, and ensure required artifacts are present before execution.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Profile configuration supports an optional structured flash command with separate command and args fields
- [x] #2 `nxus flash` follows the normal selected profile behavior and reports an actionable error when flash is not configured
- [x] #3 Supported placeholders for project workspace build profile and artifacts are expanded by Nxus before execution
- [x] #4 Unknown placeholders and missing requested artifact files return clear errors
- [x] #5 Flash reuses existing configure or build behavior to ensure artifacts are available before running the configured programmer
- [x] #6 Automated tests cover deserialization placeholder expansion missing artifacts dry-run command construction and runner error propagation
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Extend the core config schema with an optional structured per-profile flash command type that maps naturally onto the existing `Cmd` abstraction.
2. Add focused flash resolution helpers in `nxus-core` to expand documented placeholders for project/workspace/build/profile and expected NuttX artifacts, with explicit errors for unknown placeholders and missing files.
3. Implement `nxus flash` in `nxus-shell` by reusing the existing build/config flow to ensure artifacts exist before executing the structured flash command through `Runner`.
4. Add tests for deserialization, placeholder expansion, artifact lookup, dry-run command construction, and runner error propagation.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
This task was left behind by an earlier Backlog tool duplication during task creation. Its scope was completed as part of the implemented flash work tracked in TASK-7.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Closed duplicate flash task created during Backlog task setup. The implemented work lives in the finished flash command changes and the detailed execution record in TASK-7.
<!-- SECTION:FINAL_SUMMARY:END -->
