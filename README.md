# Nxus

Nxus is a CLI build-system companion for opinionated NuttX projects, driven by a declarative `nxus.toml` configuration.

## Index

<!-- toc -->

- [Why?](#why)
- [Features](#features)
- [Installation](#installation)
- [Requirements](#requirements)
- [Usage](#usage)
  * [Quick start](#quick-start)
  * [Project layout](#project-layout)
- [Nxus configuration](#nxus-configuration)
  * [Config discovery](#config-discovery)
  * [Top-level schema](#top-level-schema)
  * [`[project]`](#project)
  * [`[build]`](#build)
  * [`[workspace]`](#workspace)
  * [Profiles](#profiles)
  * [Configuration overlays](#configuration-overlays)
- [Commands](#commands)
  * [Build and configuration](#build-and-configuration)
  * [Flash](#flash)
  * [Cleaning](#cleaning)
- [Custom project commands](#custom-project-commands)
  * [Command descriptions](#command-descriptions)
  * [Placeholders and `cwd`](#placeholders-and-cwd)
  * [Structured execution and shell execution](#structured-execution-and-shell-execution)
- [Workspace management](#workspace-management)
- [Verbosity and dry-run](#verbosity-and-dry-run)
- [Typical workflows](#typical-workflows)
  * [Simulator development](#simulator-development)
  * [Test profile](#test-profile)
  * [Production build and flash](#production-build-and-flash)
  * [Project tooling](#project-tooling)
- [Development](#development)
- [License](#license)

<!-- tocstop -->

## Why?

NuttX already provides the build system and board configurations. Nxus supplies
the project-level convention around them: a local NuttX workspace, repeatable
profile selection, generated board configurations, predictable build outputs,
and commands for building, running, testing, flashing, and project tooling.

Instead of remembering a different collection of CMake, Ninja, Git, simulator,
and programmer invocations for every target, keep the project choices in one
`nxus.toml`. A profile can represent a simulator, a test image, or production
hardware while retaining separate generated configurations and build artifacts.

Nxus manages its own project-local clones of `nuttx` and `nuttx-apps`; it does
not replace NuttX's build system or attempt to be a general-purpose shell DSL.

## Features

- Scaffold a conventional NuttX project or adopt an existing one.
- Manage project-local `nuttx` and `nuttx-apps` workspaces.
- Configure, build, run, test, and flash named NuttX profiles.
- Generate and link profile-specific board configurations from shared overlays.
- Keep profile build outputs separate and optionally link `compile_commands.json`.
- Define structured project commands with paths, artifacts, arguments, and a working directory.
- Discover `nxus.toml` when invoked from nested project directories.
- Preview external commands with dry-run mode and control output with verbosity.

---

## Installation

Nxus is installable from its source checkout:

```bash
cargo install --path .
```

For development without installation, run it with:

```bash
just run -- --help
```

The repository metadata permits publishing, but a public package release is not
assumed here. Install from a checkout until an explicitly published release is
available.

## Requirements

- Git, for the project-local NuttX repositories.
- CMake and Ninja, used to configure and build NuttX.
- The host tools and cross toolchain required by the NuttX board profiles you use.
- Rust only when installing or building Nxus from source.

Use the [NuttX installation guide](https://nuttx.apache.org/docs/latest/quickstart/install.html)
for the platform-specific NuttX SDK and toolchain setup. Nxus does not install
those dependencies.

---

## Usage

```text
Usage: nxus [OPTIONS] <COMMAND>

Commands:
  clean       Cleans build artifacts and workspace
  config      Configures `NuttX` for a specific profile
  build       Builds project for a specific profile
  menuconfig  Opens Kconfig config TUI for a specific profile
  run         Runs binary built for a specific profile
  flash       Flashes binary for a specific profile
  exec        Executes a project-defined command
  sim         Runs simulation in default simulation profile
  test        Runs test suite for the default test profile
  workspace   Project-local `NuttX` workspace management
  init        Initializes `NuttX` project with `nxus.toml`
  profiles    Lists available profiles

Options:
  -c, --clean              Pre-clean build dir for the given profile
  -r, --rebuild            Rebuild binary for selected profile before running/flashing
  -v, --verbose...         Verbosity (repeatable)
  -d, --dry-run            Dry run command?
  -p, --profile <PROFILE>  Profile to run command for
```

Use `nxus <command> --help` for command-specific help. Global options precede
the command. Common invocations are:

```bash
nxus profiles
nxus config
nxus build
nxus run
nxus -p prod flash
nxus exec list
```

---

### Quick start

Create a new project from the built-in template:

```bash
nxus init project demo
cd demo
nxus workspace init
nxus config
nxus build
nxus run
```

`workspace init` clones or updates the configured NuttX repositories. The first
`nxus config` also ensures that workspace, so it is safe to omit the explicit
workspace step when that is what you want. Run Nxus from `demo`: it contains
`nxus.toml`, is linked as the NuttX external application, and supplies the
overlay files.

To adopt an existing application directory, run this from its intended project
root. It only creates `nxus.toml` and refuses to overwrite an existing one:

```bash
nxus init config
```

Then add the profiles and overlays appropriate for the application. Nxus expects
the application/configuration working directory to be the directory from which
you run normal project commands.

### Project layout

`nxus init project demo` creates this canonical layout (some source directories
contain starter files and others are intentionally empty):

```bash
.
├── app                 # Project app
│   ├── include         # App includes
│   │   └── ...
│   ├── src             # App sources
│   │   └── ...
│   │
│   ├── CMakeLists.txt
│   └── Kconfig
│
├── config              # Profile-specific config overlays
│   └── ...
│
├── docs                # Doxide documentation helpers
│   └── ...
│
├── lib                 # App dependencies
│   └── ...
│
├── test                # Tests
│   └── ...
│
├── CMakeLists.txt      # Global project cmake config
├── doxide.yaml         # Doxide docs config
├── Kconfig             # Global app kconfig
├── mkdocs.yaml         # Mkdocs config for doc generation
├── nxus.toml           # Nxus config
└── README.md

```

`nxus.toml`, the application sources, CMake files, Kconfig files, and `config/`
are project inputs worth committing. `workspace/` and `build/` are generated by
Nxus and are ignored by the template. The application source root is the project
root in the template, not an `app/app/` directory.

---

## Nxus configuration

`nxus.toml` is the declarative project configuration. Nxus loads defaults first,
then overlays values written in this file. The default profiles are `sim` and
`test`; a project can add `prod` or any other named profile.

### Config discovery

Except for `nxus init`, Nxus walks upward from the current directory until it
finds `nxus.toml`. Therefore commands work from nested directories such as
`app/src/module/`. The directory containing `nxus.toml` is the project root.

This is distinct from the invocation directory. Nxus links the invocation
directory to `workspace/nuttx-apps/external` and reads configuration overlays
from `<invocation-directory>/config/`. In the generated template, invoke Nxus
from the project root so these paths are the intended application and `config/`
directory.

### Top-level schema

| Table | Purpose |
| --- | --- |
| `[project]` | Project defaults, including the active profile selection. |
| `[build]` | Build-output location and compile-database linking. |
| `[workspace]` | Project-local workspace location. |
| `[workspace.nuttx]` | NuttX repository source and optional revision. |
| `[workspace.nuttx_apps]` | NuttX apps repository source and optional revision. |
| `[profile.<name>]` | A complete target/profile definition. |
| `[profile.<name>.flash]` | Structured flash command for one profile. |
| `[command.<name>]` | A reusable project command run through `nxus exec`. |

### `[project]`

| Key | Type | Default | Description |
| --- | --- | --- | --- |
| `default_profile` | string | `"sim"` | Profile used when `-p` / `--profile` is absent. |
| `overlay_root` | string | `"config"` | Resolved as metadata, but currently does not change the overlay files consumed by `nxus config`; use the invocation directory's `config/` directory. |

`-p prod` always selects `prod` for ordinary profile-aware commands. `sim` and
`test` deliberately select their names regardless of `default_profile` or `-p`.

### `[build]`

| Key | Type | Default | Description |
| --- | --- | --- | --- |
| `root` | string | `"build"` | Build root under the project root. |
| `link_compile_commands` | bool | `true` | Link the selected profile's compile database at the build root. |

Build output for profile `prod` is `<project>/build/prod/`; the standard NuttX
artifacts there are `nuttx`, `nuttx.bin`, and `nuttx.hex`.

When `link_compile_commands` is enabled, after a successful `config` or `build`
with a profile build directory and its `compile_commands.json`, Nxus recreates:

```text
<project>/build/compile_commands.json -> <project>/build/<active-profile>/compile_commands.json
```

This makes a conventional compile database available to editors and tools that
only look at the build root. The link follows whichever profile was most recently
configured or built. Set the option to `false` to leave no Nxus-managed link.

### `[workspace]`

| Key | Type | Default | Description |
| --- | --- | --- | --- |
| `root` | string | `"workspace"` | Workspace root under the project root. |
| `nuttx.src` | string | `https://github.com/apache/nuttx.git` | NuttX Git source. |
| `nuttx.rev` | string | unset | Revision to check out after fetching. |
| `nuttx_apps.src` | string | `https://github.com/apache/nuttx-apps.git` | NuttX apps Git source. |
| `nuttx_apps.rev` | string | unset | Revision to check out after fetching. |

For reproducible projects, pin both revisions:

```toml
[workspace]
root = "workspace"

[workspace.nuttx]
rev = "master"

[workspace.nuttx_apps]
rev = "master"
```

`workspace init` creates the root as needed, clones missing repositories, fetches
existing repositories, and checks out configured revisions. Nxus then links the
invocation directory to `workspace/nuttx-apps/external` during configuration.

### Profiles

A profile is a complete NuttX board selection and an isolated build/configuration
name. Its required fields identify the board configuration Nxus uses:

| Key | Type | Description |
| --- | --- | --- |
| `arch` | string | NuttX board architecture path segment. |
| `family` | string | NuttX board family path segment. |
| `board` | string | NuttX board directory name. |
| `config_base` | string | Existing NuttX board configuration used as the base. |
| `flash` | table | Optional structured flash command. |

The generated defaults are `sim` and `test`, both using the NuttX `sim/sim/sim`
board and `nsh` base configuration. A typical project adds production hardware:

```toml
[project]
default_profile = "sim"

[profile.sim]
arch = "sim"
family = "sim"
board = "sim"
config_base = "nsh"

[profile.test]
arch = "sim"
family = "sim"
board = "sim"
config_base = "nsh"

[profile.prod]
arch = "arm"
family = "stm32f7"
board = "nucleo-f767zi"
config_base = "evalos"
```

Use `nxus profiles` to see configured names and targets. A requested unknown
profile is an error.

### Configuration overlays

For profile `prod`, `nxus config` generates:

```text
<project>/workspace/config/prod/defconfig
```

The generated file includes, in order:

1. The selected upstream base: `workspace/nuttx/boards/<arch>/<family>/<board>/configs/<config_base>/defconfig`.
2. `<project_dir>/config/common.config`, when present.
3. `<project_dir>/config/prod.overlay`, when present.

It links that generated configuration into the selected NuttX board's `configs/`
directory under the profile name, then configures CMake with `BOARD_CONFIG` set
to `<board>:<profile>`. Put common Kconfig selections in `common.config` and
profile-specific selections in `<profile>.overlay`.

The generated defconfig is created only when absent. Delete it or use `nxus clean`
when you need Nxus to regenerate it after changing its inputs.

---

## Commands

### Build and configuration

`nxus config` ensures the workspace, links the application, creates the generated
defconfig and board-config link, creates the profile build directory, and invokes
CMake with Ninja generation.

`nxus build` invokes Ninja in the active build directory. If that directory does
not exist, it runs `config` first. If it exists, Nxus assumes it is configured;
run `nxus config` yourself when its CMake configuration needs refreshing.

```bash
nxus config
nxus build
nxus -p prod config
nxus -p prod build
```

`nxus menuconfig` opens `ninja -C <build-dir> menuconfig`; it configures first
only when the active build directory does not exist.

`nxus run` executes the selected profile's `nuttx` binary. It builds first when
the binary does not exist and honours `--rebuild` when it does. Its program output
is inherited so interactive simulators remain usable.

`nxus sim` always runs profile `sim`, while `nxus test` always runs profile
`test`; both follow the `run` build-if-missing/rebuild behavior. They are product
commands, not the Nxus Rust test suite.

### Flash

Add a structured flash command under the profile:

```toml
[profile.prod.flash]
command = "openocd"
args = [
    "-f",
    "board/st_nucleo_f7.cfg",
    "-c",
    "program {{elf}} verify reset exit",
]
```

Run it with `nxus -p prod flash`. Flash builds when the profile ELF is missing,
and rebuilds it first with `--rebuild`. A profile without `flash` cannot be
flashed. Each TOML array item remains one process argument; Nxus does not invoke
a shell for this command.

### Cleaning

`nxus clean` without an explicit profile removes the whole build root and unlinks
the external application and all profile board-config links. With `-p <name>`, it
removes only that profile build directory and board-config link:

```bash
nxus clean
nxus -p prod clean
```

`--clean` pre-cleans an explicitly selected profile before most ordinary commands.
`sim` and `test` also honour it after forcing their profile.

## Custom project commands

Use `[command.<name>]` for project tooling that belongs beside the build workflow:

```toml
# Print firmware size.
[command.size]
command = "arm-none-eabi-size"
args = ["{{elf}}"]

# Build documentation from its source directory.
[command.docs]
command = "sh"
args = ["-c", "doxide build && mkdocs build"]
cwd = "docs"
```

Run a command, list commands, or append raw runtime arguments:

```bash
nxus exec list
nxus exec size
nxus exec objdump -- -d -S
nxus exec --help
```

Nxus appends arguments after `--` to the configured `args` unchanged, preserving
their argv boundaries. `exec` never builds implicitly; commands using an artifact
placeholder require that artifact to already exist.

### Command descriptions

`nxus exec list` displays commands in configuration order. It uses the single
physical `#` comment line immediately before `[command.<name>]` as the optional
description; no `description` key exists.

Leading indentation and surrounding whitespace are removed. A blank line breaks
the association. When multiple immediately adjacent comment lines exist, only the
last one is used. Commands without a description are still listed. `list` is
reserved: `[command.list]` is displayed but `nxus exec list` always lists rather
than executes it.

### Placeholders and `cwd`

Nxus expands placeholders in `command`, every configured `args` item, and `cwd`:

| Placeholder | Meaning |
| --- | --- |
| `{{project_dir}}` | Directory containing `nxus.toml`. |
| `{{workspace_dir}}` | Configured project-local workspace root. |
| `{{build_dir}}` | Active profile build directory. |
| `{{profile}}` | Active profile name. |
| `{{elf}}` | Existing `<build-dir>/nuttx` artifact. |
| `{{bin}}` | Existing `<build-dir>/nuttx.bin` artifact. |
| `{{hex}}` | Existing `<build-dir>/nuttx.hex` artifact. |

Artifact placeholders fail if their expected file does not exist. Unknown or
unclosed Nxus placeholders also fail. Nxus placeholders use double braces:
```text
{{profile}}  Nxus placeholder
${HOME}      shell-variable syntax, interpreted only by an explicitly invoked shell
```

`cwd` is optional. An absolute value is used as written; a relative value is
resolved against the project root. For example, `cwd = "docs"` runs from
`<project>/docs`, and `cwd = "{{build_dir}}"` runs from the active profile build
directory. Use it instead of prepending `cd ... &&` to a command definition.

### Structured execution and shell execution

Nxus starts the configured program directly and treats `args` as structured argv:

```toml
[command.size]
command = "arm-none-eabi-size"
args = ["{{elf}}"]
```

It does not detect shell syntax, expand pipes, or interpret shell variables. When
shell semantics are intentional, invoke a shell explicitly:

```toml
# Check source formatting.
[command.format-check]
command = "sh"
args = [
    "-c",
    "find app lib -type f \\( -name '*.cpp' -o -name '*.hpp' \\) -print0 | xargs -0 clang-format --dry-run --Werror",
]
```

## Workspace management

```text
Usage: nxus workspace <COMMAND>

Commands:
  clean  Clean workspace
  init   Initialize workspace
  prune  Prune workspace
```

`nxus workspace init` ensures the configured `nuttx` and `nuttx-apps` clones,
fetching both and checking out pinned revisions when configured. It does not
create application links or generated board configurations; `nxus config` does.

`nxus workspace prune` first runs `git stash` in both repositories, then unlinks
the external application and every profile's NuttX board-configuration link. It
keeps the workspace clones and generated configs. `nxus workspace clean` removes
the entire configured workspace directory.

## Verbosity and dry-run

Nxus defaults to verbosity level 2. Repeat `-v` or use `--verbose`:

| Invocation | Behavior for ordinary external commands |
| --- | --- |
| `-v` | Quiet: suppresses command output. |
| default / `-vv` | Shows Nxus spinner/status; command output is shown on failure. |
| `-vvv` and higher | Prints the command and inherits its output. |

Custom commands deliberately avoid the spinner/output capture used by ordinary
commands. `nxus exec` runs quietly at `-v`; at the default level and above it
prints the resolved command and inherits its output. This keeps project tools
such as documentation generators and formatters readable.

`--dry-run` prevents process execution but does not make Nxus read-only: commands
may still validate configuration, create required directories/links, or check for
required artifact placeholders before reaching an external process. The resolved
external command is printed in dry-run mode at `-vvv` or higher.

---

## Typical workflows

### Simulator development

```bash
nxus sim
```

This builds the `sim` profile if its ELF is missing, then runs it. Use the more
explicit sequence when configuration changes matter:

```bash
nxus -p sim config
nxus -p sim build
nxus -p sim run
```

### Test profile

```bash
nxus test
```

This targets profile `test`, independently of the default profile.

### Production build and flash

```bash
nxus -p prod config
nxus -p prod build
nxus -p prod flash
```

### Project tooling

```bash
nxus exec list
nxus exec format-check
nxus exec docs
```

## Development

Nxus requires Rust 1.85 or newer for source builds and [`just`](https://crates.io/crates/just)
for repository workflows. Bootstrap the extra development tools and hooks with:

```bash
cargo install just
just init
```

Useful commands:

```bash
just list
just run -- --help
just test
just pre-commit
just ci
```

`just ci` is the non-modifying repository validation flow. See `AGENTS.md` for
the complete contributor command guidance.

---

## Development

### Prequisites

- Rust stable toolchain with `rustfmt` and `clippy` (`rust-toolchain.toml`)
- Rust `1.85.0` or newer for workspace builds
- [`just`](https://crates.io/crates/just)

For a first time setup, run:
```bash
cargo install just
just init
```

This installs just and its `init` bootstrap recipe installs all extra tooling used by this repository, including coverage, lint/audit tools, README indexing, pre-commit hooks and more.

### Getting started

Run nxus:
```bash
just run -- <pass in args>

```

Run tests:
```bash
just test
```

Before committing work:
```bash
just pre-commit

```

To see all available recipes:
```bash
just list

# or

just help
```

---

## Documentation

Codebase documented using a consistent rustdoc style described in [rustdoc style guide](docs/rustdoc_style.md) and this README.

---

## License

Nxus is licensed under either of:

- [Apache License, Version 2.0](LICENSE-APACHE)
- [MIT license](LICENSE-MIT)

Unless you explicitly state otherwise, contributions intentionally submitted for
inclusion are dual licensed under the same terms.
