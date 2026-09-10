---
id: TASK-7
title: Implement flash execution from profile command config
status: Done
assignee:
  - OpenCode
created_date: '2026-08-29 07:39'
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
Add the remaining `nxus flash` command path using per-profile structured command configuration in `nxus.toml`. Extend the core schema, resolve and expand supported Nxus placeholders for paths and build artifacts, ensure the selected profile has the required outputs, and execute the structured command through the existing runner abstraction without using shell quoting.
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
1. Extend the core config schema with an optional structured per-profile flash command type that maps onto the existing `Cmd` abstraction.
2. Add flash resolution helpers in `nxus-core` to expand supported placeholders for project/workspace/build/profile and expected NuttX artifacts, with explicit errors for unknown placeholders and missing files.
3. Implement `nxus flash` in `nxus-shell` by reusing the existing build path to ensure artifacts are available before executing the structured flash command through `Runner`.
4. Add tests for deserialization, placeholder expansion, artifact lookup, dry-run command construction, and runner error propagation.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Extended `ProfileConfig` and `ResolvedConfig` with optional structured flash command data, added standard artifact path helpers, and implemented placeholder expansion with explicit errors for missing flash config, unknown placeholders, and missing artifacts.

Implemented `nxus flash` through the existing build runner path so argument boundaries stay intact and dry-run or runner failures are surfaced through the existing execution abstraction. Added unit and integration tests for deserialization, placeholder expansion, dry-run command construction, and runner failure propagation.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Implemented profile-configured `nxus flash` support. Profiles can now define a structured `[profile.<name>.flash]` command with separate arguments, and `nxus-core` expands supported placeholders for project paths and standard NuttX artifacts before building a `Cmd`. The shell flash command reuses the existing build flow, errors cleanly when flash is not configured or an artifact is missing, and executes through the existing `Runner` without shell quoting. Added targeted unit and integration coverage for parsing, expansion, dry-run behavior, and runner error propagation.
<!-- SECTION:FINAL_SUMMARY:END -->
