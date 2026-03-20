# Adding a New Teaching Page

This project keeps page registration in Rust and page layout in Slint so both sides stay easy to understand.

## 1. Register the page in Rust

Edit `src/navigation.rs`.

- Add a new `PageId` variant.
- Insert that variant into `PageId::ALL` in the position you want it to appear.
- Add the index mapping in `as_index()` and `from_index()`.
- Add metadata in `PageId::meta()`.
- Extend `PAGE_REGISTRY` if needed.

The unit tests in `src/navigation.rs` will fail if you forget to register the page or leave its metadata empty.

## 2. Add the Slint page component

Create a new file in `ui/pages/`, for example `ui/pages/animation-basics.slint`.

Use the existing pages as templates:

- Keep the page instructional and label each section clearly.
- Prefer a self-contained component that exposes only the inputs it needs.
- Reuse cards, spacing, and explanatory labels so the shell feels consistent.

## 3. Host the page inside the shell

Edit `ui/app-window.slint`.

- Import the new page file.
- Add a navigation button in the left sidebar.
- Add a conditional page instance in the central content area.

## 4. Optional Rust-owned text

If the new page needs shared context from Rust, add a property on `AppWindow` and populate it from `src/app.rs`.

That pattern keeps the page demo-focused while making state ownership explicit.
