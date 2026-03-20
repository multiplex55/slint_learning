# Learning Notes

This short guide summarizes what each major demo teaches and suggests a few safe experiments.

## Root shell

- **`ui/app-window.slint`**: demonstrates a two-column application shell with a persistent navigation panel and a content area that swaps between pages.
- **Watch for**: callback declarations such as `request-page`, `dashboard-action`, and the window-management callbacks. Those are the main Slint-to-Rust hooks.
- **Try changing this**: adjust the navigation card sizes or root window dimensions and observe how much of the shell responds automatically because of layout containers rather than manual coordinates.

## Dashboard and command routing

- **`ui/pages/dashboard.slint`**: demonstrates a custom menu-bar teaching example using booleans plus conditional popup panels.
- **`src/dashboard.rs`**: shows how string action identifiers from the UI become typed Rust commands.
- **Watch for**: nested submenu state, callback wiring through `trigger-action`, and the shared progress controls that prove the dashboard can also participate in cross-page updates.
- **Try changing this**: add a new action id in both files and confirm that the Rust test suite catches inconsistent metadata or missing dispatch logic.

## Layout patterns

- **`ui/pages/layouts.slint`**: demonstrates vertical stacks, horizontal rows, grid-like nested rows, and nested alignment containers.
- **Watch for**: how spacing/padding explain structure more clearly than decorative styling.
- **Try changing this**: increase spacing values or card widths and notice how layout containers redistribute space without extra Rust changes.

## Interactive widgets

- **`ui/pages/buttons-and-inputs.slint`**: demonstrates standard widgets such as buttons, toggles, line edits, combo boxes, sliders, and spin boxes.
- **Watch for**: which widgets only hold local state versus which ones would need a callback once Rust must validate or persist the value.
- **Try changing this**: bind the slider fill example to the slider value if you want to turn the static teaching example into a live binding demo.

## Repeated content and models

- **`ui/pages/lists-and-models.slint`**: demonstrates repeated cards rendered from a list literal inside a `for` loop.
- **Watch for**: the repeated-model shape (`for lesson[index] in [...]`) and how the index can drive labels or alternating styles.
- **Try changing this**: add or remove lessons and see that the list expands without changing the surrounding layout code.

## Styling and themes

- **`ui/pages/styling-and-themes.slint`**: demonstrates cards that differ mainly by palette, border, and typography decisions rather than layout structure.
- **Watch for**: how the `theme-name` property supplied by Rust is displayed inside a purely visual page.
- **Try changing this**: swap colors while leaving layout constants alone to isolate style changes from structural changes.

## Cross-page shared state

- **`ui/pages/cross-page-data.slint`**: demonstrates note editing, progress adjustment, and explicit callbacks that push changes into Rust.
- **`src/models.rs`**: demonstrates the canonical state plus a view-model method used to republish display-friendly text.
- **Watch for**: one-way note propagation, two-way progress updates, and input validation/clamping in Rust.
- **Try changing this**: modify the default note text or progress value in Rust and verify both the dashboard and this page reflect the new defaults.

## Window management

- **`ui/pages/window-management.slint`** + **`ui/windows/*.slint`**: demonstrate parent/child window coordination, reusable modeless windows, hidden windows, and newly spawned windows.
- **`src/window_management.rs`**: demonstrates the state machine behind those behaviors.
- **Watch for**: how child windows send callbacks back to Rust and how Rust retains handles so the windows stay alive.
- **Try changing this**: alter the payload helper text for one `WindowKind` and see how the corresponding child window receives the new explanation.

## Smaller reference pages

- **`ui/pages/welcome-page.slint`**, **`ui/pages/shared-state-page.slint`**, and **`ui/pages/generated-code-page.slint`** are intentionally compact components.
- **Use them for**: introducing property input, generated code from `build.rs`, and simple Rust-to-Slint string/integer handoff without the larger shell context.
