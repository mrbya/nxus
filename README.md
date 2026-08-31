# nxus

> NuttX CLI companion

<!-- toc -->

- [Features](#features)
- [Installation](#installation)
- [Usage](#usage)
  * [Project layout](#project-layout)
  * [Flash configuration](#flash-configuration)
- [Nxus expands placeholders before executing the configured process, keeps arguments as distinct process arguments, and reports unknown placeholders or missing artifacts as errors.](#nxus-expands-placeholders-before-executing-the-configured-process-keeps-arguments-as-distinct-process-arguments-and-reports-unknown-placeholders-or-missing-artifacts-as-errors)
- [Development](#development)
  * [Prequisites](#prequisites)
  * [Getting started](#getting-started)
- [Documentation](#documentation)
  * [Style](#style)
- [License](#license)

<!-- tocstop -->

## Features

- Nxus project initialization for existing and new NuttX workspaces
- Profile-aware configure, build, run, test, and flash workflows
- Project-local NuttX and `nuttx-apps` workspace management

---

## Installation

Install using cargo:
```bash
cargo install nxus
```

## Usage

```bash
nxus <COMMAND>

Commands:
  init         Initialize Nxus config or scaffold a project
  build        Build project for a specific profile
  flash        Flash project binary for a specific profile
  menuconfig   Open Kconfig config TUI for a specific profile
  sim          Run the default simulator profile
  test         Run the default test profile
  workspace    Manage the project-local NuttX workspace
  profiles     List available profiles
  help         Print this message or the help of the given subcommand(s)

Options:
  -c, --clean
  -v...
  -d, --dry-run
  -p, --profile <PROFILE>
  -h, --help     Print help
  -V, --version  Print version

```

Common flows:

```bash
# Adopt Nxus in an existing project directory
nxus init config

# Scaffold a new project rooted at ./demo
nxus init project demo

# Build or flash a profile
nxus -p prod build
nxus -p prod flash
```

### Project layout

`nxus init project demo` creates a small canonical scaffold:

```text
demo/
├── .gitignore
├── nxus.toml
└── app/
    ├── CMakeLists.txt
    ├── Kconfig
    ├── app/
    ├── config/
    │   ├── common.config
    │   ├── sim.overlay
    │   └── test.overlay
    ├── lib/
    └── test/
```

Run project commands from the application root, such as `demo/app`, so Nxus can discover the project config from `nxus.toml` in the parent directory while using the current directory as the app/config root.

### Flash configuration

Configure flashing per profile in `nxus.toml`:

```toml
[profile.prod]
arch = "arm"
family = "stm32f7"
board = "nucleo-f767zi"
config_base = "evalos"

[profile.prod.flash]
command = "openocd"
args = [
  "-f",
  "board/st_nucleo_f7.cfg",
  "-c",
  "program {elf} verify reset exit",
]
```

Supported substitutions:

```text
{project_dir}
{workspace_dir}
{build_dir}
{profile}
{elf}
{bin}
{hex}
```

Nxus expands placeholders before executing the configured process, keeps arguments as distinct process arguments, and reports unknown placeholders or missing artifacts as errors.
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

TBD

### Style

Codebase documented using a consistent rustdoc style described in [rustdoc style guide](docs/rustdoc_style.md).

---

## License

Dual licensed under:

- Apache License 2.0 (`LICENSE-APACHE`)
- MIT (`LICENSE-MIT`)
