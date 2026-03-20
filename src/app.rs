//! Application wiring keeps the shell focused on the split between testable Rust state
//! and declarative Slint page composition.

use std::cell::RefCell;
use std::rc::Rc;

use slint::ComponentHandle;

use crate::models::SharedLearningState;
use crate::navigation::{page_category, page_description, page_title, PageId};
use crate::AppWindow;

pub fn run_desktop_learning_app() -> Result<(), slint::PlatformError> {
    let app_window = AppWindow::new()?;
    let shared_state = Rc::new(RefCell::new(SharedLearningState::default()));

    apply_state_to_ui(&app_window, &shared_state.borrow());

    let weak_window = app_window.as_weak();
    let state_for_callback = Rc::clone(&shared_state);

    app_window.on_request_page(move |page_index| {
        if let Some(window) = weak_window.upgrade() {
            let mut state = state_for_callback.borrow_mut();
            state.navigation.select_page_by_index(page_index);
            apply_state_to_ui(&window, &state);
        }
    });

    app_window.run()
}

fn apply_state_to_ui(app_window: &AppWindow, state: &SharedLearningState) {
    let current_page = state.navigation.current_page;

    app_window.set_current_page(current_page.as_index());
    app_window.set_page_title(page_title(current_page).into());
    app_window.set_page_description(page_description(current_page).into());
    app_window.set_page_category(page_category(current_page).into());
    app_window.set_page_notes(state.page_focus_prompt(current_page).into());
    app_window.set_shared_status(state.shared_status().into());
    app_window.set_learner_name(state.learner_name.clone().into());
    app_window.set_cohort_name(state.cohort_name.clone().into());
    app_window.set_theme_name(state.current_theme.clone().into());
    app_window.set_total_pages(PageId::ALL.len() as i32);
}
