---
id: TASK-1
title: Add meaningful unit tests for nxus-core and nxus-shell
status: Done
assignee:
  - OpenCode
created_date: '2026-08-28 09:39'
updated_date: '2026-08-28 09:50'
labels:
  - tests
  - coverage
dependencies: []
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Increase unit test coverage across the `crates/nxus-core` and `crates/nxus-shell` modules, using the existing nxus-core tests as the style baseline. Focus on meaningful behavior and edge cases rather than shallow line hits. Prefer repo `just` recipes for validation, and finish with the `just ci` quality gate plus a backlog status check.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Unit tests cover meaningful success and failure paths in the key nxus-core modules including config resolution path helpers command execution helpers and workspace helpers
- [x] #2 Unit tests cover meaningful parsing and command behavior in nxus-shell using established naming and coding style conventions
- [x] #3 The resulting coverage for the nxus-core and nxus-shell modules reaches at least 85 percent lines in the coverage report
- [x] #4 Validation is run with repo just recipes including just ci and any failing issues introduced by the test changes are fixed
- [x] #5 The backlog task record is updated with the implementation plan progress notes and final summary and no related task is left unfinished
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Expand `nxus-core` unit coverage first because it has existing inline test patterns to follow. Target `config::resolution`, `paths`, `exec`, and `workspace` with tempdir-driven and dry-run-focused tests that exercise success cases plus edge/error conditions without depending on external NuttX tooling.
2. Add focused `nxus-shell` tests next. Start with `Cli::try_parse_from` coverage for flags, aliases, and subcommands, then add command-handler tests where behavior can be exercised safely through dry-run runners and temp directories.
3. Use the repo coverage recipe to measure progress and close gaps until `nxus-core` and `nxus-shell` are at or above 85 percent line coverage. Keep additions minimal and aligned with current naming/style.
4. Run `just ci`, fix any formatting/lint/test issues, then record final coverage/check results and complete the backlog task.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Added shared test helpers in both crates and implemented focused unit tests for core config resolution path helpers command execution workspace behavior and shell CLI and command handlers using temp directories plus dry-run runners. Starting validation and coverage checks now.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Added a broad unit-test pass across `nxus-core` and `nxus-shell` with shared test helpers, tempdir-based filesystem fixtures, and dry-run command execution checks. Coverage now exceeds the requested threshold for the targeted crates, and `just ci` passes cleanly. The new tests focus on meaningful behavior and edge cases in config discovery/loading/resolution, path helpers, command execution, workspace setup, CLI parsing, and shell command handlers.
<!-- SECTION:FINAL_SUMMARY:END -->
