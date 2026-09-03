---
id: TASK-9
title: Implement project-defined exec commands
status: Done
assignee:
  - '@OpenCode'
created_date: '2026-09-03 13:11'
updated_date: '2026-09-03 16:45'
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
1. Inspect the current `exec` command path, config discovery/loading APIs, and existing command-resolution tests to find the smallest place to reserve `list` without changing `nxus exec <command>` parsing or execution semantics.
2. Add a small metadata reader in `nxus-core` that reads the discovered `nxus.toml`, uses `toml_edit` only for optional presentation metadata, and maps directly-adjacent single-line comments onto the already-authoritative configured command names from the resolved config.
3. Implement `nxus exec list` in `nxus-shell` as a reserved built-in operation on the existing positional command model, formatting deterministic output from configured command order while ensuring no command execution, placeholder expansion, or build/tool invocation occurs.
4. Add focused tests for adjacent-comment extraction rules, multi-command metadata mapping, reserved `list` behavior, parser/runtime regressions for normal `exec` usage, and end-to-end CLI listing output with no artifact or tool requirements.
5. Update the relevant CLI/docs/template examples to document `nxus exec list`, the single-line adjacent-comment convention, blank-line behavior, and the reserved `list` name, then run the repository validation flow (`just fmt`, focused tests as needed, `just test`, and final `just ci`).
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Extended `NxusConfig` and `ResolvedConfig` with ordered project-level command maps using the existing `CommandConfig`, added config load and overlay coverage for `[command.<name>]`, and kept default configs empty for custom commands.

Moved placeholder expansion and artifact validation into a new generic `resolve_command` path in `nxus-core`, renamed the placeholder and missing-artifact errors to generic command-oriented variants, and reduced `resolve_flash_command` to flash-specific policy plus reuse of the generic resolver.

Added `nxus exec <name>` in `nxus-shell` with raw trailing argv captured after `--` as `OsString` values, appended runtime args after configured args, and executed through the existing `Runner` without any implicit build step or shell invocation.

Updated the generated `nxus.toml` template, repo example config, CLI help examples, and README documentation to describe `[command.<name>]`, placeholder support, raw `--` forwarding, profile selection behavior, no implicit build semantics, and using CMake targets as regular configured commands.

Validation ran cleanly with `just fmt`, `just index`, focused `cargo test` runs during development, `just test`, `just doctest`, and a final `just ci`. Coverage from the final CI flow was 91.21% line coverage overall, with 123/123 tests passing and zero warnings or errors in `just ci`.

Reopened the existing project-defined exec command task to cover the follow-up `nxus exec list` discovery feature instead of creating a duplicate task.

Added `nxus exec list` as a reserved built-in on top of the existing positional `exec` command model so normal `nxus exec <name>` parsing and execution semantics remain unchanged.

Added a small `nxus-core` `command_info` reader backed by `toml_edit` that reads only adjacent single-line `#` comments from `nxus.toml` and maps them onto the already-authoritative configured command names from the resolved config.

Covered direct-adjacency comment extraction rules, undocumented commands, preserved command order, reserved `list` behavior, parser/runtime regressions, and end-to-end listing output without requiring build artifacts or external tools.

Validation passed with `just fmt`, focused `just test -- command_info`, focused `just test -- exec`, full `just test`, and final `just ci`. Final coverage from `just ci` was 92.37% region coverage and 91.71% line coverage overall with zero warnings/errors.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Implemented project-defined `nxus exec` support backed by `[command.<name>]` entries in `nxus.toml`. The top-level config carries an ordered map of named commands using the existing `CommandConfig`, the resolved project context exposes those commands for the selected/default profile, and a generic `resolve_command` function in `nxus-core` handles placeholder expansion, artifact validation, and `Cmd` construction for both `exec` and `flash`.

In `nxus-shell`, `nxus exec <name>` preserves raw arguments after `--` as separate argv entries and appends them after the configured arguments before invoking the shared `Runner`. Unknown project commands, unknown placeholders, and missing artifacts report clear command-oriented errors. `nxus exec` does not trigger an implicit build, while existing flash build and rebuild behavior remains unchanged.

Added `nxus exec list` as a reserved built-in discovery operation on the existing `exec` interface. Listing uses a new `nxus-core::command_info` helper that reads `nxus.toml` with `toml_edit`, inspects the table-header prefix decoration for `[command.<name>]`, and extracts only the single physical `#` comment line immediately above each command table as an optional description. Blank lines break the association, multiline comments are not joined, malformed metadata falls back to no description, and configured command names still come from the already-loaded Nxus config. A configured `[command.list]` entry is still shown in listings, but `nxus exec list` always performs listing instead of executing it.

Updated the project template config, checked-in example config, CLI examples, and README documentation to cover `nxus exec list`, the adjacent single-line comment convention, the reserved `list` name, and the fact that undocumented commands are still listed. Added focused unit, CLI, and integration coverage for comment extraction, metadata mapping, reserved-name behavior, and regressions for normal custom-command execution. Validation finished cleanly with `just test` and `just ci`; the final coverage report from `just ci` showed 91.71% overall line coverage with 137/137 tests passing and zero warnings or errors.
<!-- SECTION:FINAL_SUMMARY:END -->
