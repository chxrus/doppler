# AGENTS.md

## Purpose

This directory contains the **SvelteKit + TypeScript** frontend for a Tauri desktop app.

Goals for any frontend change:
1. Keep UI code simple and readable.
2. Preserve SvelteKit conventions (routing, load, actions, endpoints).
3. Keep TypeScript strict and explicit.
4. Keep Tauri integration isolated behind a small frontend API layer.

This file contains frontend-specific rules for AI coding agents.

---

## Working style for this frontend

The project prefers:
- top-down readability,
- explicit names,
- small focused functions/components,
- minimal accidental complexity,
- no unnecessary abstractions.

Use idiomatic SvelteKit patterns instead of bringing OOP-heavy frontend architecture.

Do not over-engineer stores, service layers, or component hierarchies.

---

## Setup and validation commands

Run the relevant commands before finishing a frontend task.

Use the package manager used in this repo (`npm`). Check `package.json` scripts first.

Typical commands (adjust to repo scripts):
- Install deps: `npm install`
- Dev server: `npm run dev`
- Type check: `npm run check` or `npm run svelte-check`
- Lint: `npm run lint`
- Tests: `npm test`

If a script name differs in this repo, use the repo-defined script.

Do not skip type checking for TypeScript/Svelte changes.

---

## SvelteKit conventions (must follow)

SvelteKit has strong file-based conventions. Keep them.

### 1) Routes and special files
Use SvelteKit route files correctly:
- `+page.svelte` — page UI
- `+page.ts` / `+page.server.ts` — page `load`
- `+layout.svelte` / `+layout.ts` / `+layout.server.ts` — layout and layout load
- `+server.ts` — endpoint handlers (GET/POST/etc.)

Follow existing route structure instead of inventing custom routing layers. SvelteKit documents this project/file structure explicitly. :contentReference[oaicite:1]{index=1}

### 2) Data loading
Use `load` for page/layout data fetching and shaping page data.

Rules:
- Keep `load` focused on loading and mapping data for the route.
- Do not place complex business logic directly in `load`; call a helper/service if needed.
- Use generated route types (`./$types`) for `PageLoad`, `PageServerLoad`, etc.

SvelteKit’s `load` is the standard mechanism for route data. Typed route helpers are generated and should be used. :contentReference[oaicite:2]{index=2}

### 3) Form handling
For server-backed form submissions, prefer SvelteKit form actions (`+page.server.ts`) instead of ad-hoc client-side fetches, unless there is a good reason not to.

Rules:
- Keep actions thin and explicit.
- Validate input.
- Return structured errors/results for UI rendering.
- Do not hide server mutations inside random component code.

Form actions are a first-class SvelteKit pattern and should be preferred for route-level forms. :contentReference[oaicite:3]{index=3}

### 4) Endpoints
Use `+server.ts` for HTTP-style handlers when needed (API-like routes, downloads, webhooks, etc.).

Keep endpoint handlers:
- small,
- typed,
- explicit about status and error handling.

---

## TypeScript rules (Svelte + TS)

TypeScript is required for this frontend.

### 1) Use TypeScript in Svelte files
Use `<script lang="ts">` in Svelte components.

Svelte supports TypeScript directly, and Svelte docs recommend TypeScript-enabled setup in SvelteKit projects. :contentReference[oaicite:4]{index=4}

### 2) Prefer generated and explicit types
- Prefer SvelteKit-generated route types from `./$types`.
- Type component props, events, and function parameters explicitly when inference is not obvious.
- Avoid `any`.

### 3) Keep types close to usage
- Small local types stay near the component/module.
- Shared types go to a dedicated `lib/types/` (or project equivalent).
- Do not create giant “global types” files too early.

### 4) Avoid type noise
Use enough typing to make behavior obvious, but do not create complex generic abstractions unless they solve a real problem now.

---

## Component design rules

## 1) Components should be focused
A component should have one UI responsibility.

If a `.svelte` file grows too much:
- extract presentational child components,
- move non-UI logic to a helper/module.

## 2) Keep pages thin
Route components (`+page.svelte`, `+layout.svelte`) should compose UI and bind data.
Do not pack them with business logic or low-level API details.

## 3) Naming
- Use full descriptive names.
- Do not abbreviate variable names.
- Avoid vague names like `data`, `item`, `handler`, `utils` unless context is obvious.

## 4) Imports
- Prefer alias imports via `$lib` for shared frontend modules.
- Do not use deep relative imports like `../../` or `../../../` when the same module can be imported from `$lib/...`.
- Keep relative imports only for very local neighbors in the same route/component folder when it improves readability.

## 4) Top-down readability
Inside components/modules:
1. imports
2. exported props / top-level state
3. derived values
4. event handlers / actions
5. helper functions (lower in file)
6. markup
7. styles (if local styles are used)

Make the “main path” easy to read first.

---

## State management rules

Use the simplest state mechanism that works.

Preferred order:
1. local component state
2. props + events
3. route `load` data
4. Svelte stores (only for truly shared state)

Do not introduce global stores for page-local state.

If using stores:
- keep them small and focused,
- avoid one giant app store,
- keep side effects out of store definitions when possible.

---

## Tauri integration from frontend (important)

This is a Tauri app. Frontend should not call Rust commands everywhere directly.

### Rule: isolate Tauri calls
Create/maintain a small frontend boundary (for example):
- `src/lib/tauri/commands.ts`
- or `src/lib/api/desktop.ts`

All `invoke(...)` calls should go there (or the existing equivalent).

Benefits:
- one place for command names,
- one place for request/response typing,
- easier refactoring and testing,
- cleaner Svelte components.

### Validate and map
At the Tauri boundary:
- keep command names centralized,
- type inputs/outputs,
- map backend errors into UI-friendly frontend errors.

Do not spread raw backend error strings across components.

---

## Error handling and UX

### 1) Handle expected failures explicitly
For async UI operations:
- show loading state,
- handle failure state,
- handle empty state where applicable.

### 2) Do not swallow errors
- No empty `catch`.
- Log in development when useful.
- Show user-safe messages in UI.

### 3) Keep error messages actionable
Prefer:
- “Could not save settings. Try again.”
over
- “Operation failed”

---

## Styling and UI consistency

Follow the existing UI approach in this repo (CSS modules, Tailwind, component library, etc.). Do not introduce a new styling system in a small feature task.

When changing UI:
- preserve spacing and naming conventions already used,
- avoid one-off styles if a reusable class/pattern exists,
- keep components visually consistent with nearby screens.

---

## Architecture rules (frontend)

Preferred frontend flow:

`route load / action -> page component -> child components`

or for Tauri commands:

`component event -> frontend tauri boundary -> Tauri command`

Avoid:
- hidden side effects in component initialization,
- direct backend calls scattered across components,
- generic “service managers” with no real benefit,
- over-abstracting simple form/page logic.

A little duplication is acceptable if it keeps code obvious.

---

## What not to do

- Do not bypass SvelteKit conventions for routing/data without a good reason.
- Do not introduce broad state management for local problems.
- Do not create abstraction layers “for future scalability”.
- Do not use abbreviated variable names.
- Do not leave `any` in finished code unless unavoidable and documented.

---

## Git collaboration safety (must follow)

- Never revert or overwrite changes you did not create for the current task.
- Broad staging commands are allowed (`git add .`, `git commit -a`) only after verifying they contain only task-related files.
- Always verify staged content with `git status` and `git diff --staged` before committing.
- If unrelated staged or unstaged changes are present, leave them untouched and continue with your scoped files.
- Never use destructive restore/reset commands (`git reset --hard`, `git checkout -- <file>`, `git restore <file>`) unless the user explicitly asks for them.
- Do not use deep relative imports for shared code; use `$lib/...`.
- Do not rewrite unrelated components in the same task.

---

## Output expectations for frontend changes

When finishing a task, include:
1. What changed (files/components/routes).
2. Where the main logic now lives.
3. Whether `load`, form actions, or `+server.ts` were added/changed.
4. Whether Tauri command bindings were added/changed.
5. Which checks were run (`svelte-check`, lint, tests) and result.

If checks could not be run, say so explicitly.

---

## TypeScript nullish and type narrowing rules

### 1) Nullish checks
Prefer nullish checks with `== null` when the intent is “null or undefined”.

Use:
- `value == null`  // matches null and undefined
- `value != null`  // excludes null and undefined

Do not write:
- `value === undefined`
- `value !== undefined`

Reason: in this project, nullish checks should be explicit and compact.

### 2) Avoid noisy ternary mapping chains
Do not create long chains of nested ternaries (`?:`) for value mapping.

Avoid:
- nested `condition ? a : condition2 ? b : c`
- large inline transformations in markup

Prefer:
- small named helper functions
- `if/else`
- lookup objects / maps (when appropriate)
- derived values computed above markup

### 3) Avoid excessive `typeof` checks
Do not spam `typeof` checks throughout the code.

Use `typeof` only when actually narrowing unknown input at boundaries (external data, Tauri responses, URL params, etc.).

Inside normal app code, prefer:
- typed function signatures
- explicit interfaces/types
- parsing/validation at the boundary once, not repeated checks everywhere

### 4) Boundary-first validation
Validate unknown data once (API/Tauri/JSON boundary), then pass typed objects through the app.

Do not repeat null/type checks in every component if the data was already validated.

---

## Quick review checklist

Before finalizing a patch, verify:

- [ ] SvelteKit route/file conventions are respected
- [ ] `load`/actions/endpoints are used appropriately
- [ ] Components are focused and readable
- [ ] Tauri calls are isolated behind a frontend boundary
- [ ] TypeScript types are explicit where needed (no stray `any`)
- [ ] Names are explicit and not abbreviated
- [ ] Type check passed
- [ ] Lint passed (if configured)
- [ ] Tests passed (if applicable)
