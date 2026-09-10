---
id: TASK-5
title: Add profile-based flash execution
status: To Do
assignee: []
created_date: '2026-08-29 07:38'
labels:
  - cli
  - core
  - flash
dependencies: []
priority: high
---

## Description

<!-- SECTION:DESCRIPTION:BEGIN -->
Implement the remaining `nxus flash` command path using a structured flash command configured per profile in `nxus.toml`. Extend the core configuration schema, resolve the selected profile's flash command, expand a limited set of Nxus-managed placeholders for project and build artifacts, reuse the existing runner abstraction, and ensure required artifacts are present before execution.
<!-- SECTION:DESCRIPTION:END -->

## Acceptance Criteria
<!-- AC:BEGIN -->
- [ ] #1 Profile configuration supports an optional structured flash command with separate command and args fields
- [ ] #2 `nxus flash` follows the normal selected profile behavior and reports an actionable error when flash is not configured
- [ ] #3 Supported placeholders for project workspace build profile and artifacts are expanded by Nxus before execution
- [ ] #4 Unknown placeholders and missing requested artifact files return clear errors
- [ ] #5 Flash reuses existing configure or build behavior to ensure artifacts are available before running the configured programmer
- [ ] #6 Automated tests cover deserialization placeholder expansion missing artifacts dry-run command construction and runner error propagation
<!-- AC:END -->
