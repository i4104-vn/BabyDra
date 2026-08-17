# BabyDra Test Suite

This folder is the **TDD safety net** for the BabyDra workspace. It exists so
tests live in one obvious, first-class place while staying close to the code
they protect.

## Layout

```
tests/
├── Cargo.toml        # workspace package `babydra-tests` (test-only, publish = false)
├── README.md
└── <area>/           # one folder per crate/area under test
    └── <name>.rs     # one small, focused test binary per concern
```

Current areas:

| Folder         | Covers                               | Depends on              |
|----------------|--------------------------------------|-------------------------|
| `common/`      | babydra-core pure logic              | `babydra-core`          |
| `models/`      | shell theme config, explore grouping | `babydra-core`          |
| `services/`    | wallpaper avatar cropping            | `babydra-core`, `gtk4`  |
| `theme/`       | theme package engine + tokens        | `babydra-theme`         |
| `installer/`   | variant parsing, theme selection     | `babydra-installer`     |

All test binaries are declared explicitly in `tests/Cargo.toml` (`[[test]]`
entries) so each file compiles as its own small, focused binary.

## How tests are split (TDD model)

- **One concern per file**: each `tests/<area>/<name>.rs` tests a single
  behavior (e.g. `vpn.rs` only parses VPN configs, `storage.rs` only formats
  sizes). New behavior → new small file, never a growing monster file.
- **Test before refactor**: when a refactor in `planning.md` touches a module,
  write/extend its tests first, run them, then refactor until green.
- **Public API only**: integration tests call the exported crate API — the
  same surface that apps use. Inline `#[cfg(test)]` modules have been
  migrated out of the workspace crates into this suite, so the crates stay
  free of test code; anything that needs a unit test now goes through a
  `pub` function here.

## Running

```bash
# Everything (library + installer + tests)
cargo test --workspace

# Only this suite
cargo test -p babydra-tests

# One test binary
cargo test -p babydra-tests --test common_vpn

# One test case
cargo test -p babydra-tests --test common_vpn parses_openvpn_config
```

## Adding a new test

1. Create `tests/<area>/<name>.rs` with a doc comment explaining the concern.
2. Register it in `tests/Cargo.toml`:

   ```toml
   [[test]]
   name = "<area>_<name>"
   path = "<area>/<name>.rs"
   ```

3. Add the crate you test to `[dev-dependencies]` in `tests/Cargo.toml`.
4. Run it and keep it green.

Every commit that changes behavior in a tested module should also run the
matching test binary — that is the contract this suite enforces.
