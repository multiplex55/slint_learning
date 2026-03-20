//! Entry point for the Slint learning workspace.
//! Module responsibility: keep startup tiny so learners can immediately spot where the
//! Rust/Slint boundary begins.
//! UI connection: this file pulls in the Slint-generated component types with
//! `slint::include_modules!()` and hands control to `app::run_desktop_learning_app()`.
//! Study here: how a Rust binary stays minimal while feature-specific behavior lives in
//! smaller modules that are easier to test and explain.

mod app;
mod dashboard;
mod models;
mod navigation;
mod window_management;

// Take note: `include_modules!()` exposes the components exported from `ui/app-window.slint`
// as Rust types such as `AppWindow`, `DetailsWindow`, and the other top-level windows.
slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    app::run_desktop_learning_app()
}
