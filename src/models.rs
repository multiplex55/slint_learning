//! Shared learning data sits outside the `.slint` files so later lessons can connect
//! richer state without rewriting the teaching shell.
//!
//! This module now includes a dedicated cross-page example: Rust owns the shared
//! application state, Slint pages emit callbacks when the learner edits values,
//! and Rust pushes the canonical values back into UI properties for every page.
//! That pattern keeps borrowing, validation, and reset behavior in one place.

use crate::dashboard::default_recent_items;
use crate::navigation::{page_title, NavigationState, PageId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CrossPageViewModel {
    pub note_text: String,
    pub note_status: String,
    pub progress_value: i32,
    pub progress_label: String,
    pub dashboard_preview: String,
    pub editor_hint: String,
    pub synchronization_notes: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SharedLearningState {
    pub learner_name: String,
    pub cohort_name: String,
    pub current_theme: String,
    pub navigation: NavigationState,
    pub last_dashboard_action: String,
    shared_note_text: String,
    shared_progress_value: i32,
}

impl Default for SharedLearningState {
    fn default() -> Self {
        let recent_items = default_recent_items();
        Self {
            learner_name: "Curious Slint learner".to_string(),
            cohort_name: "Spring UI study group".to_string(),
            current_theme: "Studio Light".to_string(),
            navigation: NavigationState::default(),
            last_dashboard_action: format!(
                "Dashboard ready. Recent menu demos include {} and {}.",
                recent_items[0].label, recent_items[1].label
            ),
            shared_note_text: Self::default_note_text().to_string(),
            shared_progress_value: Self::default_progress_value(),
        }
    }
}

impl SharedLearningState {
    pub const fn default_note_text() -> &'static str {
        "Watch this note travel from the editor page into the dashboard preview."
    }

    pub const fn default_progress_value() -> i32 {
        35
    }

    pub const fn min_progress_value() -> i32 {
        0
    }

    pub const fn max_progress_value() -> i32 {
        100
    }

    pub fn shared_status(&self) -> String {
        format!(
            "{} is exploring the shell with the {} cohort using the {} theme.",
            self.learner_name, self.cohort_name, self.current_theme
        )
    }

    pub fn dashboard_status_summary(&self) -> String {
        format!(
            "Selected page: {}. Last dashboard action: {}",
            page_title(self.navigation.current_page),
            self.last_dashboard_action
        )
    }

    pub fn page_focus_prompt(&self, page: PageId) -> String {
        match page {
            PageId::Dashboard => {
                format!("Start with the dashboard, then branch into the focused concept pages, {}.", self.learner_name)
            }
            PageId::Layouts => "Spot the spacing, alignment, and nesting decisions before thinking about styling.".to_string(),
            PageId::ButtonsAndInputs => "Interact with the controls and discuss which values belong in Rust versus the UI layer.".to_string(),
            PageId::ListsAndModels => "Notice how repeated content and scrollable regions prepare the app for real data models.".to_string(),
            PageId::StylingAndThemes => format!("The current example theme is {}—use it to talk about visual hierarchy.", self.current_theme),
            PageId::WindowManagement => "Relate the shell header, navigation, content area, and footer to overall window composition.".to_string(),
            PageId::CrossPageData => format!("Edit the shared note or progress here, then watch the dashboard reflect the same Rust-owned values for {}.", self.cohort_name),
        }
    }

    pub fn shared_note_text(&self) -> &str {
        &self.shared_note_text
    }

    pub const fn shared_progress_value(&self) -> i32 {
        self.shared_progress_value
    }

    pub fn update_shared_note<S: AsRef<str>>(&mut self, value: S) {
        let trimmed = value.as_ref().trim();
        self.shared_note_text = if trimmed.is_empty() {
            "Empty input falls back to a teaching prompt so every page keeps a visible value."
                .to_string()
        } else {
            trimmed.to_string()
        };
    }

    pub fn update_shared_progress(&mut self, value: i32) {
        self.shared_progress_value =
            value.clamp(Self::min_progress_value(), Self::max_progress_value());
    }

    pub fn nudge_shared_progress(&mut self, delta: i32) {
        self.update_shared_progress(self.shared_progress_value.saturating_add(delta));
    }

    pub fn reset_cross_page_demo(&mut self) {
        self.shared_note_text = Self::default_note_text().to_string();
        self.shared_progress_value = Self::default_progress_value();
    }

    pub fn cross_page_view_model(&self) -> CrossPageViewModel {
        let progress_label = format!("Shared progress: {}% complete", self.shared_progress_value);
        CrossPageViewModel {
            note_text: self.shared_note_text.clone(),
            note_status: format!(
                "One-way example: edit the note on the Cross-page Data page and Rust republishes '{}' everywhere that binds to it.",
                self.shared_note_text
            ),
            progress_value: self.shared_progress_value,
            progress_label: progress_label.clone(),
            dashboard_preview: format!(
                "Watch update there → dashboard preview note: {}",
                self.shared_note_text
            ),
            editor_hint: "Edit here → callbacks send LineEdit and button changes into Rust before the shell pushes fresh property values back to every page.".to_string(),
            synchronization_notes: "Prefer direct property binding when a Slint-only value stays inside one component. Prefer callback-driven synchronization when Rust must validate input, share state across pages, or reset multiple widgets together. Rc<RefCell<_>> lets the shell share one owner-friendly state object across callbacks without fighting Rust lifetimes.".to_string(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SharedLearningState;
    use crate::navigation::PageId;

    #[test]
    fn shared_state_starts_with_instructional_defaults() {
        let state = SharedLearningState::default();

        assert_eq!(state.learner_name, "Curious Slint learner");
        assert_eq!(state.cohort_name, "Spring UI study group");
        assert_eq!(state.current_theme, "Studio Light");
        assert_eq!(state.navigation.current_page, PageId::Dashboard);
        assert!(state.last_dashboard_action.contains("sample-1.slint"));
        assert_eq!(
            state.shared_note_text(),
            SharedLearningState::default_note_text()
        );
        assert_eq!(
            state.shared_progress_value(),
            SharedLearningState::default_progress_value()
        );
    }

    #[test]
    fn updating_shared_state_from_the_source_api_changes_stored_values() {
        let mut state = SharedLearningState::default();

        state.update_shared_note("  Pair design review at 2pm  ");
        state.update_shared_progress(72);

        assert_eq!(state.shared_note_text(), "Pair design review at 2pm");
        assert_eq!(state.shared_progress_value(), 72);
    }

    #[test]
    fn cross_page_view_model_receives_latest_shared_values() {
        let mut state = SharedLearningState::default();
        state.update_shared_note("Shared preview is live");
        state.nudge_shared_progress(13);

        let view_model = state.cross_page_view_model();

        assert_eq!(view_model.note_text, "Shared preview is live");
        assert!(view_model
            .dashboard_preview
            .contains("Shared preview is live"));
        assert_eq!(view_model.progress_value, 48);
        assert!(view_model.progress_label.contains("48%"));
    }

    #[test]
    fn reset_restores_cross_page_defaults() {
        let mut state = SharedLearningState::default();
        state.update_shared_note("Temporary draft");
        state.update_shared_progress(91);

        state.reset_cross_page_demo();

        assert_eq!(
            state.shared_note_text(),
            SharedLearningState::default_note_text()
        );
        assert_eq!(
            state.shared_progress_value(),
            SharedLearningState::default_progress_value()
        );
    }

    #[test]
    fn empty_or_invalid_input_uses_demo_fallback_rules() {
        let mut state = SharedLearningState::default();
        state.update_shared_note("   ");
        state.update_shared_progress(999);

        assert_eq!(
            state.shared_note_text(),
            "Empty input falls back to a teaching prompt so every page keeps a visible value."
        );
        assert_eq!(
            state.shared_progress_value(),
            SharedLearningState::max_progress_value()
        );
    }
}
