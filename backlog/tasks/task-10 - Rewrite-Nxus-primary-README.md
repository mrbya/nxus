---
id: TASK-10
title: Rewrite Nxus primary README
status: Done
assignee:
  - '@OpenCode'
created_date: '2026-09-09 05:56'
updated_date: '2026-09-09 06:07'
labels:
  - documentation
  - readme
  - release-readiness
dependencies: []
references:
  - README.md
  - .idea/kaze-readme.md
  - AGENTS.md
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Make the root README the concise but comprehensive user-facing manual for Nxus. It must accurately reflect the current CLI, configuration schema, project template, workspace behavior, and custom-command functionality rather than preserving stale documentation.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Root README explains Nxus purpose installation requirements quick start and generated project layout
- [x] #2 Configuration reference accurately documents discovery schema profiles overlays workspace and compile-command linkage
- [x] #3 Command and workflow documentation matches current CLI behavior
- [x] #4 Custom commands document exec list adjacent comment descriptions cwd double-brace placeholders structured execution and raw argument forwarding
- [x] #5 README table of contents is regenerated and documentation validation including just ci passes
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Audit the current CLI help, configuration schema/resolution, command paths, workspace behavior, template, and tests to establish documentation facts. 2. Replace README.md with an approachable manual covering setup, scaffold/adoption, configuration, normal workflows, custom commands, workspace management, development, and licensing using only verified behavior. 3. Regenerate the established TOC, review examples against source/help, check for obsolete placeholder syntax, and run README validation plus just ci. 4. Record results and any separately tracked discrepancy in this task.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Audited live CLI help, config schema/resolution, command handlers, workspace paths, project template, and integration tests before rewriting README. Created follow-up TASK-11 for the template's stale single-brace custom-command placeholder and TASK-12 because overlay_root is resolved but not used by generated configuration. Neither was changed in this documentation-scoped work.

Regenerated the managed README TOC with `just index`, checked the README for obsolete single-brace Nxus placeholders, and reviewed all examples against live CLI help and source behavior. `just ci` passed: formatting, Clippy with warnings denied, udeps, audit, doctests, and 137/137 nextest tests; final overall line coverage was 91.50%.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Replaced the root README with Nxus's primary user-facing manual. It now explains the problem Nxus solves, source installation and host requirements, project scaffolding/adoption, the generated layout, config discovery, all public configuration tables, profile and overlay behavior, build/flash/run/test/clean workflows, workspace lifecycle, custom structured commands, `exec list` comment descriptions, `cwd`, double-brace placeholders, compile-command linkage, verbosity/dry-run behavior, development, and licensing.

The documentation was grounded in the current CLI help, config/resolution code, command handlers, workspace/path helpers, template, and integration tests. The managed TOC was regenerated with `just index`. Validation passed with `git diff --check` and `just ci`, including formatting, strict Clippy, unused-dependency and security audits, doctests, and 137/137 nextest tests (91.50% overall line coverage).

Follow-ups are tracked separately: TASK-11 fixes a stale single-brace placeholder in the generated template, and TASK-12 addresses the currently ineffective `overlay_root` setting.
<!-- SECTION:FINAL_SUMMARY:END -->
