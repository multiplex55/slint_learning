//! Entry point for the Slint learning workspace.
//! Start here after skimming `README.md`: it keeps the binary tiny and delegates
//! the interesting behavior into plain Rust modules that are easy to unit test.

mod app;
mod models;
mod navigation;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    app::run_desktop_learning_app()
}
