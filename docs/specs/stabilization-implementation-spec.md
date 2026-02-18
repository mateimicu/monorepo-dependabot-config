# Implementation Spec: Stabilization (Phases 1-3)

**Date**: 2026-02-18
**PRD**: [Stabilization (Phases 1-3)](../prd/stabilization.md)
**Status**: Draft

---

## Context

### Background

`monorepo-dependabot-config` is a Rust CLI (v0.1.3, 189 source lines across 2 files) that generates Dependabot configuration for monorepos. Five independent reviews identified 29 issues. This spec covers the stabilization work (WP1-WP5) that brings the tool from "functional but fragile" to "reliable and secure."

### Current Architecture

```
src/
├── main.rs          (145 lines — all logic: detection, generation, config loading, CLI)
├── strucs.rs        (44 lines — Cli, Detector, Generator, Config structs)
└── default_rules.yaml (empty)
```

**Data flow**: CLI args → `load_configs()` → `generate_dependabot_config()` → stdout YAML

- `main()` parses CLI, loads configs, generates output, prints to stdout
- `load_configs()` reads default rules + optional extra config YAML, merges generators
- `generate_dependabot_config()` walks directory tree, runs detectors per directory, builds YAML
- `run_detector()` dispatches on detector type string (`"DIRECOTRY_HAS_FILE_FILE_MATCHING"`)
- `detector_has_file_matching()` compiles regex per call, checks file names in a directory

**Key issues**: 17 `.unwrap()` calls (panics on any bad input), deprecated `serde_yaml`, string-typed dispatch, typos in identifiers and file names, no input validation, unpinned CI Actions.

### Constraints

- All logic stays in `main.rs` and `strucs.rs` during stabilization. Module restructuring is deferred to the Enhancement PRD (Phase 4).
- The `DIRECOTRY_HAS_FILE_FILE_MATCHING` → `DIRECTORY_HAS_FILE_MATCHING` rename is a hard break (no backward compatibility).
- `serde_yml` is the chosen replacement for `serde_yaml` (API-compatible fork).
- Stabilization must not change public behavior except where explicitly required (error messages instead of panics, identifier rename, security limits).

### Related Resources

- [Stabilization PRD](../prd/stabilization.md)
- [Enhancement PRD](../prd/enhancement.md) (context for forward compatibility)
- [Maintenance Plan](../maintenance/maintenance-plan.md)

---

## Architecture Decisions

### ADR-1: Error Handling Strategy

**Decision**: Use `anyhow` for all error handling in `main.rs`.

**Rationale**: The codebase has 17 `.unwrap()` calls. `anyhow` provides ergonomic `?` + `.context()` without defining custom error types. Custom error types are unnecessary for a CLI application of this size and would be over-engineering. The Enhancement PRD's module restructuring may later introduce `thiserror` for library error types if needed — `anyhow` in the binary layer is compatible with that future.

**Implications**:
- `main()` signature changes to `fn main() -> anyhow::Result<()>`
- All `.unwrap()` calls become `?` with `.context("descriptive message")`
- Functions that can fail (`load_configs`, `generate_dependabot_config`, `run_detector`, `detector_has_file_matching`) return `anyhow::Result<T>` instead of bare `T`
- `run_detector` returns `anyhow::Result<bool>`, not `bool`

### ADR-2: DetectorType as Enum

**Decision**: Replace string-based detector dispatch with a `DetectorType` enum that implements serde `Deserialize`.

**Rationale**: Compile-time safety, exhaustive match, and clearer error messages. The enum lives in `strucs.rs` (soon `structs.rs`) alongside the existing struct definitions. The Enhancement PRD plans to move this into `src/detectors/mod.rs`, so the enum should be defined cleanly for easy relocation.

**Implications**:
- New `DetectorType` enum with variant `DirectoryHasFileMatching`
- Serde rename attribute maps `"DIRECTORY_HAS_FILE_MATCHING"` to the enum variant
- `Detector.type_` field changes from `String` to `DetectorType`
- `run_detector` takes `DetectorType` instead of `String`, uses `match` on enum

### ADR-3: Hard Break for Identifier Rename

**Decision**: `DIRECOTRY_HAS_FILE_FILE_MATCHING` is replaced with `DIRECTORY_HAS_FILE_MATCHING` with no backward compatibility.

**Rationale**: PRD explicitly chooses hard break. The tool is pre-1.0 with limited adoption. Maintaining backward compatibility for a typo adds unnecessary code that would need to be cleaned up later.

**Implications**:
- All source code, examples, README, and test fixtures update to new identifier
- Old config files will fail with a clear serde deserialization error (unknown variant)

### Implementation Notes

- **serde_yaml → serde_yml migration**: Drop-in replacement. `serde_yml` is the community-maintained fork of the deprecated `serde_yaml` with a compatible API. The migration is a find-and-replace of the crate name in `Cargo.toml`, imports, and type references across `main.rs`, `strucs.rs`, and `tests/smoke_test.rs`. Before adoption, the coding expert should verify `serde_yml` legitimacy: check crates.io download counts, maintainer reputation, and the RustSec advisory database for known vulnerabilities.
- **Input validation placement**: Each validation check lives where the relevant data is first available. `search_dir` existence check goes in `main()` (CLI boundary). Config file size check goes in `load_configs()` (before `read_to_string`). Regex length check goes in `detector_has_file_matching()` (before `Regex::new`). Symlink disabling is a `WalkDir` builder option in `generate_dependabot_config()`. This keeps each function responsible for validating its own inputs, which aligns with the Enhancement PRD's planned module extraction.

---

## Component Structure & Interfaces

### File Change Map

| File | Action | WP |
|------|--------|----|
| `Cargo.toml` | Modify (add anyhow, swap serde_yaml→serde_yml, add metadata) | WP1, WP2, WP3, WP5 |
| `src/main.rs` | Modify (error handling, serde migration, security checks, identifier rename) | WP1, WP2, WP3, WP4 |
| `src/strucs.rs` → `src/structs.rs` | Rename + modify (serde migration, DetectorType enum, typo fixes) | WP2, WP4 |
| `tests/smoke_test.rs` | Modify (serde migration, remove debug println) | WP2, WP4 |
| `.github/workflows/rust.yml` | Modify (pin Actions to SHAs) | WP3 |
| `.github/workflows/release.yml` | Modify (pin Actions to SHAs) | WP3 |
| `.github/dependabot.yml` | Create | WP5 |
| `SECURITY.md` | Create | WP3 |
| `.editorconfig` | Create | WP5 |
| `rustfmt.toml` | Create | WP5 |
| `README.md` | Modify ("Quick Star"→"Quick Start", identifier rename) | WP4, WP5 |
| `TODO.md` | Modify (mark completed, remove stale) | WP5 |
| `LICENSE` | Modify (year 2024 → 2024-2026) | WP5 |
| `examples/no-default-config/01-terraform-simple-find/config.yml` | Modify (identifier rename) | WP4 |

### Interface Changes

**Return type changes** (all functions in `src/main.rs` that currently return bare values):

| Function | Current Return | New Return |
|----------|---------------|------------|
| `main()` | `()` | `anyhow::Result<()>` |
| `load_configs()` | `Config` | `anyhow::Result<Config>` |
| `generate_dependabot_config()` | `serde_yaml::Value` | `anyhow::Result<serde_yml::Value>` |
| `run_detector()` | `bool` | `anyhow::Result<bool>` |
| `detector_has_file_matching()` | `bool` | `anyhow::Result<bool>` |

**Parameter type changes**: `run_detector`'s first parameter changes from `String` to `DetectorType` (enum). All `serde_yaml::` types become `serde_yml::`. Exact parameter signatures (borrow vs. owned, etc.) are left to the coding expert's discretion.

#### `src/structs.rs` (renamed from `strucs.rs`) — New Type

```rust
#[derive(Deserialize, Serialize, Debug)]
pub enum DetectorType {
    #[serde(rename = "DIRECTORY_HAS_FILE_MATCHING")]
    DirectoryHasFileMatching,
}

pub struct Detector {
    #[serde(rename = "type")]
    pub type_: DetectorType, // was: String
    pub config: serde_yml::Value,
}
```

### Constants / Limits

| Constant | Value | Location | Purpose |
|----------|-------|----------|---------|
| Max regex pattern length | 1024 chars | `detector_has_file_matching` | Defense-in-depth input bound |
| Max config file size | 1 MB (1_048_576 bytes) | `load_configs` | Memory exhaustion prevention |

Note: Rust's `regex` crate guarantees linear-time matching (no backtracking), so ReDoS is not a risk with the default engine. The 1024-char limit is defense-in-depth against unexpectedly large patterns, not a primary ReDoS mitigation.

---

## Integration Points

### Dependency Changes (`Cargo.toml`)

| Dependency | Action | Notes |
|------------|--------|-------|
| `anyhow` | Add | Error handling for WP1 |
| `serde_yaml` | Remove | Deprecated |
| `serde_yml` | Add | API-compatible replacement |

All other dependencies (`clap`, `env_logger`, `log`, `regex`, `serde`, `walkdir`) remain unchanged.

### CI Workflow Changes

**GitHub Actions to pin** (both `rust.yml` and `release.yml`):

| Action | Current | Pin to SHA |
|--------|---------|------------|
| `actions/checkout` | `@v4` | Look up latest v4 SHA at implementation time |
| `dtolnay/rust-toolchain` | `@stable` | Look up latest SHA at implementation time |
| `Swatinem/rust-cache` | `@v2` | Look up latest v2 SHA at implementation time |
| `MarcoIeni/release-plz-action` | `@v0.5` | Look up latest v0.5 SHA at implementation time |

Format: `uses: owner/repo@<full-sha> # v<tag>` — comment preserves human-readable version.

### Example Config Changes

The example config at `examples/no-default-config/01-terraform-simple-find/config.yml` uses `DIRECOTRY_HAS_FILE_FILE_MATCHING`. This must be updated to `DIRECTORY_HAS_FILE_MATCHING` as part of WP4.

---

## Data Flow & State Management

### Current Data Flow (unchanged by stabilization)

```
CLI args (clap)
  │
  ▼
main()
  ├─→ validate search_dir [NEW: WP3]
  ├─→ env_logger::init() [FIXED: before first log, WP1]
  ├─→ load_configs(enable_default_rules, extra_config_file)
  │     ├─→ check file size [NEW: WP3]
  │     ├─→ read_to_string + serde_yml::from_str [CHANGED: WP2]
  │     └─→ return Result<Config> [CHANGED: WP1]
  ├─→ generate_dependabot_config(config, search_dir)
  │     ├─→ WalkDir with follow_links(false) [NEW: WP3]
  │     ├─→ for each dir: run_detector(type, config, path)
  │     │     ├─→ match on DetectorType enum [CHANGED: WP4]
  │     │     └─→ detector_has_file_matching(path, regex)
  │     │           ├─→ check regex length [NEW: WP3]
  │     │           └─→ Regex::new + file scan
  │     └─→ return Result<serde_yml::Value> [CHANGED: WP1, WP2]
  └─→ println serde_yml::to_string [CHANGED: WP2]
```

### State

The application is stateless — no persistent state, no caches (regex caching deferred to Enhancement PRD). All data flows through function parameters and return values.

---

## Test Strategy

### Existing Tests (must continue passing)

- `tests/smoke_test.rs::execute_examples_with_no_default_config` — Integration test that runs the binary against example fixtures and compares YAML output.

### New Tests Required by Stabilization

No new test files are created during stabilization. The existing smoke test validates that the tool still works end-to-end after all changes. The Enhancement PRD (Phase 5) will add unit tests.

However, the acceptance criteria require **manual verification** or **CI validation** of specific behaviors:

| Requirement | Verification Method |
|-------------|-------------------|
| Nonexistent directory → human-readable error | Run binary manually or add a shell-based integration test |
| Invalid config file → human-readable error | Run binary manually or add a shell-based integration test |
| Regex > 1024 chars → error | Run binary manually or add a shell-based integration test |
| Config file > 1 MB → error | Run binary manually or add a shell-based integration test |
| Symlinks not followed | Not tested (test fixture would need symlinks; deferred to Enhancement) |
| Actions pinned to SHAs | `grep` check in CI or manual review |
| `cargo clippy` clean | Already in CI |
| `cargo fmt --check` | Already in CI |

**Recommendation for coding experts**: If adding quick error-path tests is trivial (e.g., a few lines in `smoke_test.rs` calling the binary with bad args and asserting non-zero exit + stderr contains a message), do so. Otherwise, manual verification is acceptable for stabilization; comprehensive test coverage comes in the Enhancement PRD.

---

## Coding Expert Assignment

### 1 Coding Expert, 2 PRs

The codebase is 189 lines across 2 source files. A single expert avoids all coordination overhead and merge conflicts. The work is split into 2 PRs for reviewability:

**PR 1 — Foundation & Migration (WP5, WP1, WP2)**:
- WP5: Add Cargo.toml metadata, create `.github/dependabot.yml`, fix README typo, update TODO.md, update LICENSE year, add clap help text, create `.editorconfig` and `rustfmt.toml`
- WP1: Add `anyhow`, replace all 17 unwraps with `?` + `.context()`, fix logger init order, change function return types to `anyhow::Result<T>`
- WP2: Replace `serde_yaml` with `serde_yml` across all files (Cargo.toml, main.rs, strucs.rs, smoke_test.rs)

**PR 2 — Security & Quality (WP3, WP4)**:
- WP3: Disable symlink following, pin GitHub Actions to SHAs, add regex length limit, add search_dir validation, add config file size limit, create `SECURITY.md`
- WP4: Rename `strucs.rs` → `structs.rs`, fix comment typos, rename `DIRECOTRY_HAS_FILE_FILE_MATCHING` → `DIRECTORY_HAS_FILE_MATCHING`, create `DetectorType` enum, remove debug `println!` from tests, update examples and README

PR 2 depends on PR 1 (WP3 needs WP1's error handling; WP4 needs WP2's serde migration).

### Recommended Execution Order

Within each PR, the expert should apply changes in WP order (WP5 → WP1 → WP2, then WP3 → WP4) and run `cargo test` after each WP to catch regressions early.

---

## Acceptance Criteria (Technical)

These are the verifiable technical criteria. Each maps to a PRD acceptance criterion.

### WP1 — Error Handling & Logger Fix
- [ ] `anyhow` is in `Cargo.toml` `[dependencies]`
- [ ] Zero `.unwrap()` calls remain in `src/main.rs` (verify: `grep -c '.unwrap()' src/main.rs` returns 0)
- [ ] `main()` returns `anyhow::Result<()>`
- [ ] `env_logger::init()` is called before any `log::` macro
- [ ] `cargo clippy -- -D warnings` passes
- [ ] Existing smoke test passes (`cargo test`)

### WP2 — serde_yaml Migration
- [ ] `serde_yaml` does not appear in `Cargo.toml`
- [ ] `serde_yml` is in `Cargo.toml` `[dependencies]`
- [ ] Zero `serde_yaml::` references in source code (verify: `grep -r 'serde_yaml' src/ tests/` returns nothing)
- [ ] `cargo test` passes

### WP3 — Security Hardening
- [ ] `WalkDir::new(...)` call includes `.follow_links(false)`
- [ ] All `uses:` lines in `.github/workflows/*.yml` reference full 40-character SHAs
- [ ] Regex pattern > 1024 chars returns `Err` (not panic)
- [ ] Nonexistent `search_dir` argument returns `Err` before entering business logic
- [ ] Config file > 1 MB returns `Err` in `load_configs`
- [ ] `SECURITY.md` exists at repo root

### WP4 — Naming Fixes & Type Safety
- [ ] `src/structs.rs` exists; `src/strucs.rs` does not
- [ ] `mod structs;` in `main.rs` (not `mod strucs;`)
- [ ] `DetectorType` is an enum in `structs.rs`
- [ ] `Detector.type_` field is `DetectorType`, not `String`
- [ ] Zero occurrences of `DIRECOTRY` in any source file, example, or README
- [ ] Zero occurrences of `cofiguration`, `pachages`, `appropiate`, `recursevely` in comments
- [ ] Zero `println!` calls in `tests/smoke_test.rs`
- [ ] `cargo test` passes

### WP5 — Project Metadata & Documentation
- [ ] `Cargo.toml` has `repository`, `homepage`, `keywords`, `categories`, `readme` fields
- [ ] `.github/dependabot.yml` exists with `package-ecosystem: cargo`
- [ ] README has "Quick Start" (not "Quick Star")
- [ ] `LICENSE` year reads "2024-2026"
- [ ] `.editorconfig` exists at repo root
- [ ] `rustfmt.toml` exists at repo root
- [ ] Clap `#[arg()]` attributes have `help` text
- [ ] `cargo run -- --help` output includes descriptive help text
