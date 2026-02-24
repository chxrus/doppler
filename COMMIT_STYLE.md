# Commit Style Guide

This repository uses **Google-style commit messages**.

In addition, you must **commit only your own changes**: do not include unrelated edits, drive-by refactors, mass formatting, or accidental generated files.

---

## Golden rule: commit only your changes

Before every commit:

1) Check the working tree:
- `git status`

2) Inspect what you are about to commit (staged diff):
- `git diff --staged`

3) If something unrelated is staged, unstage it:
- `git restore --staged <path>` (or `git reset <path>`)

4) If a file was modified accidentally by you, discard it:
- `git restore <path>`

5) If unrelated files are changed by other people/processes (parallel work), do not discard them:
- do not run `git restore <path>` on those files
- keep them untouched and simply exclude them from staging/commit

### What counts as “not your changes”
Do not include these unless your task explicitly requires them:
- reformatting unrelated files
- renaming/moving files without a reason
- dependency bumps (Cargo/npm) not needed for the change
- lockfile changes (`Cargo.lock`, `pnpm-lock.yaml`, `package-lock.json`, `yarn.lock`) unless required
- generated outputs (build artifacts, dist, coverage, logs)

If your task legitimately requires lockfile updates, mention it in the commit body.

---

## Commit message format (Google style)

Use:

```

<Subject line>

<Body>
```

Subject line is required. Body is required for non-trivial changes.

---

## Subject line rules

1. Imperative mood (write as a command)

Good:

* Add settings persistence
* Fix crash on window close
* Refactor Tauri command boundary
* Improve error mapping

Bad:

* Added settings persistence
* Fixes crash
* Settings persistence
* Updating code

2. Keep it short

* Target: up to 72 characters.

3. Capitalize the first letter.

4. No trailing period.

---

## Body rules

The body explains:

* what changed (high-level)
* why it changed (intent)
* important side effects
* architectural/security implications (especially for Tauri)

Do not just restate the diff.

### When body is required

Body is required if the commit:

* changes architecture or module boundaries
* modifies Tauri permissions/capabilities/plugins
* introduces or removes an abstraction
* changes shared state handling
* affects performance or security
* changes IPC (frontend invoke <-> Rust command boundary)
* is non-obvious in any way

---

## Body formatting

* Use short paragraphs.
* Prefer complete sentences.
* Mention any required follow-ups (migrations, config changes).

Example:

```
Add settings persistence

Persist settings via filesystem store.
Move logic out of Tauri commands into application services.

This keeps command layer thin and improves testability.
```

---

## Optional prefixes

Prefixes are optional, but allowed:

* Add
* Fix
* Remove
* Refactor
* Improve
* Rename
* Update
* Revert

Do not use Conventional Commits syntax (`feat:`, `fix:`) unless the repository already uses it everywhere.

---

## Tauri-specific rule

If the commit affects:

* filesystem/network/system access
* plugins
* permissions/capabilities

Then the body must explicitly mention:

* what capability/permission changed
* why it is needed
* scope/minimality (least privilege)

Example:

```
Add filesystem capability for settings

Introduce minimal capability for settings file access.
Scope it to the main window only (least privilege).
```

---

## Frontend TypeScript note (if relevant)

If the commit changes:

* nullish handling strategy
* tauri invoke boundary wrappers
* route `load` / actions architecture
* shared stores

Mention it in the body.

---

## Commit granularity

Commits must be:

* focused
* atomic
* logically grouped

Avoid:

* mixing multiple unrelated changes
* “format everything” commits
* huge commits with vague messages like “update code”

---

## Checklist before committing

* [ ] `git status` looks clean (only intended files changed)
* [ ] `git diff --staged` contains only intended changes
* [ ] No accidental lockfile / generated file changes
* [ ] Subject is imperative, capitalized, <= 72 chars, no period
* [ ] Body included when needed and explains “why”
