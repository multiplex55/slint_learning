//! Rust-side controller for the window-management lesson.
//! The controller is intentionally UI-toolkit agnostic so the state transitions
//! can be unit tested without launching real desktop windows.

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowKind {
    Details,
    Inspector,
    About,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WindowVisibility {
    Closed,
    Hidden,
    Visible,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum OpenResult {
    Created,
    Reused,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChildWindowPayload {
    pub title: String,
    pub selected_text: String,
    pub helper_text: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WindowManagementViewModel {
    pub draft_text: String,
    pub details_status: String,
    pub inspector_status: String,
    pub about_status: String,
    pub child_feedback: String,
    pub lifecycle_status: String,
    pub instructional_text: String,
    pub prepared_preview: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WindowManagementController {
    draft_text: String,
    details_visibility: WindowVisibility,
    inspector_visibility: WindowVisibility,
    about_open_count: usize,
    child_feedback: String,
    lifecycle_status: String,
}

impl Default for WindowManagementController {
    fn default() -> Self {
        Self {
            draft_text: Self::default_draft_text().to_string(),
            details_visibility: WindowVisibility::Closed,
            inspector_visibility: WindowVisibility::Closed,
            about_open_count: 0,
            child_feedback:
                "No child window has replied yet. Open one and click its callback button."
                    .to_string(),
            lifecycle_status:
                "All demo child windows start closed so the learner can observe each transition."
                    .to_string(),
        }
    }
}

impl WindowManagementController {
    pub const fn default_draft_text() -> &'static str {
        "Selected list item: Window management walkthrough"
    }

    pub fn update_draft_text<S: AsRef<str>>(&mut self, value: S) {
        let trimmed = value.as_ref().trim();
        self.draft_text = if trimmed.is_empty() {
            Self::default_draft_text().to_string()
        } else {
            trimmed.to_string()
        };
    }

    pub fn details_visibility(&self) -> WindowVisibility {
        self.details_visibility
    }

    pub fn inspector_visibility(&self) -> WindowVisibility {
        self.inspector_visibility
    }

    pub fn about_open_count(&self) -> usize {
        self.about_open_count
    }

    pub fn open_details_window(&mut self) -> OpenResult {
        let result = if self.details_visibility == WindowVisibility::Visible {
            OpenResult::Reused
        } else {
            OpenResult::Created
        };
        self.details_visibility = WindowVisibility::Visible;
        self.lifecycle_status = match result {
            OpenResult::Created => {
                "Details window is now visible. This demo reuses the same modeless instance on later clicks."
                    .to_string()
            }
            OpenResult::Reused => {
                "Details window was already open, so Rust reused the existing modeless instance."
                    .to_string()
            }
        };
        result
    }

    pub fn open_inspector_window(&mut self) -> OpenResult {
        let result = if self.inspector_visibility == WindowVisibility::Closed {
            OpenResult::Created
        } else {
            OpenResult::Reused
        };
        self.inspector_visibility = WindowVisibility::Visible;
        self.lifecycle_status = match result {
            OpenResult::Created => {
                "Inspector window opened. Later hide/show actions keep the same instance alive."
                    .to_string()
            }
            OpenResult::Reused => {
                "Inspector window became visible again without recreating the component."
                    .to_string()
            }
        };
        result
    }

    pub fn hide_inspector_window(&mut self) {
        self.inspector_visibility = if self.inspector_visibility == WindowVisibility::Closed {
            WindowVisibility::Closed
        } else {
            WindowVisibility::Hidden
        };
        self.lifecycle_status =
            "Inspector window is hidden, not destroyed, so its handle can be shown again."
                .to_string();
    }

    pub fn close_details_window(&mut self) {
        self.details_visibility = WindowVisibility::Closed;
        self.lifecycle_status = "Details window closed. The next open action recreates or re-shows the stored instance from Rust."
            .to_string();
    }

    pub fn spawn_about_window(&mut self) -> OpenResult {
        self.about_open_count += 1;
        self.lifecycle_status = format!(
            "Opened about window #{}, demonstrating the create-new-instance pattern for modeless utility windows.",
            self.about_open_count
        );
        OpenResult::Created
    }

    pub fn close_one_about_window(&mut self) {
        self.about_open_count = self.about_open_count.saturating_sub(1);
        self.lifecycle_status = format!(
            "An about window closed. {} about window(s) remain open.",
            self.about_open_count
        );
    }

    pub fn close_all_windows(&mut self) {
        self.details_visibility = WindowVisibility::Closed;
        self.inspector_visibility = WindowVisibility::Closed;
        self.about_open_count = 0;
        self.lifecycle_status =
            "Rust reset every tracked child window to the closed state.".to_string();
    }

    pub fn prepare_payload(&self, kind: WindowKind) -> ChildWindowPayload {
        let (title, helper_text) = match kind {
            WindowKind::Details => (
                "Details Window",
                "Modeless and reused: opening it again should focus/update the same instance.",
            ),
            WindowKind::Inspector => (
                "Inspector Window",
                "Modeless and hideable: the Rust controller keeps it alive while hidden.",
            ),
            WindowKind::About => (
                "About Window",
                "Modeless and recreated per action: each click can produce another window.",
            ),
        };

        ChildWindowPayload {
            title: title.to_string(),
            selected_text: self.draft_text.clone(),
            helper_text: helper_text.to_string(),
        }
    }

    pub fn record_child_feedback<S: AsRef<str>>(&mut self, kind: WindowKind, message: S) {
        self.child_feedback = format!(
            "{} replied: {}",
            match kind {
                WindowKind::Details => "Details window",
                WindowKind::Inspector => "Inspector window",
                WindowKind::About => "About window",
            },
            message.as_ref().trim()
        );
    }

    pub fn view_model(&self) -> WindowManagementViewModel {
        WindowManagementViewModel {
            draft_text: self.draft_text.clone(),
            details_status: match self.details_visibility {
                WindowVisibility::Visible => "Details window: visible (reused single instance).".to_string(),
                WindowVisibility::Hidden => "Details window: hidden.".to_string(),
                WindowVisibility::Closed => "Details window: closed.".to_string(),
            },
            inspector_status: match self.inspector_visibility {
                WindowVisibility::Visible => "Inspector window: visible (same hidden/revealed instance).".to_string(),
                WindowVisibility::Hidden => "Inspector window: hidden (handle retained in Rust).".to_string(),
                WindowVisibility::Closed => "Inspector window: closed.".to_string(),
            },
            about_status: format!(
                "About windows: {} open (new instance per click, if the backend supports multiple top-level windows).",
                self.about_open_count
            ),
            child_feedback: self.child_feedback.clone(),
            lifecycle_status: self.lifecycle_status.clone(),
            instructional_text: "Try this sequence: type new text, open Details, then click the child callback button. Next hide/show the Inspector and observe that its state is reused. Finally spawn multiple About windows to compare the create-new-instance approach."
                .to_string(),
            prepared_preview: format!(
                "The next child window will receive: '{}'",
                self.draft_text
            ),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{OpenResult, WindowKind, WindowManagementController, WindowVisibility};

    #[test]
    fn controller_tracks_window_open_state_across_multiple_child_types() {
        let mut controller = WindowManagementController::default();

        assert_eq!(controller.details_visibility(), WindowVisibility::Closed);
        assert_eq!(controller.inspector_visibility(), WindowVisibility::Closed);
        assert_eq!(controller.about_open_count(), 0);

        controller.open_details_window();
        controller.open_inspector_window();
        controller.spawn_about_window();

        assert_eq!(controller.details_visibility(), WindowVisibility::Visible);
        assert_eq!(controller.inspector_visibility(), WindowVisibility::Visible);
        assert_eq!(controller.about_open_count(), 1);
    }

    #[test]
    fn opening_same_demo_window_twice_reuses_existing_details_instance() {
        let mut controller = WindowManagementController::default();

        assert_eq!(controller.open_details_window(), OpenResult::Created);
        assert_eq!(controller.open_details_window(), OpenResult::Reused);
        assert_eq!(controller.details_visibility(), WindowVisibility::Visible);
    }

    #[test]
    fn child_payload_is_prepared_from_main_window_state_before_display() {
        let mut controller = WindowManagementController::default();
        controller.update_draft_text("Selected item: Modal vs modeless");

        let payload = controller.prepare_payload(WindowKind::Inspector);

        assert_eq!(payload.title, "Inspector Window");
        assert_eq!(payload.selected_text, "Selected item: Modal vs modeless");
        assert!(payload.helper_text.contains("hideable"));
    }

    #[test]
    fn closing_or_resetting_children_updates_parent_controller_state() {
        let mut controller = WindowManagementController::default();
        controller.open_details_window();
        controller.open_inspector_window();
        controller.spawn_about_window();
        controller.spawn_about_window();

        controller.hide_inspector_window();
        controller.close_details_window();
        controller.close_one_about_window();

        assert_eq!(controller.details_visibility(), WindowVisibility::Closed);
        assert_eq!(controller.inspector_visibility(), WindowVisibility::Hidden);
        assert_eq!(controller.about_open_count(), 1);

        controller.close_all_windows();

        assert_eq!(controller.details_visibility(), WindowVisibility::Closed);
        assert_eq!(controller.inspector_visibility(), WindowVisibility::Closed);
        assert_eq!(controller.about_open_count(), 0);
    }
}
