# Adding a New Teaching Page

This project keeps page registration in Rust and page layout in Slint so both sides stay easy to understand.

## Maintenance checklist

Before you consider a new page done, verify that it:

- appears in navigation,
- is described in repo docs,
- includes teaching comments or explanatory labels,
- keeps testable state/helpers in Rust with unit tests,
- and exposes interactions that are visible and intentional.

## 1. Create the Slint page component

Create a new file in `ui/pages/`, for example `ui/pages/animation-basics.slint`.

- Use lowercase kebab-case names that describe the teaching topic.
- Keep the page instructional and label each section clearly.
- Prefer a self-contained component that exposes only the inputs it needs.
- Reuse cards, spacing, and explanatory labels so the shell feels consistent.

## 2. Register the page in Rust

Edit `src/navigation.rs`.

- Add a new `PageId` variant.
- Insert that variant into `PageId::ALL` in the position you want it to appear.
- Add the index mapping in `as_index()` and `from_index()`.
- Add metadata in `PageId::meta()`.
- Confirm the central registry continues to match the enum ordering and required fields.

The unit tests in `src/navigation.rs` will fail if you forget to register the page, leave metadata empty, or break registry consistency.

## 3. Host the page inside the shell

Edit `ui/app-window.slint`.

- Import the new page file.
- Connect the page to the shell navigation.
- Add the conditional page instance in the central content area.

## 4. Add sample controls or data

Make the page worth opening on its own:

- include small controls, mock data, or visual state changes,
- make placeholder interactions obvious,
- and add teaching comments/labels that explain what to observe.

## 5. Keep testable logic in Rust

If the page needs shared context from Rust, add a property on `AppWindow` and populate it from `src/app.rs`.

Prefer Rust modules for concerns that should be testable or reusable, including:

- navigation and registry metadata,
- validation, clamping, parsing, or formatting helpers,
- shared state used by multiple pages,
- and command routing or action translation.

That pattern keeps the page demo-focused while making state ownership explicit.

## 6. Add or update tests

At minimum, update tests for any Rust-side helpers plus registry/navigation behavior touched by the new page.

- If you add metadata fields, extend the registry validation tests.
- If you change navigation/page discovery, update the consistency checks so the central registry remains trustworthy.
