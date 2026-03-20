//! Application wiring keeps the shell focused on the split between testable Rust state
//! and declarative Slint page composition.

use std::cell::RefCell;
use std::rc::Rc;

use slint::ComponentHandle;

use crate::dashboard::{dispatch_dashboard_action, DashboardCommand};
use crate::models::SharedLearningState;
use crate::navigation::{page_category, page_description, page_title, PageId};
use crate::window_management::{OpenResult, WindowKind};
use crate::{AboutWindow, AppWindow, DetailsWindow, InspectorWindow};

#[derive(Default)]
struct RuntimeWindows {
    // Ownership note: top-level windows must stay owned by Rust after `show()`.
    // If these handles were created inside a callback and then dropped, the
    // secondary windows would disappear immediately on most backends.
    details: Option<DetailsWindow>,
    inspector: Option<InspectorWindow>,
    about_windows: Vec<AboutWindow>,
}

pub fn run_desktop_learning_app() -> Result<(), slint::PlatformError> {
    let app_window = AppWindow::new()?;
    // Lifetime note: `Rc<RefCell<_>>` gives each callback shared ownership of the
    // controller state and child-window handles without requiring `'static` borrows
    // from the stack frame of `run_desktop_learning_app`.
    let shared_state = Rc::new(RefCell::new(SharedLearningState::default()));
    let runtime_windows = Rc::new(RefCell::new(RuntimeWindows::default()));

    apply_state_to_ui(&app_window, &shared_state.borrow());

    let weak_window = app_window.as_weak();
    let state_for_page_callback = Rc::clone(&shared_state);
    app_window.on_request_page(move |page_index| {
        if let Some(window) = weak_window.upgrade() {
            let mut state = state_for_page_callback.borrow_mut();
            state.navigation.select_page_by_index(page_index);
            state.last_dashboard_action = format!(
                "Navigation button opened the {} page.",
                page_title(state.navigation.current_page)
            );
            apply_state_to_ui(&window, &state);
        }
    });

    let weak_window = app_window.as_weak();
    let state_for_menu_callback = Rc::clone(&shared_state);
    app_window.on_dashboard_action(move |action_id| {
        if let Some(window) = weak_window.upgrade() {
            let mut state = state_for_menu_callback.borrow_mut();
            if let Some(dispatched) = dispatch_dashboard_action(action_id.as_str()) {
                state.last_dashboard_action = dispatched.status_message.to_string();
                if let DashboardCommand::Navigate(page) = dispatched.command {
                    state.navigation.current_page = page;
                }
                apply_state_to_ui(&window, &state);
            }
        }
    });

    let weak_window = app_window.as_weak();
    let state_for_note_callback = Rc::clone(&shared_state);
    app_window.on_update_shared_note(move |note_text| {
        if let Some(window) = weak_window.upgrade() {
            let mut state = state_for_note_callback.borrow_mut();
            state.update_shared_note(note_text.as_str());
            state.last_dashboard_action =
                "Cross-page editor updated the shared note text.".to_string();
            apply_state_to_ui(&window, &state);
        }
    });

    let weak_window = app_window.as_weak();
    let state_for_progress_callback = Rc::clone(&shared_state);
    app_window.on_adjust_shared_progress(move |delta| {
        if let Some(window) = weak_window.upgrade() {
            let mut state = state_for_progress_callback.borrow_mut();
            state.nudge_shared_progress(delta);
            state.last_dashboard_action = format!(
                "Shared progress changed to {}% and every page received the refreshed value.",
                state.shared_progress_value()
            );
            apply_state_to_ui(&window, &state);
        }
    });

    let weak_window = app_window.as_weak();
    let state_for_reset_callback = Rc::clone(&shared_state);
    app_window.on_reset_cross_page_demo(move || {
        if let Some(window) = weak_window.upgrade() {
            let mut state = state_for_reset_callback.borrow_mut();
            state.reset_cross_page_demo();
            state.last_dashboard_action =
                "Cross-page demo reset restored the shared note and progress defaults.".to_string();
            apply_state_to_ui(&window, &state);
        }
    });

    let weak_window = app_window.as_weak();
    let state_for_draft_callback = Rc::clone(&shared_state);
    app_window.on_update_window_demo_text(move |draft_text| {
        if let Some(window) = weak_window.upgrade() {
            let mut state = state_for_draft_callback.borrow_mut();
            state.window_demo.update_draft_text(draft_text.as_str());
            state.last_dashboard_action =
                "Window management page updated the text prepared for child windows.".to_string();
            apply_state_to_ui(&window, &state);
        }
    });

    let weak_window = app_window.as_weak();
    let state_for_details = Rc::clone(&shared_state);
    let runtime_for_details = Rc::clone(&runtime_windows);
    app_window.on_open_details_window(move || {
        if let Some(window) = weak_window.upgrade() {
            let result = {
                let mut state = state_for_details.borrow_mut();
                let result = state.window_demo.open_details_window();
                state.last_dashboard_action = match result {
                    OpenResult::Created => "Opened the reusable Details window.".to_string(),
                    OpenResult::Reused => {
                        "Reused the existing Details window instance.".to_string()
                    }
                };
                result
            };
            if let Err(error) =
                show_details_window(&window, &state_for_details, &runtime_for_details)
            {
                let mut state = state_for_details.borrow_mut();
                state.last_dashboard_action = format!("Failed to show the Details window: {error}");
            }
            apply_state_to_ui(&window, &state_for_details.borrow());
            let _ = result;
        }
    });

    let weak_window = app_window.as_weak();
    let state_for_close_details = Rc::clone(&shared_state);
    let runtime_for_close_details = Rc::clone(&runtime_windows);
    app_window.on_close_details_window(move || {
        if let Some(window) = weak_window.upgrade() {
            close_details_window(&state_for_close_details, &runtime_for_close_details);
            state_for_close_details.borrow_mut().last_dashboard_action =
                "Closed the Details window from the main page.".to_string();
            apply_state_to_ui(&window, &state_for_close_details.borrow());
        }
    });

    let weak_window = app_window.as_weak();
    let state_for_open_inspector = Rc::clone(&shared_state);
    let runtime_for_open_inspector = Rc::clone(&runtime_windows);
    app_window.on_open_inspector_window(move || {
        if let Some(window) = weak_window.upgrade() {
            {
                let mut state = state_for_open_inspector.borrow_mut();
                state.window_demo.open_inspector_window();
                state.last_dashboard_action =
                    "Opened or revealed the reusable Inspector window.".to_string();
            }
            if let Err(error) = show_inspector_window(
                &window,
                &state_for_open_inspector,
                &runtime_for_open_inspector,
            ) {
                state_for_open_inspector.borrow_mut().last_dashboard_action =
                    format!("Failed to show the Inspector window: {error}");
            }
            apply_state_to_ui(&window, &state_for_open_inspector.borrow());
        }
    });

    let weak_window = app_window.as_weak();
    let state_for_hide_inspector = Rc::clone(&shared_state);
    let runtime_for_hide_inspector = Rc::clone(&runtime_windows);
    app_window.on_hide_inspector_window(move || {
        if let Some(window) = weak_window.upgrade() {
            hide_inspector_window(&state_for_hide_inspector, &runtime_for_hide_inspector);
            state_for_hide_inspector.borrow_mut().last_dashboard_action =
                "Inspector window was hidden while Rust retained its handle.".to_string();
            apply_state_to_ui(&window, &state_for_hide_inspector.borrow());
        }
    });

    let weak_window = app_window.as_weak();
    let state_for_spawn_about = Rc::clone(&shared_state);
    let runtime_for_spawn_about = Rc::clone(&runtime_windows);
    app_window.on_spawn_about_window(move || {
        if let Some(window) = weak_window.upgrade() {
            {
                let mut state = state_for_spawn_about.borrow_mut();
                state.window_demo.spawn_about_window();
                state.last_dashboard_action = "Spawned another About window instance.".to_string();
            }
            if let Err(error) =
                show_about_window(&window, &state_for_spawn_about, &runtime_for_spawn_about)
            {
                state_for_spawn_about.borrow_mut().last_dashboard_action =
                    format!("Failed to show an About window: {error}");
            }
            apply_state_to_ui(&window, &state_for_spawn_about.borrow());
        }
    });

    let weak_window = app_window.as_weak();
    let state_for_close_all = Rc::clone(&shared_state);
    let runtime_for_close_all = Rc::clone(&runtime_windows);
    app_window.on_close_all_demo_windows(move || {
        if let Some(window) = weak_window.upgrade() {
            close_all_child_windows(&state_for_close_all, &runtime_for_close_all);
            state_for_close_all.borrow_mut().last_dashboard_action =
                "Closed every tracked child window from the main page.".to_string();
            apply_state_to_ui(&window, &state_for_close_all.borrow());
        }
    });

    app_window.run()
}

fn show_details_window(
    app_window: &AppWindow,
    shared_state: &Rc<RefCell<SharedLearningState>>,
    runtime_windows: &Rc<RefCell<RuntimeWindows>>,
) -> Result<(), slint::PlatformError> {
    let payload = shared_state
        .borrow()
        .window_demo
        .prepare_payload(WindowKind::Details);
    ensure_details_window(app_window, shared_state, runtime_windows)?;

    if let Some(details) = runtime_windows.borrow().details.as_ref() {
        details.set_selected_text(payload.selected_text.into());
        details.set_helper_text(payload.helper_text.into());
        details.show()?;
    }

    Ok(())
}

fn ensure_details_window(
    app_window: &AppWindow,
    shared_state: &Rc<RefCell<SharedLearningState>>,
    runtime_windows: &Rc<RefCell<RuntimeWindows>>,
) -> Result<(), slint::PlatformError> {
    if runtime_windows.borrow().details.is_some() {
        return Ok(());
    }

    let details = DetailsWindow::new()?;
    let weak_main = app_window.as_weak();
    let state_for_ack = Rc::clone(shared_state);
    details.on_acknowledge_selection(move |message| {
        if let Some(main) = weak_main.upgrade() {
            let mut state = state_for_ack.borrow_mut();
            state
                .window_demo
                .record_child_feedback(WindowKind::Details, message.as_str());
            state.last_dashboard_action =
                "Details window emitted a callback back to the parent shell.".to_string();
            apply_state_to_ui(&main, &state);
        }
    });

    let weak_main = app_window.as_weak();
    let state_for_close = Rc::clone(shared_state);
    let runtime_for_close = Rc::clone(runtime_windows);
    details.on_request_close(move || {
        close_details_window(&state_for_close, &runtime_for_close);
        if let Some(main) = weak_main.upgrade() {
            state_for_close.borrow_mut().last_dashboard_action =
                "Details window requested to close itself.".to_string();
            apply_state_to_ui(&main, &state_for_close.borrow());
        }
    });

    runtime_windows.borrow_mut().details = Some(details);
    Ok(())
}

fn close_details_window(
    shared_state: &Rc<RefCell<SharedLearningState>>,
    runtime_windows: &Rc<RefCell<RuntimeWindows>>,
) {
    shared_state.borrow_mut().window_demo.close_details_window();
    if let Some(details) = runtime_windows.borrow().details.as_ref() {
        details.hide().ok();
    }
}

fn show_inspector_window(
    app_window: &AppWindow,
    shared_state: &Rc<RefCell<SharedLearningState>>,
    runtime_windows: &Rc<RefCell<RuntimeWindows>>,
) -> Result<(), slint::PlatformError> {
    let payload = shared_state
        .borrow()
        .window_demo
        .prepare_payload(WindowKind::Inspector);
    ensure_inspector_window(app_window, shared_state, runtime_windows)?;

    if let Some(inspector) = runtime_windows.borrow().inspector.as_ref() {
        inspector.set_selected_text(payload.selected_text.into());
        inspector.set_helper_text(payload.helper_text.into());
        inspector.show()?;
    }

    Ok(())
}

fn ensure_inspector_window(
    app_window: &AppWindow,
    shared_state: &Rc<RefCell<SharedLearningState>>,
    runtime_windows: &Rc<RefCell<RuntimeWindows>>,
) -> Result<(), slint::PlatformError> {
    if runtime_windows.borrow().inspector.is_some() {
        return Ok(());
    }

    let inspector = InspectorWindow::new()?;
    let weak_main = app_window.as_weak();
    let state_for_ping = Rc::clone(shared_state);
    inspector.on_ping_parent(move |message| {
        if let Some(main) = weak_main.upgrade() {
            let mut state = state_for_ping.borrow_mut();
            state
                .window_demo
                .record_child_feedback(WindowKind::Inspector, message.as_str());
            state.last_dashboard_action =
                "Inspector window emitted a callback back to the parent shell.".to_string();
            apply_state_to_ui(&main, &state);
        }
    });

    let weak_main = app_window.as_weak();
    let state_for_hide = Rc::clone(shared_state);
    let runtime_for_hide = Rc::clone(runtime_windows);
    inspector.on_request_hide(move || {
        hide_inspector_window(&state_for_hide, &runtime_for_hide);
        if let Some(main) = weak_main.upgrade() {
            state_for_hide.borrow_mut().last_dashboard_action =
                "Inspector window requested to hide itself.".to_string();
            apply_state_to_ui(&main, &state_for_hide.borrow());
        }
    });

    runtime_windows.borrow_mut().inspector = Some(inspector);
    Ok(())
}

fn hide_inspector_window(
    shared_state: &Rc<RefCell<SharedLearningState>>,
    runtime_windows: &Rc<RefCell<RuntimeWindows>>,
) {
    shared_state
        .borrow_mut()
        .window_demo
        .hide_inspector_window();
    if let Some(inspector) = runtime_windows.borrow().inspector.as_ref() {
        inspector.hide().ok();
    }
}

fn show_about_window(
    app_window: &AppWindow,
    shared_state: &Rc<RefCell<SharedLearningState>>,
    runtime_windows: &Rc<RefCell<RuntimeWindows>>,
) -> Result<(), slint::PlatformError> {
    let payload = shared_state
        .borrow()
        .window_demo
        .prepare_payload(WindowKind::About);
    let about = AboutWindow::new()?;
    about.set_selected_text(payload.selected_text.into());
    about.set_helper_text(payload.helper_text.into());

    let weak_about = about.as_weak();
    let weak_main = app_window.as_weak();
    let state_for_close = Rc::clone(shared_state);
    about.on_request_close(move || {
        if let Some(window) = weak_about.upgrade() {
            window.hide().ok();
        }
        state_for_close
            .borrow_mut()
            .window_demo
            .close_one_about_window();
        if let Some(main) = weak_main.upgrade() {
            state_for_close.borrow_mut().last_dashboard_action =
                "An About window closed itself.".to_string();
            apply_state_to_ui(&main, &state_for_close.borrow());
        }
    });

    about.show()?;
    runtime_windows.borrow_mut().about_windows.push(about);
    Ok(())
}

fn close_all_child_windows(
    shared_state: &Rc<RefCell<SharedLearningState>>,
    runtime_windows: &Rc<RefCell<RuntimeWindows>>,
) {
    shared_state.borrow_mut().window_demo.close_all_windows();
    let mut windows = runtime_windows.borrow_mut();
    if let Some(details) = windows.details.as_ref() {
        details.hide().ok();
    }
    if let Some(inspector) = windows.inspector.as_ref() {
        inspector.hide().ok();
    }
    for about in &windows.about_windows {
        about.hide().ok();
    }
    windows.about_windows.clear();
}

fn apply_state_to_ui(app_window: &AppWindow, state: &SharedLearningState) {
    let current_page = state.navigation.current_page;
    let cross_page = state.cross_page_view_model();
    let window_demo = state.window_management_view_model();

    app_window.set_current_page(current_page.as_index());
    app_window.set_page_title(page_title(current_page).into());
    app_window.set_page_description(page_description(current_page).into());
    app_window.set_page_category(page_category(current_page).into());
    app_window.set_page_notes(state.page_focus_prompt(current_page).into());
    app_window.set_shared_status(state.shared_status().into());
    app_window.set_dashboard_status_summary(state.dashboard_status_summary().into());
    app_window.set_learner_name(state.learner_name.clone().into());
    app_window.set_cohort_name(state.cohort_name.clone().into());
    app_window.set_theme_name(state.current_theme.clone().into());
    app_window.set_total_pages(PageId::ALL.len() as i32);
    app_window.set_shared_note_text(cross_page.note_text.into());
    app_window.set_shared_note_status(cross_page.note_status.into());
    app_window.set_shared_progress_value(cross_page.progress_value);
    app_window.set_shared_progress_label(cross_page.progress_label.into());
    app_window.set_dashboard_preview_text(cross_page.dashboard_preview.into());
    app_window.set_editor_hint(cross_page.editor_hint.into());
    app_window.set_sync_notes(cross_page.synchronization_notes.into());
    app_window.set_window_demo_text(window_demo.draft_text.into());
    app_window.set_window_demo_details_status(window_demo.details_status.into());
    app_window.set_window_demo_inspector_status(window_demo.inspector_status.into());
    app_window.set_window_demo_about_status(window_demo.about_status.into());
    app_window.set_window_demo_child_feedback(window_demo.child_feedback.into());
    app_window.set_window_demo_lifecycle_status(window_demo.lifecycle_status.into());
    app_window.set_window_demo_instructional_text(window_demo.instructional_text.into());
    app_window.set_window_demo_prepared_preview(window_demo.prepared_preview.into());
}
