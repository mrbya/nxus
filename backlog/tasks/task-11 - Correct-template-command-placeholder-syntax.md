---
id: TASK-11
title: Correct template command placeholder syntax
status: To Do
assignee: []
created_date: '2026-09-09 05:57'
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
- [ ] #1 Generated template custom-command examples use current Nxus double-brace placeholder syntax
- [ ] #2 Template-related validation passes
<!-- AC:END -->
