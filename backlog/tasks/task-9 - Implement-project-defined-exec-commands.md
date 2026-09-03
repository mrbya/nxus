---
id: TASK-9
title: Implement project-defined exec commands
status: Done
assignee:
  - '@OpenCode'
created_date: '2026-09-03 13:11'
updated_date: '2026-09-03 13:22'
labels:
  - cli
  - core
  - config
  - docs
dependencies: []
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Add a small project-level custom command mechanism driven by `[command.<name>]` entries in `nxus.toml` and executed via `nxus exec <name> [-- <args...>]`. Reuse the existing `CommandConfig`, config loading/resolution pipeline, placeholder expansion, `Cmd`/`Runner`, and flash command behavior. Scope includes schema and resolved-config support, a generic configured-command resolver reused by flash, the new shell `exec` command path with raw trailing argv forwarding, user-facing docs/template updates, regression protection for flash, and final validation with the repository `just` workflows.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Top-level config supports ordered named commands loaded from `[command.<name>]` using the existing `CommandConfig` with correct overlay replacement semantics
- [x] #2 Resolved project context exposes named commands so `nxus exec` uses the selected or default profile without reparsing config
- [x] #3 Generic configured-command resolution expands the existing placeholder set for program and configured args, preserves argv boundaries, validates required artifacts, and is reused by flash
- [x] #4 `nxus exec <name>` and `nxus exec <name> -- <args...>` parse and execute through the existing `Runner` without shell invocation and without implicit build behavior
- [x] #5 Unknown project commands, unknown placeholders, and missing artifacts return clear errors while flash semantics remain unchanged
- [x] #6 Docs, generated template config, unit tests, integration tests, coverage, and final `just ci` validation all pass cleanly for the new feature and flash regressions are covered
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Extend `NxusConfig` with an ordered `commands` map using the existing `CommandConfig`, keep defaults empty, and update overlay and config-load tests so later layers replace whole command definitions while profile flash behavior stays intact.
2. Move placeholder expansion and structured command-to-`Cmd` resolution out of `flash.rs` into a generic core module or API, expose named commands on `ResolvedConfig`, and keep flash-specific policy limited to "flash not configured" and existing build/rebuild behavior.
3. Add a new `exec` shell command with Clap parsing that captures a command name plus raw trailing argv after `--`, looks up the configured project command from resolved config, appends runtime args after expanded configured args, and executes via `Runner` without implicit builds.
4. Update the generated `nxus.toml` template, root config example/comments, README, and CLI help to document `[command.<name>]`, placeholder support, `nxus exec <name> -- <args...>`, profile selection, artifact requirements, and the fact that CMake targets are just regular configured commands.
5. Add focused unit and integration coverage for config deserialization and overlay, generic command resolution, exec CLI parsing, dry-run runtime behavior, profile-sensitive placeholder expansion, missing command or artifact failures, and flash regression behavior.
6. Run the repository validation flow with the established `just` recipes, fix any issues, confirm coverage remains above 90%, and finalize the Backlog record with notes, checked acceptance criteria, and a completion summary.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Extended `NxusConfig` and `ResolvedConfig` with ordered project-level command maps using the existing `CommandConfig`, added config load and overlay coverage for `[command.<name>]`, and kept default configs empty for custom commands.

Moved placeholder expansion and artifact validation into a new generic `resolve_command` path in `nxus-core`, renamed the placeholder and missing-artifact errors to generic command-oriented variants, and reduced `resolve_flash_command` to flash-specific policy plus reuse of the generic resolver.

Added `nxus exec <name>` in `nxus-shell` with raw trailing argv captured after `--` as `OsString` values, appended runtime args after configured args, and executed through the existing `Runner` without any implicit build step or shell invocation.

Updated the generated `nxus.toml` template, repo example config, CLI help examples, and README documentation to describe `[command.<name>]`, placeholder support, raw `--` forwarding, profile selection behavior, no implicit build semantics, and using CMake targets as regular configured commands.

Validation ran cleanly with `just fmt`, `just index`, focused `cargo test` runs during development, `just test`, `just doctest`, and a final `just ci`. Coverage from the final CI flow was 91.21% line coverage overall, with 123/123 tests passing and zero warnings or errors in `just ci`.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Implemented project-defined `nxus exec` support backed by `[command.<name>]` entries in `nxus.toml`. The top-level config now carries an ordered map of named commands using the existing `CommandConfig`, the resolved project context exposes those commands for the selected/default profile, and a new generic `resolve_command` function in `nxus-core` handles placeholder expansion, artifact validation, and `Cmd` construction for both `exec` and `flash`.

In `nxus-shell`, added the `exec` subcommand with Clap parsing that preserves raw arguments after `--` as separate argv entries and appends them after the configured arguments before invoking the shared `Runner`. Unknown project commands, unknown placeholders, and missing artifacts now report clear command-oriented errors. `nxus exec` does not trigger an implicit build, while existing flash build and rebuild behavior remains unchanged.

Updated the project template config, checked-in example config, CLI examples, and README documentation to cover command configuration, supported placeholders, profile selection, raw argument forwarding, artifact requirements, and using CMake targets as ordinary configured commands. Validation finished cleanly with `just test`, `just doctest`, and `just ci`; the final coverage report showed 91.21% overall line coverage with 123/123 tests passing.
<!-- SECTION:FINAL_SUMMARY:END -->
