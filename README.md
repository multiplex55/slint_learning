# Slint Learning Playground

This repository is a runnable Slint learning workspace for experimenting with how Rust application code and `.slint` UI files fit together.
It is intentionally organized like a study reference instead of a minimal one-file demo, so you can grow it into multiple examples without rewriting the basics.

## Purpose

Use this repo as a playground for learning:

- how a Slint desktop app is bootstrapped from Rust,
- how `.slint` files are compiled through `build.rs`,
- how navigation and shared state can stay in plain Rust modules,
- and how page-oriented demos can scale over time.

## Project organization

The project starts with an explicit, educational structure:

- `Cargo.toml` configures a binary desktop app and enables Slint build integration.
- `build.rs` compiles `ui/app-window.slint` so exported Slint components become Rust types.
- `src/main.rs` is the small entry point.
- `src/app.rs` wires Rust state into the generated Slint component API.
- `src/navigation.rs` holds page identifiers and conversion logic that is ready for unit tests.
- `src/models.rs` holds shared application state and data-shaping helpers.
- `ui/app-window.slint` is the root window.
- `ui/pages/*.slint` contains page-sized components to keep the UI easy to explore.

## How to run

1. Install Rust and a native desktop toolchain for your platform.
2. From the repository root, run:

   ```bash
   cargo run
   ```

3. The app opens a small desktop window with starter pages for navigation, shared state, and generated-code integration.

## Where to look first

If you are learning Slint step by step, a good reading order is:

1. `ui/app-window.slint` to see the root window and its Rust-facing properties/callback.
2. `src/app.rs` to see how Rust updates the generated Slint API.
3. `src/navigation.rs` to understand page selection without UI details.
4. `src/models.rs` to see the initial shared state and simple transformation logic.
5. `build.rs` to understand how `.slint` files become available through `slint::include_modules!()`.

## Cross-page shared state lesson

The `Cross-page Data` page is now a dedicated teaching example for shared Rust-side state.

- **Where state lives:** `src/models.rs` owns the canonical `SharedLearningState`, including the shared note text and progress value.
- **How Slint updates Rust:** `ui/pages/cross-page-data.slint` emits callbacks when the learner applies a note or adjusts progress, and `src/app.rs` handles those callbacks.
- **How Rust pushes changes back:** `src/app.rs` recomputes a small cross-page view model and republishes it into `AppWindow` properties that both `ui/pages/cross-page-data.slint` and `ui/pages/dashboard.slint` read.
- **Direct binding vs callbacks:** keep direct bindings for values that stay local to one component; use callbacks when Rust must validate, clamp, reset, or share a value across pages.
- **Rust ownership caveat:** the app uses `Rc<RefCell<SharedLearningState>>` so multiple Slint callbacks can mutate one shared state object without fighting ownership or lifetime rules.

This gives you one **one-way** example (editing a note on the source page and only displaying it on the dashboard) and one **two-way** example (both pages can adjust the same shared progress value through Rust-owned state).

## Included test seams

The initial structure already includes unit-test-ready seams for:

- default page selection at startup,
- enum/index route conversion,
- and safe shared state initialization for later demos.

Run the test suite with:

```bash
cargo test
```
