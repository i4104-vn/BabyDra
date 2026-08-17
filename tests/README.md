# BabyDra Test Suite

This folder is the **TDD safety net** for the BabyDra workspace. It exists so
tests live in one obvious, first-class place while staying close to the code
they protect.

## Layout

```
tests/
├── Cargo.toml        # workspace package `babydra-tests` (test-only, publish = false)
├── README.md
├── src/lib.rs        # shared test helpers (empty for now)
└── <area>/           # one folder per crate/area under test
    └── <name>.rs     # one small, focused test binary per concern
```

Current areas:

| Folder        | Covers                         | Depends on           |
|---------------|--------------------------------|----------------------|
| `common/`     | babydra-common pure logic      | `babydra-common`     |
| *(future)*    | ui-kit / theme / variants…     | —                    |

## How tests are split (TDD model)

- **One concern per file**: each `tests/<area>/<name>.rs` tests a single
  behavior (e.g. `vpn.rs` only parses VPN configs, `storage.rs` only formats
  sizes). New behavior → new small file, never a growing monster file.
- **Test before refactor**: when a refactor in `planning.md` touches a module,
  write/extend its tests first, run them, then refactor until green.
- **Public API only**: integration tests call the exported crate API — the
  same surface that apps use. Pure internals that need testing get
  `#[cfg(test)]` unit tests inside the crate module itself.

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
