# Maintenance Plan — monorepo-dependabot-config

**Date**: 2026-02-17
**Branch**: `maintenance/recommendations-plan` (stacked on `maintenance/dependency-updates`)
**Version reviewed**: 0.1.3

## Executive Summary

Five independent reviews (maintenance, security, database/I/O, testing, architecture) were conducted on monorepo-dependabot-config, a Rust CLI tool that generates Dependabot configuration for monorepos.

**Health Assessment**: The tool is functional for its primary use case (terraform detection via custom config) but has significant gaps in reliability, security, and test coverage. The codebase is small (189 lines across 2 source files) which makes remediation straightforward.

| Metric | Current State |
|--------|---------------|
| Source lines | 189 (main.rs: 145, strucs.rs: 44) |
| Dependencies | 7 direct (1 deprecated: serde_yaml) |
| Test coverage | 1 integration test, 0 unit tests |
| .unwrap() calls | 17 (all in main.rs — crash on any bad input) |
| Default rules | Empty (core feature non-functional) |
| Typos in identifiers | 2 (DIRECOTRY, strucs) |
| Typos in comments | 4 (cofiguration, pachages, appropiate, recursevely) |

**Consolidated findings**: 29 unique items de-duplicated from 5 reports, organized into 9 work packages across 6 phases.

**Estimated total effort**: ~40-60 hours of focused development work.

---

## Findings Index

Each finding has a unique ID (F1-F29) for cross-referencing. The "Reports" column shows which of the 5 reviews flagged it (1=Maintenance, 2=Security, 3=Database/IO, 4=Testing, 5=Architecture).

### Critical Priority

These items affect basic reliability or represent active risks.

| ID | Finding | Reports | Effort | Files |
|----|---------|---------|--------|-------|
| F1 | **Replace 17 `.unwrap()` calls with proper error handling.** Panics on any malformed input, unreadable directory, or invalid regex. Use `anyhow` for application errors with context. | 1,2,3,4,5 | Medium | `src/main.rs`, `Cargo.toml` |
| F2 | **Migrate from deprecated `serde_yaml`.** The `serde_yaml` crate (0.9.x) is unmaintained. Evaluate `serde_yml` as drop-in replacement. | 1,2 | Medium | `Cargo.toml`, `src/main.rs`, `src/strucs.rs`, `tests/smoke_test.rs` |

### High Priority

High-impact items that should follow the critical fixes.

| ID | Finding | Reports | Effort | Files |
|----|---------|---------|--------|-------|
| F3 | **Populate `default_rules.yaml`.** The `--enable-default-rules` flag exists but the file is empty. Core advertised feature is non-functional. 22 ecosystems listed in `docs/default-rules-support.md`. | 1,3,4,5 | Large | `src/default_rules.yaml`, `examples/`, `tests/` |
| F4 | **Fix typos in code identifiers.** `DIRECOTRY_HAS_FILE_FILE_MATCHING` → `DIRECTORY_HAS_FILE_MATCHING` and `strucs.rs` → `structs.rs`. The detector string is a breaking change for existing config files — needs migration path (accept both, warn on old). | 1,5 | Small | `src/strucs.rs` → `src/structs.rs`, `src/main.rs`, `README.md`, `examples/` |
| F5 | **Fix logger initialization order.** `main.rs:137` calls `log::debug!` before `env_logger::init()` on line 138. Swap the two lines. | 1,5 | Small | `src/main.rs:135-138` |
| F6 | **Add unit tests for core functions.** Test health is 4/10. Zero unit tests exist. Functions to cover: `detector_has_file_matching`, `run_detector`, `generate_dependabot_config`, `load_configs`. | 4,5 | Medium | `src/main.rs` (extract to `lib.rs`), `tests/` |
| F7 | **Disable symlink following in `walkdir`.** Default behavior follows symlinks, enabling directory traversal attacks and infinite loops. Add `.follow_links(false)` to `WalkDir::new()`. | 2 | Small | `src/main.rs:68` |

### Medium Priority

Improvements to security, code quality, and developer experience.

| ID | Finding | Reports | Effort | Files |
|----|---------|---------|--------|-------|
| F8 | **Pin GitHub Actions to commit SHAs.** Current tags (`@v4`, `@v2`, `@stable`, `@v0.5`) are mutable — supply chain risk. Pin to full commit SHA with version comment. | 2 | Small | `.github/workflows/rust.yml`, `.github/workflows/release.yml` |
| F9 | **Add ReDoS protection.** User-supplied regex patterns (via config YAML) have no complexity or size limits. Add length cap and compilation timeout or use `regex` crate's built-in size limit. | 2 | Medium | `src/main.rs:13` |
| F10 | **Validate `search_dir` argument.** No check that the path exists, is a directory, or is accessible. Add validation in CLI parsing or early in `main()`. | 2 | Small | `src/main.rs` or `src/strucs.rs` |
| F11 | **Add YAML file size limits.** No bounds on config file size before parsing. Add a size check before `read_to_string`. | 2 | Small | `src/main.rs:125` |
| F12 | **Add Cargo.toml metadata.** Missing: `repository`, `homepage`, `keywords`, `categories`, `readme`. Needed for crates.io discoverability. | 1 | Small | `Cargo.toml` |
| F13 | **Add `dependabot.yml` for the repo itself.** A tool that generates Dependabot configs should use Dependabot. | 1 | Small | `.github/dependabot.yml` (new) |
| F16 | **Populate empty example directories.** `examples/only-default-config/` and `examples/with-default-config/` are empty placeholders. | 1,4 | Medium | `examples/` |
| F17 | **Restructure into modules.** All logic lives in 2 files. Extract into: `lib.rs`, `config/`, `detectors/`, `generator/`. | 5 | Large | `src/` |
| F18 | **Convert detector type from `String` to enum.** Replace string-matching dispatch with a typed `DetectorType` enum for compile-time safety. | 5 | Small | `src/strucs.rs`, `src/main.rs` |
| F20 | **Separate binary and library.** No `lib.rs` — all logic in `main.rs`. Prevents unit testing and reuse as a library. | 5 | Medium | `src/main.rs` → `src/lib.rs` + `src/main.rs` |
| F21 | **Extract mixed concerns from `generate_dependabot_config`.** Function (lines 57-111) handles traversal, detection, YAML generation, and path manipulation in one block. | 5 | Medium | `src/main.rs:57-111` |
| F22 | **Add CI code coverage reporting.** Integrate `cargo-tarpaulin` or `cargo-llvm-cov` into CI workflow. | 4 | Small | `.github/workflows/rust.yml`, `Cargo.toml` |
| F24 | **Add directory exclusion patterns.** No way to skip `node_modules/`, `target/`, `.git/`, etc. Large monorepos will be slow. | 5 | Medium | `src/main.rs`, `src/strucs.rs` |

### Low Priority

Polish, documentation, and nice-to-have improvements.

| ID | Finding | Reports | Effort | Files |
|----|---------|---------|--------|-------|
| F14 | **Fix documentation.** README "Quick Star" → "Quick Start". Update TODO.md (clippy/fmt items are done). LICENSE year 2024 → 2024-2026. | 1 | Small | `README.md`, `TODO.md`, `LICENSE` |
| F15 | **Add SECURITY.md.** Document security considerations and vulnerability reporting process. | 2 | Small | `SECURITY.md` (new) |
| F19 | **Cache regex compilation.** `Regex::new()` is called per directory per detector. Compile once, reuse. Minor perf win for large repos. | 5 | Small | `src/main.rs:13` |
| F23 | **Remove debug `println!` from tests.** `smoke_test.rs:33-34` has debug output that clutters test runs. | 4 | Small | `tests/smoke_test.rs:33-34` |
| F25 | **Add `--output` flag.** Currently stdout-only. Offer optional file output. | 3,5 | Small | `src/main.rs`, `src/strucs.rs` |
| F26 | **Add CLI help examples.** No `#[arg(help = "...")]` examples in clap attributes. | 5 | Small | `src/strucs.rs` |
| F27 | **Add developer config files.** `.editorconfig`, `rustfmt.toml`, `clippy.toml` for consistent formatting. | 1 | Small | Project root (new files) |
| F28 | **Clean merged remote branches.** Housekeeping task. | 1 | Small | Git operations only |
| F29 | **Add fuzz testing.** `cargo-fuzz` for regex parsing and YAML deserialization. | 2 | Medium | `fuzz/` (new) |

---

## Dependency Graph

```
F1 (error handling) ──────┬──→ F9  (ReDoS protection)
                          ├──→ F10 (path validation)
                          ├──→ F11 (YAML size limits)
                          ├──→ F6  (unit tests)
                          └──→ F17 (module restructuring)

F2 (serde_yaml migration) ──→ F4  (typo fixes — struct changes overlap)
                              └──→ F17 (module restructuring)

F4 (typo fixes) ──────────┬──→ F3  (default rules — needs correct identifier)
                           └──→ F18 (detector enum — rename first)

F17 (modules) ─────────────┬──→ F3  (default rules — clean arch needed)
                            ├──→ F24 (exclusion patterns)
                            └──→ F25 (--output flag)

F20 (lib.rs split) ────────→ F6  (unit tests — need public functions)

F3 (default rules) ────────→ F16 (example directories — need rules first)
```

---

## Implementation Roadmap

### Phase 1: Foundation

**Goal**: Fix critical reliability issues and project metadata.

#### WP1 — Error Handling & Logger Fix
- **Findings**: F1, F5
- **Files**: `src/main.rs`, `Cargo.toml`
- **Changes**:
  - Add `anyhow` dependency for ergonomic error handling
  - Replace all 17 `.unwrap()` calls with `?` operator and `.context()` messages
  - Change `main()` to return `anyhow::Result<()>`
  - Swap `env_logger::init()` before the first `log::debug!` call
- **Acceptance criteria**:
  - `cargo clippy` passes with no warnings
  - Running with a nonexistent directory prints a human-readable error (not a panic)
  - Running with an invalid config file prints a human-readable error
  - Existing smoke test still passes

#### WP5 — Project Metadata & Documentation
- **Findings**: F12, F13, F14, F26, F27
- **Files**: `Cargo.toml`, `.github/dependabot.yml`, `README.md`, `TODO.md`, `LICENSE`, `src/strucs.rs`, project root
- **Changes**:
  - Add `repository`, `homepage`, `keywords`, `categories`, `readme` to Cargo.toml
  - Create `.github/dependabot.yml` with cargo ecosystem config
  - Fix "Quick Star" → "Quick Start" in README
  - Mark completed TODO items, remove stale ones
  - Update LICENSE year to 2024-2026
  - Add help text and examples to clap `#[arg()]` attributes
  - Add `.editorconfig` and `rustfmt.toml`
- **Acceptance criteria**:
  - `cargo package --list` shows correct metadata
  - `cargo run -- --help` shows descriptive help with examples

### Phase 2: Dependency Health & Security

**Goal**: Address deprecated dependencies and security gaps. Depends on Phase 1 (WP1).

#### WP2 — serde_yaml Migration
- **Findings**: F2
- **Files**: `Cargo.toml`, `src/main.rs`, `src/strucs.rs`, `tests/smoke_test.rs`
- **Changes**:
  - Evaluate `serde_yml` as drop-in replacement (API-compatible fork)
  - Replace `serde_yaml` with chosen alternative in all files
  - Update `Cargo.lock`
- **Acceptance criteria**:
  - No deprecated crates in dependency tree
  - All existing tests pass
  - `cargo audit` clean (no advisories)

#### WP3 — Security Hardening
- **Findings**: F7, F8, F9, F10, F11, F15
- **Depends on**: WP1 (error handling needed for validation error paths)
- **Files**: `src/main.rs`, `.github/workflows/*.yml`, `SECURITY.md`
- **Changes**:
  - Add `.follow_links(false)` to `WalkDir::new()` (F7)
  - Pin all GitHub Actions to commit SHAs with version comments (F8)
  - Add regex pattern length limit (e.g., 1024 chars) before `Regex::new()` (F9)
  - Validate `search_dir` exists and is a directory early in `main()` (F10)
  - Add config file size limit (e.g., 1MB) before `read_to_string` (F11)
  - Create SECURITY.md with reporting instructions (F15)
- **Acceptance criteria**:
  - Symlinks in test directory are not followed
  - All GitHub Actions pinned to SHA (verify with `grep -v '@[a-f0-9]\{40\}'` returns nothing)
  - Regex pattern > 1024 chars returns error (not panic)
  - Nonexistent search_dir returns descriptive error

### Phase 3: Code Quality

**Goal**: Fix naming issues and improve type safety. Depends on Phase 2 (WP2).

#### WP4 — Naming Fixes & Type Safety
- **Findings**: F4, F18, F23
- **Depends on**: WP2 (serde changes may overlap with struct file changes)
- **Files**: `src/strucs.rs` → `src/structs.rs`, `src/main.rs`, `tests/smoke_test.rs`, `README.md`, `examples/`
- **Changes**:
  - Rename `src/strucs.rs` → `src/structs.rs`, update `mod` declaration
  - Fix all comment typos (cofiguration, pachages, appropiate, recursevely)
  - Replace `DIRECOTRY_HAS_FILE_FILE_MATCHING` with `DIRECTORY_HAS_FILE_MATCHING`
    - Accept both old and new strings during transition, log deprecation warning for old
  - Create `DetectorType` enum with serde deserialization
  - Remove `println!` debug lines from `smoke_test.rs:33-34`
  - Update README and example configs to use corrected identifier
- **Acceptance criteria**:
  - No typos in source code identifiers or comments
  - Old config files using `DIRECOTRY_HAS_FILE_FILE_MATCHING` still work (with deprecation warning)
  - `DetectorType` is an enum, not a String
  - All tests pass

### Phase 4: Architecture

**Goal**: Improve code organization for maintainability. Depends on Phases 1-3.

#### WP7 — Module Restructuring
- **Findings**: F17, F20, F21, F19
- **Depends on**: WP1, WP2, WP4
- **Files**: `src/` (full restructure)
- **Target structure**:
  ```
  src/
  ├── main.rs          (CLI entry point only)
  ├── lib.rs           (public API re-exports)
  ├── config.rs        (Config loading, merging, validation)
  ├── detectors/
  │   ├── mod.rs       (DetectorType enum, trait)
  │   └── file_matching.rs  (DIRECTORY_HAS_FILE_MATCHING impl)
  └── generator.rs     (Dependabot YAML generation)
  ```
- **Changes**:
  - Extract all logic from `main.rs` into `lib.rs` modules
  - `main.rs` becomes a thin CLI wrapper
  - Split `generate_dependabot_config` into: traversal, detection, generation
  - Cache compiled `Regex` per detector (compile once, reuse across directories) (F19)
  - Each detector type gets its own module file
- **Acceptance criteria**:
  - `main.rs` is < 30 lines
  - Each module has a single responsibility
  - All existing tests pass
  - `cargo doc` generates clean documentation

### Phase 5: Testing

**Goal**: Bring test coverage to acceptable levels. Depends on Phase 4 (WP7).

#### WP6 — Unit Tests & Coverage
- **Findings**: F6, F22
- **Depends on**: WP1, WP4, WP7 (test the final architecture)
- **Files**: `src/` (inline unit tests), `tests/`, `.github/workflows/rust.yml`
- **Changes**:
  - Add `#[cfg(test)]` module in each source file with unit tests
  - Test cases for each core function (happy path + error paths)
  - Add `cargo-tarpaulin` to CI for coverage reporting
  - Use `tempfile` crate for test isolation where needed
- **Target test cases**:
  - `detector_has_file_matching`: matching file, no match, empty dir, unreadable dir
  - `run_detector`: valid type, unknown type, missing config keys
  - `generate_dependabot_config`: empty config, single match, nested matches, no matches
  - `load_configs`: default only, extra only, merged, missing file, invalid YAML
- **Acceptance criteria**:
  - Test health score >= 7/10
  - Coverage > 70% on core logic
  - CI reports coverage on every PR

### Phase 6: Feature Completion

**Goal**: Deliver the promised default rules feature and CLI improvements.

#### WP8 — Default Rules & Examples
- **Findings**: F3, F16
- **Depends on**: WP4, WP7
- **Files**: `src/default_rules.yaml`, `examples/`, `tests/`, `docs/default-rules-support.md`
- **Changes**:
  - Populate `default_rules.yaml` starting with high-value ecosystems:
    - Tier 1: npm (package.json), pip (requirements.txt), Cargo (Cargo.toml), Go (go.mod), Docker (Dockerfile), Terraform (.tf)
    - Tier 2: Maven (pom.xml), Gradle (build.gradle), Composer (composer.json), Bundler (Gemfile)
    - Tier 3: Remaining ecosystems from `docs/default-rules-support.md`
  - Populate `examples/only-default-config/` with test fixtures
  - Populate `examples/with-default-config/` with merged config test fixtures
  - Add integration tests for each tier
  - Update `docs/default-rules-support.md` checklist
- **Acceptance criteria**:
  - `--enable-default-rules` produces valid Dependabot config for all Tier 1 ecosystems
  - Each example directory has config, expected output, and working-dir fixtures
  - All integration tests pass

#### WP9 — CLI Enhancements
- **Findings**: F24, F25
- **Depends on**: WP7
- **Files**: `src/main.rs`, `src/structs.rs` (or equivalent after restructure)
- **Changes**:
  - Add `--exclude` flag for directory exclusion patterns (glob-based)
  - Default exclusions: `.git`, `node_modules`, `target`, `vendor`, `__pycache__`
  - Add `--output` flag for writing to file instead of stdout
- **Acceptance criteria**:
  - `--exclude node_modules` skips all `node_modules/` directories
  - `--output dependabot.yml` writes file and prints confirmation
  - Help text documents both new flags with examples

---

## Work Package Summary

| WP | Phase | Name | Findings | Effort | Depends On |
|----|-------|------|----------|--------|------------|
| WP1 | 1 | Error Handling & Logger Fix | F1, F5 | Medium | — |
| WP5 | 1 | Metadata & Documentation | F12-F14, F26-F27 | Small | — |
| WP2 | 2 | serde_yaml Migration | F2 | Medium | — |
| WP3 | 2 | Security Hardening | F7-F11, F15 | Medium | WP1 |
| WP4 | 3 | Naming Fixes & Type Safety | F4, F18, F23 | Small | WP2 |
| WP7 | 4 | Module Restructuring | F17, F19-F21 | Large | WP1, WP2, WP4 |
| WP6 | 5 | Unit Tests & Coverage | F6, F22 | Medium | WP1, WP4, WP7 |
| WP8 | 6 | Default Rules & Examples | F3, F16 | Large | WP4, WP7 |
| WP9 | 6 | CLI Enhancements | F24, F25 | Medium | WP7 |

Not scheduled (deferred): F28 (clean branches — one-time housekeeping), F29 (fuzz testing — after test infra matures).

---

## Review Sources

| # | Review Type | Reviewer Focus | Key Findings |
|---|-------------|---------------|--------------|
| 1 | Maintenance | Dependency health, project hygiene, documentation | serde_yaml deprecation, empty default rules, typos, metadata gaps |
| 2 | Security | Vulnerability assessment, supply chain, input validation | .unwrap() panics, symlink traversal, ReDoS, unpinned Actions |
| 3 | Database/IO | Data persistence, I/O patterns | Confirmed stateless CLI; flagged error handling and --output flag |
| 4 | Testing | Test coverage, test quality, CI integration | 4/10 health score, 0 unit tests, missing example coverage |
| 5 | Architecture | Code structure, design patterns, scalability | Flat modules, string-typed dispatch, mixed concerns, no lib.rs |
