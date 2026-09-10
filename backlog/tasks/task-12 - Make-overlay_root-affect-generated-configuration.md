---
id: TASK-12
title: Make overlay_root affect generated configuration
status: Done
assignee:
  - '@OpenCode'
created_date: '2026-09-09 05:57'
updated_date: '2026-09-10 06:40'
labels:
  - bug
  - config
  - documentation
dependencies: []
references:
  - crates/nxus-core/src/config/resolution.rs
  - crates/nxus-core/src/paths.rs
  - crates/nxus-core/src/workspace.rs
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The project configuration schema resolves `overlay_root`, but configuration generation currently reads common and profile overlay files from `config/` under the invocation working directory. Clarify the intended behavior and either apply `overlay_root` consistently or remove the unsupported setting.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 The configured overlay-root behavior is implemented consistently or the unsupported option is removed
- [x] #2 User-facing configuration documentation reflects the resulting behavior
- [x] #3 Relevant tests cover configured overlay location behavior
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Audit the current config resolution, paths, workspace/configuration generation, CLI handlers, template, and tests to verify resolved-root and command behavior. 2. Update the existing README task's documentation scope in place: correct path-root precedence and semantics, generated configuration/build/workspace layouts, custom commands, and stale template/changelog content without changing product code. 3. Regenerate the README TOC, audit every user-facing placeholder and path example, then run the established test and CI recipes including coverage. 4. Record validation results, complete all applicable acceptance criteria, and finalize this existing task. The user explicitly requested this final documentation pass, which approves this plan.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Audit verified that `ResolvedConfig` applies `NXUS_WORKSPACE`, `NXUS_BUILD_ROOT`, and `NXUS_OVERLAY_ROOT` before config/default values; relative values resolve from `project_dir`. `workspace::generate_config` consumes the resolved overlay root and writes generated defconfig under the resolved workspace root. Integration tests cover relative and absolute environment overrides.

Updated the README to remove the obsolete overlay-root caveat and document resolved workspace/build/overlay roots, root precedence, absolute and relative behavior, generated configuration order, compile database locations, workspace commands, custom command execution, and `exec list`. Regenerated the README TOC. `just test` passed 140/140 tests; `just ci` passed with zero warnings/errors and 91.68% overall line coverage.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Completed the final documentation and release-readiness audit against the current implementation. The README now documents environment precedence and project-dir path resolution for all roots; resolved workspace/build/overlay layouts; generated defconfig inputs and order; compile database linking; workspace lifecycle; structured commands, cwd, all supported double-brace placeholders, descriptions, shell behavior, and reserved `exec list`. The template configuration and active example use current placeholders and concise root override comments. CHANGELOG now uses a release-ready Unreleased section following Keep a Changelog. The managed README TOC was regenerated. Validation passed: `just test` 140/140, and `just ci` including format, strict Clippy, udeps, audit, doctests, and coverage at 91.68%.
<!-- SECTION:FINAL_SUMMARY:END -->
