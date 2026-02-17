# PRD: Stabilization (Phases 1–3)

**Date**: 2026-02-18
**Status**: Draft
**Companion PRD**: [Enhancement (Phases 4–5)](enhancement.md)
**Source**: [Maintenance Plan](../maintenance/maintenance-plan.md)

## Goal

Stabilize `monorepo-dependabot-config` by addressing critical reliability issues, deprecated dependencies, security gaps, and code quality problems identified across five independent reviews. This PRD covers Phases 1–3 of the maintenance plan (WP1–WP5) and brings the tool from "functional but fragile" to "reliable and secure."

## Background

The tool is a Rust CLI (v0.1.3, 189 source lines) that generates Dependabot configuration for monorepos. Five reviews found 29 issues. The tool currently panics on any bad input (17 `.unwrap()` calls), depends on a deprecated YAML library, has no input validation, and has identifier typos that affect config file compatibility.

## Requirements

### Phase 1: Foundation

#### R1 — Error Handling & Logger Fix (WP1: F1, F5)

1. Add `anyhow` as a dependency for ergonomic error handling.
2. Replace all 17 `.unwrap()` calls in `src/main.rs` with the `?` operator and `.context()` messages.
3. Change `main()` to return `anyhow::Result<()>`.
4. Fix logger initialization order: move `env_logger::init()` before the first `log::debug!` call (`main.rs:137-138`).

#### R2 — Project Metadata & Documentation (WP5: F12, F13, F14, F26, F27)

5. Add missing fields to `Cargo.toml`: `repository`, `homepage`, `keywords`, `categories`, `readme`.
6. Create `.github/dependabot.yml` with cargo ecosystem configuration for this repository.
7. Fix README typo: "Quick Star" → "Quick Start".
8. Update `TODO.md`: mark completed items, remove stale entries.
9. Update `LICENSE` year range to 2024-2026.
10. Add help text and examples to clap `#[arg()]` attributes in the CLI struct.
11. Add `.editorconfig` and `rustfmt.toml` for consistent formatting.

### Phase 2: Dependency Health & Security

#### R3 — serde_yaml Migration (WP2: F2)

12. Replace the deprecated `serde_yaml` crate with `serde_yml` (API-compatible fork).
13. Update all imports and usage across `src/main.rs`, `src/strucs.rs`, and `tests/smoke_test.rs`.
14. Update `Cargo.lock` to reflect the new dependency.

#### R4 — Security Hardening (WP3: F7, F8, F9, F10, F11, F15)

15. Disable symlink following: add `.follow_links(false)` to `WalkDir::new()` (`src/main.rs:68`).
16. Pin all GitHub Actions in `.github/workflows/*.yml` to full commit SHAs (with version comments).
17. Add regex pattern length limit (1024 chars max) before `Regex::new()` to prevent ReDoS.
18. Validate `search_dir` CLI argument: check that the path exists and is a directory early in `main()`.
19. Add config file size limit (1 MB max) before `read_to_string`.
20. Create `SECURITY.md` with vulnerability reporting instructions.

### Phase 3: Code Quality

#### R5 — Naming Fixes & Type Safety (WP4: F4, F18, F23)

21. Rename `src/strucs.rs` → `src/structs.rs` and update the `mod` declaration.
22. Fix all comment typos: `cofiguration`, `pachages`, `appropiate`, `recursevely`.
23. Replace `DIRECOTRY_HAS_FILE_FILE_MATCHING` with `DIRECTORY_HAS_FILE_MATCHING` (hard break — no backward compatibility for the typo'd identifier).
24. Create a `DetectorType` enum with serde deserialization to replace string-based dispatch.
25. Remove debug `println!` lines from `tests/smoke_test.rs:33-34`.
26. Update README and example configs to use the corrected identifier.

## Acceptance Criteria

### R1 — Error Handling & Logger Fix
- [ ] `cargo clippy` passes with no warnings
- [ ] Running with a nonexistent directory prints a human-readable error (not a panic)
- [ ] Running with an invalid config file prints a human-readable error (not a panic)
- [ ] Existing smoke test passes

### R2 — Project Metadata & Documentation
- [ ] `cargo package --list` shows correct metadata fields
- [ ] `cargo run -- --help` shows descriptive help text with usage examples
- [ ] `.github/dependabot.yml` exists and is valid
- [ ] `.editorconfig` and `rustfmt.toml` exist

### R3 — serde_yaml Migration
- [ ] No deprecated crates in dependency tree (`serde_yaml` fully removed)
- [ ] All existing tests pass
- [ ] `cargo audit` reports no advisories

### R4 — Security Hardening
- [ ] Symlinks in test directories are not followed
- [ ] All GitHub Actions are pinned to full commit SHAs (`grep -v '@[a-f0-9]\{40\}'` returns no unpinned actions)
- [ ] Regex pattern longer than 1024 chars returns an error (not a panic)
- [ ] Nonexistent `search_dir` returns a descriptive error
- [ ] Config file larger than 1 MB is rejected with an error
- [ ] `SECURITY.md` exists with reporting instructions

### R5 — Naming Fixes & Type Safety
- [ ] No typos in source code identifiers or comments
- [ ] `DetectorType` is an enum, not a `String`
- [ ] `src/structs.rs` exists (not `strucs.rs`)
- [ ] README and examples reference `DIRECTORY_HAS_FILE_MATCHING` only
- [ ] All tests pass

## Out of Scope

- **WP6 (Unit Tests & Coverage)**: Covered in the Enhancement PRD (Phases 4–5).
- **WP7 (Module Restructuring)**: Covered in the Enhancement PRD (Phases 4–5).
- **WP8 (Default Rules & Examples)**: Deferred — will be scoped in a future PRD.
- **WP9 (CLI Enhancements — `--exclude`, `--output`)**: Deferred — will be scoped in a future PRD.
- **F28 (Clean merged remote branches)**: One-time housekeeping, not part of any PRD.
- **F29 (Fuzz testing)**: Deferred until test infrastructure matures.
- **Backward compatibility for `DIRECOTRY_HAS_FILE_FILE_MATCHING`**: Hard break chosen. Existing config files using the typo'd identifier will need to be updated.

## Dependencies

This PRD has no external dependencies. The Enhancement PRD (Phases 4–5) depends on the completion of this PRD.

### Internal dependency order

```
WP1 (Error Handling) ───→ WP3 (Security Hardening)
WP2 (serde_yaml) ───────→ WP4 (Naming & Type Safety)
WP5 (Metadata) ─────────→ (no downstream dependencies in this PRD)
```

WP1 and WP2 and WP5 can proceed in parallel. WP3 depends on WP1. WP4 depends on WP2.
