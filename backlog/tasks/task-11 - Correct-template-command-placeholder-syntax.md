---
id: TASK-11
title: Correct template command placeholder syntax
status: Done
assignee:
  - '@OpenCode'
created_date: '2026-09-09 05:57'
updated_date: '2026-09-10 06:40'
labels:
  - documentation
  - template
  - config
dependencies: []
references:
  - crates/nxus-core/src/project_template/template/nxus.toml
priority: medium
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
The canonical generated project template contains a custom-command example using obsolete single-brace Nxus placeholder syntax. Replace it with the current double-brace syntax so generated projects do not receive a non-expanding Nxus placeholder.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [x] #1 Generated template custom-command examples use current Nxus double-brace placeholder syntax
- [x] #2 Template-related validation passes
<!-- AC:END -->

## Implementation Plan

<!-- SECTION:PLAN:BEGIN -->
1. Replace obsolete single-brace Nxus placeholders in the canonical generated template and active repository example with the implemented double-brace syntax. 2. Keep template comments concise while documenting the relevant root configuration semantics. 3. Validate the generated template through the repository test and CI recipes. The user explicitly requested this documentation pass, which approves this plan.
<!-- SECTION:PLAN:END -->

## Implementation Notes

<!-- SECTION:NOTES:BEGIN -->
Updated the canonical generated `nxus.toml` command example to use `{{build_dir}}`, added the supported `cwd = "docs"` example, and aligned concise root comments with environment overrides. `just test` passed 140/140 tests and `just ci` passed with 91.68% line coverage.
<!-- SECTION:NOTES:END -->

## Final Summary

<!-- SECTION:FINAL_SUMMARY:BEGIN -->
Updated the canonical generated project configuration so custom-command placeholders use the implemented double-brace syntax. The documentation command now demonstrates `cwd`, and concise comments identify overlay/build/workspace root overrides without duplicating the main README. Validation passed with `just test` (140/140) and `just ci` (format, strict Clippy, udeps, audit, doctests, coverage); overall line coverage was 91.68%.
<!-- SECTION:FINAL_SUMMARY:END -->
