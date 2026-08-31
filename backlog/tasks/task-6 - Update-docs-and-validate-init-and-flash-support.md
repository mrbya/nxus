---
id: TASK-6
title: Update docs and validate init and flash support
status: Done
assignee:
  - OpenCode
created_date: '2026-08-29 07:38'
updated_date: '2026-08-29 07:56'
labels:
  - docs
  - validation
  - init
  - flash
dependencies:
  - TASK-5
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Update CLI help text and repository documentation so the new `nxus init` subcommands and `nxus flash` command are discoverable and accurate. Run the repository's standard validation workflows, fix any issues that surface, and leave Backlog state synchronized with the completed implementation.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 CLI examples and help text reflect `nxus init config`, `nxus init project [PATH]`, and `nxus flash`
- [x] #2 Documentation explains the flash profile configuration shape and supported placeholders
- [x] #3 Relevant `just` formatting lint and test workflows are run and any failures are fixed
- [x] #4 Final Backlog task states plan notes and completion summaries match the completed implementation
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Update CLI examples/help text and README usage/documentation for the new `init` subcommands and `flash` profile configuration.
2. Run the repository validation workflows with `just` recipes, fix any failures, and confirm `just ci` passes cleanly.
3. Finalize Backlog records with checked acceptance criteria, implementation notes, and final summaries.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Updated CLI examples/help text for the new init subcommands and flash usage, rewrote the README usage section to describe the current command set, documented the canonical scaffold shape, and documented flash configuration placeholders.

Ran `just fmt`, `just test`, `just index`, and `just ci`. `just ci` passed, and the coverage summary reported 92.61% line coverage overall.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Updated user-facing help and README documentation for `nxus init config`, `nxus init project [PATH]`, and `nxus flash`, including the current scaffold layout and supported flash placeholders. Regenerated the README TOC and ran the repository validation flows, finishing with a clean `just ci` pass and overall line coverage above 90%.
<!-- SECTION:FINAL_SUMMARY:END -->
