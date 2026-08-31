---
id: TASK-8
title: Restore rebuild test compatibility and coverage
status: Done
assignee:
  - OpenCode
created_date: '2026-08-31 12:53'
updated_date: '2026-08-31 13:00'
labels:
  - tests
  - cli
  - config
dependencies: []
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Update Nxus test helpers and existing Rust tests to match the current ResolvedConfig shape and ResolvedConfig::resolve(...) signature after the rebuild flag addition, then add meaningful coverage for rebuild parsing and run/flash rebuild behavior without broadening scope into unrelated production changes.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Shared test helpers construct ResolvedConfig with the current fields including rebuild defaulting to false
- [x] #2 All direct ResolvedConfig constructions and ResolvedConfig::resolve(...) call sites in the workspace are updated to the current API with no stale test usages remaining
- [x] #3 Resolver tests verify rebuild propagates as false and true while preserving existing assertions for profile and path behavior
- [x] #4 CLI parser tests cover --rebuild, -r, default rebuild false, and distinguish -r from the run alias
- [x] #5 Run and flash tests verify existing-artifact behavior with and without rebuild using observable dry-run or equivalent command assertions
- [x] #6 Project validation passes including just test, just ci, and the repository coverage workflow with coverage above 90 percent
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Inspect the current ResolvedConfig definition and resolve signature in nxus-core, plus the CLI/global flag handling in nxus-shell.
2. Search the workspace for all direct ResolvedConfig constructions and ResolvedConfig::resolve(...) invocations, prioritizing shared test helpers in crates/nxus-core/src/tests.rs and crates/nxus-shell/src/tests.rs.
3. Update shared helpers to default rebuild to false, then fix any remaining stale constructors and resolver call sites with minimal edits that preserve existing test intent.
4. Add or extend resolver tests to assert rebuild=false and rebuild=true propagation without weakening the existing path/profile assertions.
5. Add CLI parser tests for --rebuild, -r, default rebuild=false, and parsing of -r alongside the run alias.
6. Add behavioral tests for run and flash rebuild semantics using the existing test style and observable dry-run command output, including artifact-present branches with and without rebuild and preserving missing-artifact behavior.
7. Add lightweight propagation coverage for sim or test only if the current implementation delegates through run in a way that makes the rebuild flag worth asserting.
8. Run focused tests while iterating, then run just test, just ci, and the repository coverage workflow; update the task notes and acceptance criteria as results are confirmed.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Audited all workspace `ResolvedConfig { ... }` constructions and `ResolvedConfig::resolve(...)` invocations. The only stale direct constructors were the shared helpers in `crates/nxus-core/src/tests.rs` and `crates/nxus-shell/src/tests.rs`; the only stale resolver call sites were the resolver unit tests in `crates/nxus-core/src/config/resolution.rs`.

Added CLI parser coverage for `--rebuild`, `-r`, default `rebuild = false`, and `nxus -r r` parsing to prove Clap distinguishes the global flag from the `run` alias.

Strengthened integration coverage to observe dry-run command emission and ordering for missing-artifact auto-build, artifact-present run without rebuild, artifact-present run with rebuild, artifact-present flash without rebuild, artifact-present flash with rebuild, and `sim` delegation preserving rebuild semantics.

`just ci` initially failed because the added `rebuild` field pushed `ResolvedConfig` over Clippy's `struct_excessive_bools` threshold. Fixed this narrowly by replacing `profile_selected: bool` with `ProfileSelection::{Default, Explicit}` and updating the small set of affected call sites without changing behavior.

Validation completed successfully: focused `cargo test` runs for resolver, CLI parser, and integration tests; `just test`; `just ci`; coverage summary 92.51% regions / 92.44% lines.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Restored the test suite to the current `ResolvedConfig` API by updating the shared nxus-core and nxus-shell test helpers to include `rebuild: false`, correcting every stale `ResolvedConfig::resolve(...)` test call, and tightening resolver assertions so rebuild propagation is explicitly verified for both false and true cases.

Extended CLI parser coverage in `crates/nxus-shell/src/cli.rs` for `--rebuild`, `-r`, default rebuild behavior, and the `nxus -r r` ambiguity case so the global flag and `run` alias are both exercised.

Expanded integration coverage in `tests/integration_tests.rs` to observe dry-run command output for rebuild semantics: missing binaries still auto-build, existing binaries skip rebuild by default, `--rebuild` forces build-before-run and build-before-flash, and `nxus --rebuild sim` retains rebuild behavior through delegated profile switching. Assertions also verify relative command ordering where relevant.

A narrow production-only adjustment was required to satisfy `just ci`: replacing `ResolvedConfig.profile_selected: bool` with a small `ProfileSelection` enum kept behavior unchanged while resolving Clippy's `struct_excessive_bools` error introduced by the new rebuild flag.

Validation run: focused `cargo test` for resolver/parser/integration areas, `just test`, and `just ci` including fmt, clippy with `-D warnings`, udeps, audit, doctests, and coverage generation. Final coverage: 92.51% regions and 92.44% lines.
<!-- SECTION:FINAL_SUMMARY:END -->
