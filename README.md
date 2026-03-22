# Slint Learning Playground

This repository is a runnable Slint learning workspace for experimenting with how Rust application code and `.slint` UI files fit together.
It is intentionally organized like a study reference instead of a minimal one-file demo, so you can grow it into multiple examples without rewriting the basics.

## Purpose

Use this repo as a playground for learning:

- how a Slint desktop app is bootstrapped from Rust,
- how `.slint` files are compiled through `build.rs`,
- how navigation and shared state can stay in plain Rust modules,
- how top-level windows are owned from Rust while pages stay declarative in Slint,
- and how page-oriented demos can scale over time.

## Project organization

The project starts with an explicit, educational structure:

- `Cargo.toml` configures a binary desktop app and enables Slint build integration.
- `build.rs` compiles `ui/app-window.slint` so exported Slint components become Rust types.
- `src/main.rs` is the intentionally tiny entry point.
- `src/app.rs` wires Rust state into the generated Slint component API.
- `src/navigation.rs` holds page identifiers and conversion logic that is ready for unit tests.
- `src/models.rs` holds shared application state and data-shaping helpers.
- `src/window_management.rs` isolates multi-window lifecycle rules from the UI runtime.
- `ui/app-window.slint` is the root shell window that hosts every demo page.
- `ui/pages/*.slint` contains page-sized components to keep the UI easy to explore.
- `ui/windows/*.slint` contains extra top-level windows used by the window-management lesson.

## How to run

1. Install Rust and a native desktop toolchain for your platform.
2. From the repository root, run:

   ```bash
   cargo run
   ```

3. The app opens a desktop window with focused pages for layouts, widgets, menus, cross-page state, and multi-window patterns.

## Guided tour

If you are exploring the project for the first time, this reading order highlights the most important teaching examples:

1. **`ui/app-window.slint`** — start with the shell layout, navigation buttons, and the callback/property surface that Rust talks to.
2. **`src/app.rs`** — trace how callbacks are registered, how `Rc<RefCell<_>>` enables shared callback ownership, and where Rust republishes state back into Slint.
3. **`src/navigation.rs`** — see the page registry and enum/index conversion that keeps navigation testable.
4. **`ui/pages/dashboard.slint`** + **`src/dashboard.rs`** — compare a declarative menu-like UI with Rust-side command dispatch and navigation decisions.
5. **`ui/pages/cross-page-data.slint`** + **`src/models.rs`** — follow one-way and two-way data flow between Slint pages and Rust-owned state.
6. **`ui/pages/window-management.slint`** + **`src/window_management.rs`** + **`ui/windows/*.slint`** — study child-window ownership, reuse, hide/show behavior, and callback wiring.
7. **`ui/pages/lists-and-models.slint`** and **`ui/pages/layouts.slint`** — use these as visual labs for repeated models, nested layouts, and spacing experiments.
8. **`docs/learning-notes.md`** — read the short guide that summarizes what each page is meant to teach and what to tweak next.

## Contributor checklist for future demo pages

When you add or significantly revise a demo page, keep the learning shell consistent:

- Add the page to the central navigation flow so learners can actually reach it.
- Describe the page in project docs so the teaching purpose is discoverable outside the UI.
- Include teaching comments or labels in the `.slint` page so readers understand why the example exists.
- Keep Rust-owned state, formatting helpers, validation, and routing logic in unit-testable Rust modules, and add tests for that logic.
- Make demo interactions visible and intentional: controls should clearly show what changes, and placeholder interactions should be labeled as such.

## Adding a new demo page checklist

Use this sequence for every new page so the repo stays easy to browse and maintain:

1. Create a focused `ui/pages/<topic>.slint` file.
2. Register page metadata in `src/navigation.rs` so title, description, notes, and category stay centralized.
3. Connect the page to shell navigation in `ui/app-window.slint`.
4. Add sample controls, mock data, or explanatory content that make the interaction worth opening.
5. Add or update tests for Rust-side helpers, page registry metadata, and any navigation/discovery behavior touched by the change.

### Naming guidance

- Name `.slint` page files after the concept they teach, using lowercase kebab-case such as `cross-page-data.slint` or `performance-and-best-practices.slint`.
- Keep matching Rust modules concise and topic-based, using snake_case like `navigation.rs`, `dashboard.rs`, or `window_management.rs`.
- Prefer one teaching concept per page/module pair so filenames remain scannable in directory listings.
- Avoid vague names like `page2`, `misc`, or `helpers`; favor names that tell a new contributor what they can learn there.

### What belongs in Rust instead of UI markup

Keep concerns in Rust when they benefit from tests, reuse, or stronger guarantees. That includes:

- page registries, navigation metadata, and page-discovery rules,
- validation, clamping, parsing, and formatting helpers,
- shared state that spans multiple pages,
- command routing or action identifiers that should stay typed,
- and any transformation that would be difficult to verify if it lived only inside `.slint` bindings.

Keep the `.slint` markup focused on layout, styling, local presentation state, and clearly labeled demonstrations of bindings/callbacks.

## Cross-page shared state lesson

The `Cross-page Data` page is a dedicated teaching example for shared Rust-side state.

- **Where state lives:** `src/models.rs` owns the canonical `SharedLearningState`, including the shared note text and progress value.
- **How Slint updates Rust:** `ui/pages/cross-page-data.slint` emits callbacks when the learner applies a note or adjusts progress, and `src/app.rs` handles those callbacks.
- **How Rust pushes changes back:** `src/app.rs` recomputes a small cross-page view model and republishes it into `AppWindow` properties that both `ui/pages/cross-page-data.slint` and `ui/pages/dashboard.slint` read.
- **Direct binding vs callbacks:** keep direct bindings for values that stay local to one component; use callbacks when Rust must validate, clamp, reset, or share a value across pages.
- **Rust ownership caveat:** the app uses `Rc<RefCell<SharedLearningState>>` so multiple Slint callbacks can mutate one shared state object without fighting ownership or lifetime rules.

This gives you one **one-way** example (editing a note on the source page and only displaying it on the dashboard) and one **two-way** example (both pages can adjust the same shared progress value through Rust-owned state).

## Included test seams

The project includes unit-test-friendly seams for:

- default page selection at startup,
- enum/index route conversion,
- dashboard command dispatch,
- shared-state validation and clamping,
- and child-window lifecycle state transitions.

Run the test suite with:

```bash
cargo test
```
