//! Entry point for the Slint learning workspace.
//! The shell intentionally keeps navigation logic in Rust and page composition in `.slint` files.

mod app;
mod dashboard;
mod models;
mod navigation;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    app::run_desktop_learning_app()
}
