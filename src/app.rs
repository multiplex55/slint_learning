//! Application wiring lives here so `main.rs` stays tiny and the view-model style
//! glue can evolve without hiding navigation or shared-state concepts from learners.

use std::cell::RefCell;
use std::rc::Rc;

use crate::models::SharedLearningState;
use crate::navigation::LearningPage;
use crate::AppWindow;
use slint::ComponentHandle;

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
    app_window.set_page_title(current_page.title().into());
    app_window.set_shared_state_summary(state.page_summary(current_page).into());
    app_window.set_completed_topic_count(state.completed_topics.len() as i32);
    app_window.set_welcome_message(
        format!(
            "Explore each page to compare Rust-side state with Slint-side presentation, {}.",
            state.learner_name
        )
        .into(),
    );
    app_window.set_generated_code_summary(
        state
            .page_summary(LearningPage::GeneratedCodePatterns)
            .into(),
    );
}
