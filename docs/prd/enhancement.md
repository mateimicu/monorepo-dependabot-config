# PRD: Enhancement (Phases 4–5)

**Date**: 2026-02-18
**Status**: Draft
**Companion PRD**: [Stabilization (Phases 1–3)](stabilization.md)
**Source**: [Maintenance Plan](../maintenance/maintenance-plan.md)
**Prerequisite**: Stabilization PRD must be completed first.

## Goal

Restructure the `monorepo-dependabot-config` codebase into a modular architecture and bring test coverage to acceptable levels. This PRD covers Phases 4–5 of the maintenance plan (WP7, WP6) and transforms the tool from a single-file CLI into a well-structured, well-tested Rust library + binary.

## Background

After completing the Stabilization PRD (Phases 1–3), the tool will have proper error handling, no deprecated dependencies, security hardening, and clean identifiers. However, all logic still lives in `main.rs` (no `lib.rs`), functions mix multiple concerns, and test coverage remains at 1 integration test with 0 unit tests.

## Requirements

### Phase 4: Architecture

#### R1 — Module Restructuring (WP7: F17, F19, F20, F21)

1. Create `src/lib.rs` as the public API entry point, re-exporting core types and functions.
2. Extract config loading, merging, and validation into `src/config.rs`.
3. Extract detector logic into `src/detectors/mod.rs` and `src/detectors/file_matching.rs`.
4. Extract Dependabot YAML generation into `src/generator.rs`.
5. Reduce `src/main.rs` to a thin CLI wrapper (< 30 lines) that calls into the library.
6. Split `generate_dependabot_config` into three distinct operations: directory traversal, detection, and YAML generation.
7. Cache compiled `Regex` per detector — compile once at startup, reuse across all directories (F19).

Target source layout:
```
src/
├── main.rs              (CLI entry point only, < 30 lines)
├── lib.rs               (public API re-exports)
├── config.rs            (config loading, merging, validation)
├── detectors/
│   ├── mod.rs           (DetectorType enum, detector trait)
│   └── file_matching.rs (DIRECTORY_HAS_FILE_MATCHING implementation)
└── generator.rs         (Dependabot YAML generation)
```

### Phase 5: Testing

#### R2 — Unit Tests & Coverage (WP6: F6, F22)

8. Add `#[cfg(test)]` modules with unit tests in each source file.
9. Write unit tests for core functions covering happy path and error paths:
   - `detector_has_file_matching`: matching file, no match, empty dir, unreadable dir
   - `run_detector`: valid type, unknown type, missing config keys
   - `generate_dependabot_config`: empty config, single match, nested matches, no matches
   - `load_configs`: default only, extra only, merged, missing file, invalid YAML
10. Add `cargo-tarpaulin` (or `cargo-llvm-cov`) to the CI workflow for coverage reporting.
11. Use the `tempfile` crate for test isolation where filesystem fixtures are needed.

## Acceptance Criteria

### R1 — Module Restructuring
- [ ] `src/main.rs` is under 30 lines
- [ ] Each module has a single responsibility (config, detection, generation)
- [ ] `src/lib.rs` exists and exposes the public API
- [ ] All existing tests pass (smoke test + any tests added during Stabilization)
- [ ] `cargo doc` generates clean documentation without warnings

### R2 — Unit Tests & Coverage
- [ ] Unit tests exist for each core function (config, detectors, generator)
- [ ] Each function has at least one happy-path and one error-path test
- [ ] Code coverage exceeds 70% on core logic (excluding `main.rs` CLI wrapper)
- [ ] CI pipeline reports coverage on every PR
- [ ] All tests pass (`cargo test`)

## Out of Scope

- **WP8 (Default Rules & Examples)**: Deferred — will be scoped in a future PRD after this architecture is in place.
- **WP9 (CLI Enhancements — `--exclude`, `--output`)**: Deferred — will be scoped in a future PRD.
- **F28 (Clean merged remote branches)**: One-time housekeeping, not part of any PRD.
- **F29 (Fuzz testing)**: Deferred until test infrastructure matures.

## Dependencies

This PRD depends on the completion of the Stabilization PRD (Phases 1–3), specifically:

- **WP7 (Module Restructuring)** depends on WP1 (error handling), WP2 (serde migration), and WP4 (naming fixes).
- **WP6 (Unit Tests)** depends on WP1 (error handling), WP4 (naming fixes), and WP7 (module restructuring).

### Internal dependency order

```
WP7 (Module Restructuring) ───→ WP6 (Unit Tests & Coverage)
```

WP7 must complete before WP6 starts, since unit tests should target the final module architecture.
