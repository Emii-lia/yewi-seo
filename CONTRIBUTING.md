# Contributing to yewi-seo

Thank you for considering contributing to `yewi-seo`! This document covers everything you
need to get started with local development, testing, and submitting changes.

---

## Table of Contents

- [Prerequisites](#prerequisites)
- [Getting started](#getting-started)
- [Project structure](#project-structure)
- [Development commands](#development-commands)
- [Running the example](#running-the-example)
- [Testing](#testing)
- [Code style & linting](#code-style--linting)
- [Making changes](#making-changes)
- [Branch and commit naming](#branch-and-commit-naming)
- [Pull request checklist](#pull-request-checklist)
- [CI / CD](#ci--cd)
- [Troubleshooting](#troubleshooting)

---

## Prerequisites

| Tool              | Version   | Notes                                                          |
|-------------------|-----------|----------------------------------------------------------------|
| Rust              | stable    | Installed via [`rustup`](https://rustup.rs)                    |
| `wasm32-unknown-unknown` target | — | See [Installation](#1-install-the-wasm-target) below                          |
| `cargo-make`      | ≥ 0.32.4  | Task runner used for project-level commands (optional but recommended) |

### 1. Install the wasm target

```shell
rustup target add wasm32-unknown-unknown
```

### 2. Install cargo-make (recommended)

```shell
cargo install cargo-make
```

The project uses a `Makefile.toml` at the root to standardise common tasks.
If you prefer not to install `cargo-make`, you can fall back to plain `cargo` commands.

### 3. Install Trunk and Yewi (to run the example)

The example app is built with [Trunk](https://trunkrs.dev) and [Yewi](https://yewi.fiaro.app):

```shell
cargo install trunk yewi-cli
```

---

## Getting started

```shell
# Clone your fork
git clone https://github.com/Emii-lia/yewi-seo.git
cd yewi-seo

# The toolchain is pinned via rust-toolchain.toml – just run:
rustup show

# Build everything
cargo build --workspace

# Run the macro tests (the bulk of the test suite)
cargo test -p yewi-seo-macro
```

---

## Project structure

```
yewi-seo/
├── Cargo.toml              # workspace root
├── Makefile.toml           # cargo-make tasks
├── rust-toolchain.toml     # Rust toolchain pinning
├── .editorconfig           # editor style preferences
├── CONTRIBUTING.md         # you are here
├── .github/workflows/      # CI pipeline
│
├── packages/
│   ├── yewi-seo/           # Runtime library (re-exports the macro + sets <head>)
│   │   ├── src/
│   │   │   ├── lib.rs
│   │   │   ├── dioxus.rs
│   │   │   ├── leptos.rs
│   │   │   ├── yew.rs
│   │   │   ├── meta/
│   │   │   ├── icon/
│   │   │   ├── link/
│   │   │   ├── open_graph/
│   │   │   ├── twitter/
│   │   │   ├── traits/
│   │   │   └── utils/
│   │   └── Cargo.toml
│   │
│   └── yewi-seo-macro/     # Procedural macro crate (#[seo], apply_*)
│       ├── src/
│       ├── tests/           # trybuild + unit tests
│       └── Cargo.toml
│
└── examples/
    └── basic/               # Yewi-seo yew demo app
    └── dioxus-seo/               # Yewi-seo dioxus demo app
    └── leptos-seo/               # Yewi-seo leptos demo app
```

### Package responsibilities

- **yewi-seo-macro** : Contains the `#[seo]` attribute macro and the individual
  `apply_meta!`, `apply_open_graph!`, etc. declarative macros. This is a
  `proc-macro` crate with no WASM dependency. All compile-time validation
  happens here.

- **yewi-seo** : The public-facing crate that re-exports the macros and
  provides the runtime `apply_seo_*` functions that write tags into
  `document.head` via `web-sys`.

---

## Development commands

### Plain cargo

```shell
cargo check -p yewi-seo-macro          # quick check (fastest)
cargo check --workspace                 # check all crates including example
cargo build --workspace                 # full build
cargo test -p yewi-seo-macro            # macro tests
cargo clippy --workspace -- --deny=warnings
```

### With cargo-make

```shell
cargo make check                 # same as cargo check --workspace
cargo make clippy                # clippy with -Dwarnings
cargo make test                  # cargo test --all-targets
cargo make doc-test              # cargo test --doc --all-features
```

---

## Running the example

### Yew

```shell
cd examples/basic
# Install dependencies (only once)
yewi install
# Start the Trunk dev server (auto-reloads on changes)
trunk serve --open
```


The example requires the `wasm32-unknown-unknown` target (see
[Prerequisites](#prerequisites)).

### Dioxus

```shell
dx serve --package dioxus-seo
```

### Leptos

```shell
cd examples/leptos-seo
trunk serve --open
```

---

## Testing

### Macro tests (trybuild)

The macro crate uses [`trybuild`](https://github.com/dtolnay/trybuild) for
compile-fail / compile-pass tests. Test cases live in
`packages/yewi-seo-macro/tests/`.

- **Pass tests** : `.rs` files whose names end with `_pass.rs`. They must
  compile successfully.
- **Fail tests** : `.rs` files whose names end with `_fail.rs` plus a
  matching `.stderr` snapshot. The compiler must produce the exact error
  messages captured in the snapshot.

To regenerate `.stderr` snapshots after changing error messages:

```shell
cd packages/yewi-seo-macro
cargo make test-overwrite    # equivalent: TRYBUILD=overwrite cargo test -p yewi-seo-macro
```

Commit the updated `.stderr` files together with your changes.

### Running specific tests

```shell
# Run a single test file
cargo test -p yewi-seo-macro --test seo_test

# Run all tests in a subdirectory (e.g. apply_meta)
cargo test -p yewi-seo-macro --test apply_meta_test
```

---

## Code style & linting

- **Clippy** : The CI pipeline enforces `--deny=warnings`. Run
  `cargo make clippy` locally.
- **EditorConfig** : A `.editorconfig` file is provided; most editors support
  it via a plugin. Please respect the indentation and line-ending settings.

---

## Making changes

1. **Open an issue** (or comment on an existing one) to discuss the change.
   This avoids wasted effort on something that may not be accepted.
2. **Fork the repository** and create a feature branch from `master`.
3. **Make your changes** — consider test coverage as you go.
4. **Run the full check suite** locally:
   ```shell
   cargo make clippy
   cargo make test
   cargo make doc-test
   ```
5. **Update `.stderr` snapshots** if you change compile-time error messages
   (see [Macro tests](#macro-tests-trybuild)).
6. **Open a pull request** against `master`.

---

## Branch and commit naming
### Branch / PR title naming

| Name              | Usage                                             |
|-------------------|---------------------------------------------------|
| `feat/<name>`     | for new features                                  |
| `fix/<name>`      | for bugfix                                        |
| `refactor/<name>` | for code refactoring                              |
| `perf/<name>`     | for improvements on the existing features or code |
| `docs/<name>`     | for docmentation changes                           |
| `docs/<name>`     | for docmentation changes                           |

### Commit conventions
- **Commit message format** : `<type>(<scope>): <subject>`
- **Commit types** : `feat`, `fix`, `refactor`, `perf`, `docs`, `test`, `chore`, `ci`, `build`, `revert`

> Tips: Use conventional commits plugin to make it easier to generate commits. For example, a commit message could look like: `feat(seo): add support for Twitter cards`.

## Pull request checklist

- [ ] PR title is descriptive and references the related issue (if any).
- [ ] All CI checks pass (check, fmt, clippy, test, build).
- [ ] New features include tests (pass or fail depending on the case).
- [ ] Compile-fail tests include an `.stderr` snapshot when error messages
      change.
- [ ] Documentation is updated if the public API changes
      (README, doc comments, or both).
- [ ] Changes are backward-compatible unless discussed otherwise.

---

## CI / CD

The CI pipeline (`.github/workflows/build.yml`) runs on every push and PR
against `master`. It consists of:

| Job     | Command                            |
|---------|------------------------------------|
| check   | `cargo check --workspace`          |
| clippy  | `cargo clippy --workspace -- --deny=warnings` |
| test    | `cargo test -p yewi-seo-macro` + `cargo test -p yewi-seo --doc` |
| build   | `cargo build --workspace` (blocked on previous jobs) |

All jobs respect the `rust-toolchain.toml` version pinning via `rustup show`.

---

## Troubleshooting

| Symptom                            | Likely cause / fix                               |
|------------------------------------|--------------------------------------------------|
| `error[E0463]: can't find crate for std` | Forgot to add the `wasm32-unknown-unknown` target (see [Prerequisites](#prerequisites)). |
| Trybuild tests fail unexpectedly   | Run `TRYBUILD=overwrite cargo test` to regenerate snapshots if you changed error messages intentionally. |

---

## License

By contributing you agree that your contributions will be licensed under the
[MIT License](LICENSE), the same as the project itself.
