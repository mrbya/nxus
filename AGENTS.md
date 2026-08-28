# AGENTS.md

## Commands
- Prefer `just` over guessing raw cargo commands. `just list` shows the supported workflows.
- Format with `just fmt` or `cargo +nightly fmt --all`. This repo's formatter recipe requires nightly.
- Lint with `just check` (`cargo clippy --tests --examples --all-targets --all-features --workspace`).
- Run tests with `just test` (`cargo nextest run --all-features --workspace`). For a focused run, pass nextest filters through `just test -- ...`.
- Run doc tests with `just doctest`.
- `just thorough-check` runs the expected static checks in order: `fmt --check`, `check -- -D warnings`, `unused`, `audit`.
- `just pre-commit` is heavier than CI: it formats, runs `thorough-check`, runs doctests, then runs coverage via `cargo llvm-cov nextest`.
- `just ci` is the non-modifying validation flow used by CI and the pre-commit hook.
- `just init` bootstraps required tooling beyond Rust itself: `cargo-nextest`, `cargo-llvm-cov`, `cargo-udeps`, `cargo-audit`, `markdown-toc`, and `pre-commit`.

## Workspace Shape
- This is a Cargo workspace with three packages: root `nxus`, `crates/nxus-shell`, and `crates/nxus-core`.
- The root crate is only the installable facade: `src/main.rs` calls `nxus::run()`, and `src/lib.rs` re-exports `nxus_shell::cli::run`.
- CLI parsing and command dispatch live in `crates/nxus-shell/src/cli.rs` and `crates/nxus-shell/src/commands/`.
- Core config discovery, config loading/resolution, path handling, workspace setup, and command execution helpers live in `crates/nxus-core/src/`.

## Repo-Specific Behavior
- `nxus` discovers `nxus.toml` by walking upward from the current working directory. Running the CLI anywhere under this repo will pick up the repo-root `nxus.toml` unless you change directories.
- The checked-in `nxus.toml` is active config, not just documentation. CLI commands may create or modify repo-local `build/` and `workspace/` directories.
- `nxus config`/`build` depend on external NuttX tooling and can clone/fetch `workspace/nuttx` and `workspace/nuttx-apps`, create symlinks, and invoke `cmake`, `ninja`, and `git`.
- `nxus test` is the product command for the default `test` profile, not the Rust test suite. Use `just test` for Rust tests.

## Hooks And CI
- Git hooks are configured through `.pre-commit-config.yaml`.
- Rust/TOML changes trigger `just ci`; README changes trigger `just index` to rewrite the TOC.
- Commit messages are checked for conventional-commit format by the `commit-msg` hook.
- CI is GitLab-based (`.gitlab-ci.yml`), not GitHub Actions. The test job runs `just ci`; tagged builds run `just build` and publish `target/release/nxus`.

## Style Constraints
- Crates enable aggressive linting, including `missing_docs`, `clippy::pedantic`, `clippy::nursery`, and several deny-level clippy lints. Small code changes often need matching doc comments.
- Rustdoc style guidance lives in `docs/rustdoc_style.md`; consult it before changing crate/module/item docs in `crates/`.

<!-- BACKLOG.MD MCP GUIDELINES START -->

<CRITICAL_INSTRUCTION>

## BACKLOG WORKFLOW INSTRUCTIONS

This project uses Backlog.md MCP for all task and project management activities.

**CRITICAL GUIDANCE**

- If your client supports MCP resources, read `backlog://workflow/overview` to understand when and how to use Backlog for this project.
- If your client only supports tools or the above request fails, call `backlog.get_workflow_overview()` tool to load the tool-oriented overview (it lists the matching guide tools).

- **First time working here?** Read the overview resource IMMEDIATELY to learn the workflow
- **Already familiar?** You should have the overview cached ("## Backlog.md Overview (MCP)")
- **When to read it**: BEFORE creating tasks, or when you're unsure whether to track work

These guides cover:
- Decision framework for when to create tasks
- Search-first workflow to avoid duplicates
- Links to detailed guides for task creation, execution, and finalization
- MCP tools reference

You MUST read the overview resource to understand the complete workflow. The information is NOT summarized here.

</CRITICAL_INSTRUCTION>

<!-- BACKLOG.MD MCP GUIDELINES END -->
