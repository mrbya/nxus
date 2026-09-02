# __ZAPPY_PROJECT_NAME__

<!-- Add project overview here -->

Opinionated project based on [NuttX](https://nuttx.apache.org/) managed by [Nxus](https://gitlab.com/byacrates/nxus).

<!-- Project overview end -->

## SDK setup

Project based on [NuttX RTOS](https://nuttx.apache.org/).

- [SDK Setup](https://nuttx.apache.org/docs/latest/quickstart/install.html)

You do not have to download NuttX as nxus manages a project-local workspace containing `nuttx` and `nuttx_apps`.

## Supported targets

| Profile | Target |
| -------------- | --------------- |
| `sim` | `sim` |
| `prod` | `nucleo_f767zi` |

## Building and flashing

1. Build a specific profile using `nxus`:

```bash
nxus build # defaults to sim profile

# or manually specify sim profile
nxus -p sim build

# or build for prod profile
nxus -p prod build
```

## Flashing binary and running simulations

Flashing binary:

```bash
nxus -p prod flash
```

```bash
nxus sim

# or manually build and run for sim profile
nxus -p sim build
nxus -p sim run
```

## Tests

Run tests using `nxus`:

```bash
nxus test

# or manually build and run for test profile
nxus -p test build
nxus -p test run
```

## Project structure

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
