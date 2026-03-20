//! Shared learning data sits outside the `.slint` files so later lessons can connect
//! richer state without rewriting the teaching shell.

use crate::dashboard::default_recent_items;
use crate::navigation::{page_title, NavigationState, PageId};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SharedLearningState {
    pub learner_name: String,
    pub cohort_name: String,
    pub current_theme: String,
    pub navigation: NavigationState,
    pub last_dashboard_action: String,
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
        }
    }
}

impl SharedLearningState {
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
            PageId::CrossPageData => format!("Reuse shared context like '{}' without duplicating it inside every page.", self.cohort_name),
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
    }
}
