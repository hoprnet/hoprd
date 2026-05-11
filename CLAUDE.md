# CLAUDE.md

Read `.claude/INSTRUCTIONS.md` for full development guidelines and conventions.

When working on Rust files, also read `.claude/rust.md` for language-specific rules.

## Workflow

1. Before modifying code, understand the surrounding context and existing patterns.
2. For multi-step features, plan before implementing.
3. After changes, run `cargo check` (or the closest package check) to verify.
4. For Rust changes, run `cargo shear --fix -p <crate>` (never `--fix` at workspace root without `-p`) followed by `cargo check`.
5. Run `nix fmt` to format.
6. Run `nix run -L .#check` (clippy + linters) before committing.
7. Run tests (unit and integration separately).

## Tests

### Unit Tests

```bash
nix develop -c cargo nextest run --lib
nix develop -c cargo nextest run --lib -p <crate>
```

### Integration Tests

Integration tests **must** run single-threaded:

```bash
nix develop -c cargo nextest run --test '*' -j 1
nix develop -c cargo nextest run -p <crate> --test <test_name> -j 1
```
