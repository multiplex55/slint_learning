//! Navigation is intentionally plain Rust so it stays testable even before the UI grows.
//! Learners can inspect how page identifiers convert to indexes used by the `.slint` view.

/// Explicit page identifiers make future demos easier to read than raw integers.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LearningPage {
    WelcomeOverview,
    SharedStatePatterns,
    GeneratedCodePatterns,
}

impl LearningPage {
    pub const fn title(self) -> &'static str {
        match self {
            Self::WelcomeOverview => "Welcome overview",
            Self::SharedStatePatterns => "Shared state patterns",
            Self::GeneratedCodePatterns => "Generated code patterns",
        }
    }

    pub const fn all() -> [Self; 3] {
        [
            Self::WelcomeOverview,
            Self::SharedStatePatterns,
            Self::GeneratedCodePatterns,
        ]
    }

    pub const fn as_index(self) -> i32 {
        match self {
            Self::WelcomeOverview => 0,
            Self::SharedStatePatterns => 1,
            Self::GeneratedCodePatterns => 2,
        }
    }

    pub const fn from_index(index: i32) -> Option<Self> {
        match index {
            0 => Some(Self::WelcomeOverview),
            1 => Some(Self::SharedStatePatterns),
            2 => Some(Self::GeneratedCodePatterns),
            _ => None,
        }
    }
}

/// Keep mutable navigation state small and boring so the UI can bind to it safely.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NavigationState {
    pub current_page: LearningPage,
}

impl Default for NavigationState {
    fn default() -> Self {
        Self {
            current_page: LearningPage::WelcomeOverview,
        }
    }
}

impl NavigationState {
    pub fn select_page_by_index(&mut self, index: i32) {
        if let Some(page) = LearningPage::from_index(index) {
            self.current_page = page;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{LearningPage, NavigationState};

    #[test]
    fn default_application_page_is_the_welcome_overview() {
        let navigation = NavigationState::default();

        assert_eq!(navigation.current_page, LearningPage::WelcomeOverview);
    }

    #[test]
    fn page_enum_round_trips_through_ui_indexes() {
        for page in LearningPage::all() {
            assert_eq!(LearningPage::from_index(page.as_index()), Some(page));
        }

        assert_eq!(LearningPage::from_index(99), None);
    }
}
