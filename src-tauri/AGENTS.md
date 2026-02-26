# AGENTS.md

## Purpose

This repository is a **Tauri + Rust** desktop application.

Primary goals for any change:
1. Keep architecture simple.
2. Keep code clean and readable.
3. Prefer explicit, maintainable solutions over “clever” abstractions.
4. Preserve Rust/Tauri idioms and security boundaries.

This file is for AI coding agents. It contains actionable project rules, commands, and constraints.

---

## Working style for this project

The author comes from OOP background and values:
- clean code,
- top-down readability,
- clear names,
- single responsibility,
- minimal accidental complexity.

When making changes, preserve those qualities, but implement them in **idiomatic Rust**:
- prefer **modules + structs + traits** over inheritance patterns,
- prefer composition over deep abstraction trees,
- avoid introducing architecture layers “for future use” unless needed now.

Do not over-engineer.

---

## Setup and validation commands

Run these before finishing a task (for Rust changes inside `src-tauri`):

- Format: `cargo fmt`
- Lint: `cargo clippy --all-targets --all-features -- -D warnings`
- Tests: `cargo test`

If the change touches frontend too, also run the frontend checks defined by the project (read `package.json` scripts).

If a command fails, fix the issue instead of ignoring it.

---

## GitHub workflows and releases

Current workflows in this repository:
- `.github/workflows/ci.yml` - runs checks on `push` to `main` and on `pull_request`.
- `.github/workflows/release.yml` - publishes desktop bundles on tag push `v*` and supports manual `workflow_dispatch`.

Release behavior:
- Regular commits and `push` to `main` do not create GitHub Release assets.
- Release assets are created only by `release.yml` when pushing a version tag like `v0.2.0`.
- Matrix targets include `ubuntu-24.04`, `windows-2022`, `macos-13`, `macos-14`.

How to ship a release:
1. Bump app version in `src-tauri/tauri.conf.json` (and aligned project version files if needed).
2. Commit and push branch changes.
3. Create and push tag: `git tag vX.Y.Z && git push origin vX.Y.Z`.
4. Wait for `Release` workflow to finish, then verify assets in GitHub `Releases`.

Windows artifacts:
- Expected artifacts are published under the GitHub Release `Assets` section (typically `.msi`, optionally `.exe` depending on Tauri bundle/update configuration).

---

## Where to put code

Use this structure (or the closest existing equivalent) in `src-tauri/src`:

- `commands/` — Tauri commands (`#[tauri::command]`), thin adapters only
- `application/` — use-cases / services (main application logic)
- `domain/` — domain types and rules (only if needed)
- `infrastructure/` — filesystem, HTTP, DB, OS/Tauri plugin integrations
- `state/` — shared app state
- `error/` — application error types and mapping
- `lib.rs` — app wiring (register commands, setup state)
- `main.rs` — minimal entrypoint only

### Important boundary rule
Tauri command functions are **not** the place for business logic.

A command should:
1. parse/receive input,
2. call an application service,
3. return result/error.

Keep Tauri-specific types (`AppHandle`, `Window`, `State`, plugin handles) out of domain logic.

---

## Rust code style rules

## 1) Readability first
Write code so it reads top-down:
- public API/types first,
- impl blocks next,
- internal helper functions after,
- tests at the bottom.

## 2) Naming
- Use full descriptive names.
- Do not abbreviate variable names.
- Avoid vague names like `helper`, `utils`, `manager`, `data`.

## 3) Functions
- Keep functions small and focused.
- One function = one clear responsibility.
- If a function has too many parameters, introduce a parameter struct.

## 4) Error handling
- Use `Result<T, E>` for expected failures.
- Use `?` to propagate errors.
- Avoid `panic!`, `unwrap()`, and `expect()` in production code.
- `unwrap()/expect()` are acceptable in tests or in truly impossible states with a clear comment.

## 5) Traits and abstractions
- Do not introduce traits “just in case”.
- Add a trait only when there is a real need (test seam, multiple implementations, boundary).
- Prefer concrete types until abstraction becomes necessary.

## 6) Comments
- Prefer self-explanatory code.
- Add comments only for non-obvious decisions, invariants, or platform/Tauri constraints.

---

## Tauri-specific rules

## 1) Commands
- Keep commands thin.
- Prefer `async` commands for I/O work.
- Return typed results and map errors at the command boundary.

## 2) State
- Use Tauri-managed state for shared app resources.
- Store only true shared state (services, caches, app-wide context).
- Do not store request-local temporary data in global state.

## 3) Security and permissions
Tauri security model matters:
- follow least privilege,
- do not enable permissions/capabilities “just in case”,
- scope access to the required windows/webviews only.

If a task adds a plugin, command, or privileged capability:
- update permissions/capabilities intentionally,
- keep scope minimal,
- mention the change in the final summary.

## 4) Frontend/backend boundary
Treat frontend input as untrusted:
- validate input before file/network/system operations,
- avoid exposing raw filesystem/system access directly through commands.

---

## Architecture rules (simple, clean, not overbuilt)

When implementing a feature, prefer this flow:

`command -> application service -> infrastructure/domain`

Avoid:
- deep layer stacks with pass-through code,
- excessive factories/builders/services with no real logic,
- “enterprise” patterns without current need.

A small amount of duplication is acceptable if it keeps the code simpler and clearer.

Refactor only when duplication becomes meaningful or behavior diverges.

---

## What to do before changing architecture

Before introducing a new abstraction, ask:
1. Is there a second real use case already?
2. Does this reduce complexity today?
3. Does this improve readability for a Rust developer?
4. Does this preserve top-down flow?

If the answer is mostly “no”, keep the simpler version.

---

## Output expectations for code changes

When you (the agent) finish a task:
1. Summarize what changed.
2. Mention where business logic lives now (module/file).
3. Mention any permissions/capabilities changes (if any).
4. List validation commands run and their status.

If something could not be validated locally, state that explicitly.

---

## Preferred change style

Make incremental, focused changes:
- do not rewrite unrelated files,
- do not rename modules without strong reason,
- do not move large parts of the project unless required.

Preserve existing patterns unless they are clearly harmful.

If existing code is messy, improve only the touched area enough to support the change cleanly.

---

## Optional nested AGENTS.md

If the project grows, add nested `AGENTS.md` files in subdirectories (for example `src-tauri/src/commands/AGENTS.md` or frontend folders) to provide more specific rules for those areas.

More specific `AGENTS.md` files should refine (not contradict) this root file.

---

## Quick review checklist

Before finalizing a patch, verify:

- [ ] Command layer is thin
- [ ] Business logic is not inside Tauri command functions
- [ ] No unnecessary abstractions were introduced
- [ ] Names are explicit and not abbreviated
- [ ] Error handling uses `Result` and `?`
- [ ] `cargo fmt` passed
- [ ] `cargo clippy` passed
- [ ] `cargo test` passed (or limitation explained)
- [ ] Tauri permissions/capabilities are minimal (if changed)

---

## Git collaboration safety (must follow)

- Never revert or overwrite changes you did not create for the current task.
- Broad staging commands are allowed (`git add .`, `git commit -a`) only after verifying they contain only task-related files.
- Always verify staged content with `git status` and `git diff --staged` before committing.
- If unrelated staged or unstaged changes are present, leave them untouched and continue with your scoped files.
- Never use destructive restore/reset commands (`git reset --hard`, `git checkout -- <file>`, `git restore <file>`) unless the user explicitly asks for them.
