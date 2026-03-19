//! Shared models live outside the UI so later lessons can add collections,
//! transformations, and app-wide state without tangling the rendering code.

use crate::navigation::{LearningPage, NavigationState};

/// A tiny but realistic shared state object that later demos can extend.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SharedLearningState {
    pub learner_name: String,
    pub completed_topics: Vec<String>,
    pub navigation: NavigationState,
}

impl Default for SharedLearningState {
    fn default() -> Self {
        Self {
            learner_name: "Curious Slint learner".to_string(),
            completed_topics: Vec::new(),
            navigation: NavigationState::default(),
        }
    }
}

impl SharedLearningState {
    /// This text is simple on purpose: it demonstrates plain Rust data shaping
    /// that a UI can consume without embedding formatting rules in the view layer.
    pub fn page_summary(&self, page: LearningPage) -> String {
        match page {
            LearningPage::WelcomeOverview => {
                format!("{} is starting with the welcome tour.", self.learner_name)
            }
            LearningPage::SharedStatePatterns => format!(
                "Shared state demo currently tracks {} completed topics.",
                self.completed_topics.len()
            ),
            LearningPage::GeneratedCodePatterns => {
                "Build scripts compile ui/app-window.slint into Rust before the app starts."
                    .to_string()
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::SharedLearningState;
    use crate::navigation::LearningPage;

    #[test]
    fn shared_app_state_starts_with_a_safe_learning_baseline() {
        let state = SharedLearningState::default();

        assert_eq!(state.learner_name, "Curious Slint learner");
        assert!(state.completed_topics.is_empty());
        assert_eq!(state.navigation.current_page, LearningPage::WelcomeOverview);
    }
}
