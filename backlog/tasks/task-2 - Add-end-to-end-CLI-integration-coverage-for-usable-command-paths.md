---
id: TASK-2
title: Add end-to-end CLI integration coverage for usable command paths
status: Done
assignee:
  - OpenCode
created_date: '2026-08-28 10:22'
updated_date: '2026-08-28 10:28'
labels: []
dependencies: []
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Add integration tests that exercise the installable `nxus` binary across every currently usable command path and representative CLI invocations. Focus on meaningful behavior and edge cases around config discovery, config loading, profile resolution, command aliases, and dry-run command execution. Use the integration suite to improve overall line coverage toward the requested 90% target without adding shallow tests for currently unreachable placeholder commands.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Integration tests cover successful binary invocations for each currently routed command path including workspace subcommands
- [x] #2 Integration tests cover key failure paths for missing config invalid config and unknown profile selection
- [x] #3 Integration tests cover representative CLI aliases and global flag combinations at the binary level
- [x] #4 Tests use realistic temporary project fixtures and dry-run safe command paths rather than mocked command internals
- [x] #5 The resulting suite improves overall line coverage toward 90 percent while keeping code readable and aligned with repo style
- [x] #6 Validation includes the just ci quality gate and backlog is updated with final status notes
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Add reusable integration-test helpers in `tests/integration_tests.rs` to create temporary projects, write `nxus.toml`, create nested invocation directories, and prepare minimal filesystem state for dry-run-safe command execution.
2. Cover binary-level success paths for all currently routed commands: `profiles`, `clean`, `config`, `build`, `menuconfig`, `run`, `sim`, `test`, `workspace clean`, `workspace init`, and `workspace prune`.
3. Cover binary-level failure paths for missing configuration, invalid TOML, unknown profile selection, and representative path-shape errors that users can realistically trigger.
4. Exercise representative command aliases and global flags through the compiled binary so clap parsing and command dispatch are verified end to end.
5. Run targeted tests first, then `just ci`, inspect coverage output, and add only meaningful extra coverage if still needed.
6. Finalize the backlog task with implementation notes, checked acceptance criteria, validation results, and completion summary.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Implemented a reusable binary-level integration fixture in `tests/integration_tests.rs` covering routed command success paths, config-discovery failures, profile resolution failures, and representative alias/global-flag invocations.

First `just ci` run failed only on rustfmt changes in `tests/integration_tests.rs`; no lint or test failures were reached before formatting stopped the gate.

Validation passed with `cargo test --test integration_tests` and `just ci`. The coverage summary from `cargo llvm-cov nextest --summary-only` reports 91.01 percent total line coverage and 92.02 percent region coverage.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Added a full binary-level integration suite in `tests/integration_tests.rs` for every CLI command path currently routed by `nxus::run()`, including `profiles`, `clean`, `config`, `build`, `menuconfig`, `run`, `sim`, `test`, and the `workspace` subcommands. The new fixture builds realistic temporary project layouts, exercises upward config discovery, creates dry-run-safe workspace/build state, and verifies both successful command dispatch and user-facing failures such as missing config, invalid TOML, unknown profiles, and invalid build-path shape.

Also added the root `tempfile` dev-dependency needed by the integration fixture.

Validation completed with `cargo test --test integration_tests` and `just ci`. The final coverage summary from `cargo llvm-cov nextest --summary-only` reports 91.01 percent total line coverage, exceeding the requested 90 percent target.
<!-- SECTION:FINAL_SUMMARY:END -->
