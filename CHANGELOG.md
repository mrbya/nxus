# Changelog

All notable changes to Nxus are documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.1.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [v1.0.0] - 2026-09-10

### Added

- Initial release of Nxus, an opinionated CLI companion for NuttX projects that
  treats NuttX and NuttX Apps as project-local workspace dependencies.
- `nxus.toml` configuration with upward discovery from nested project
  directories, built-in defaults, named profiles, and validation of profile
  definitions.
- Profile-aware `config`, `build`, `menuconfig`, `run`, `sim`, and `test`
  commands, including isolated build directories and automatic configuration or
  builds when required.
- `profiles` command for listing configured NuttX targets.
- Generated board configurations that combine the selected upstream defconfig
  with optional shared and profile-specific configuration overlays.
- Project-local workspace management through `workspace init`, `workspace
  prune`, and `workspace clean`, including cloning, fetching, and optional
  revision checkout for NuttX and NuttX Apps repositories.
- External-application and profile board-configuration links managed during
  configuration and cleanup.
- Profile-configured `flash` commands with structured arguments, artifact
  placeholders, optional working directories, and build-if-missing or explicit
  rebuild support.
- `init config` to create a starter configuration without overwriting an
  existing file and `init project` to scaffold a conventional NuttX application
  from the built-in project template.
- `exec` command for project-defined tooling, with ordered command discovery
  through `nxus exec list`, descriptions from adjacent TOML comments, and raw
  runtime arguments passed after `--`.
- Structured custom-command execution with placeholder expansion for project,
  workspace, build, overlay, profile, and NuttX artifact paths; validation for
  missing artifacts and malformed or unknown placeholders.
- Relative and absolute configurable roots for workspace, build output, and
  overlays, with `NXUS_WORKSPACE`, `NXUS_BUILD_ROOT`, and
  `NXUS_OVERLAY_ROOT` environment overrides.
- Optional `compile_commands.json` link at the build root, pointing to the most
  recently configured or built profile.
- Global profile selection, pre-cleaning, rebuild, repeatable verbosity, and
  dry-run options, plus concise aliases for commands and workspace subcommands.
- Automated unit and CLI integration coverage for configuration discovery and
  resolution, command handling, initialization, flashing, workspace operations,
  custom commands, and path overrides.

