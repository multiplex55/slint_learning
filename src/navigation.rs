//! Plain Rust navigation types keep the shell testable outside the Slint runtime.
//! Add pages here first, then wire them into `ui/app-window.slint`.

/// Stable identifiers for every page hosted by the teaching shell.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum PageId {
    Dashboard,
    Layouts,
    ButtonsAndInputs,
    ListsAndModels,
    StylingAndThemes,
    WindowManagement,
    CrossPageData,
}

impl PageId {
    pub const ALL: [Self; 7] = [
        Self::Dashboard,
        Self::Layouts,
        Self::ButtonsAndInputs,
        Self::ListsAndModels,
        Self::StylingAndThemes,
        Self::WindowManagement,
        Self::CrossPageData,
    ];

    pub const fn as_index(self) -> i32 {
        match self {
            Self::Dashboard => 0,
            Self::Layouts => 1,
            Self::ButtonsAndInputs => 2,
            Self::ListsAndModels => 3,
            Self::StylingAndThemes => 4,
            Self::WindowManagement => 5,
            Self::CrossPageData => 6,
        }
    }

    pub const fn from_index(index: i32) -> Option<Self> {
        match index {
            0 => Some(Self::Dashboard),
            1 => Some(Self::Layouts),
            2 => Some(Self::ButtonsAndInputs),
            3 => Some(Self::ListsAndModels),
            4 => Some(Self::StylingAndThemes),
            5 => Some(Self::WindowManagement),
            6 => Some(Self::CrossPageData),
            _ => None,
        }
    }

    pub const fn meta(self) -> PageMeta {
        match self {
            Self::Dashboard => PageMeta {
                id: Self::Dashboard,
                title: "Dashboard",
                description: "Start with the shell, shared teaching context, and a quick tour of the demos.",
                notes: "Use this page to explain how the shell is organized before diving into specific widgets.",
                category: "Getting Started",
            },
            Self::Layouts => PageMeta {
                id: Self::Layouts,
                title: "Layouts",
                description: "Compare horizontal, vertical, grid-like, and nested layout patterns with spacing cues.",
                notes: "Focus on how containers size children and how alignment changes the learning experience.",
                category: "Layout Patterns",
            },
            Self::ButtonsAndInputs => PageMeta {
                id: Self::ButtonsAndInputs,
                title: "Buttons and Inputs",
                description: "Explore common form controls such as buttons, checkboxes, switches, text input, and selectors.",
                notes: "Ask learners which controls emit events, store values, or simply reflect existing state.",
                category: "Interactive Widgets",
            },
            Self::ListsAndModels => PageMeta {
                id: Self::ListsAndModels,
                title: "Lists and Models",
                description: "Show repeated elements, scrollable regions, and list-oriented presentation patterns.",
                notes: "This is a good bridge to later lessons about Rust models and dynamic data sources.",
                category: "Interactive Widgets",
            },
            Self::StylingAndThemes => PageMeta {
                id: Self::StylingAndThemes,
                title: "Styling and Themes",
                description: "Highlight cards, panels, color systems, and reusable visual treatments for teaching Slint styling.",
                notes: "Contrast visual grouping with layout grouping so learners can see style and structure separately.",
                category: "Presentation",
            },
            Self::WindowManagement => PageMeta {
                id: Self::WindowManagement,
                title: "Window Management",
                description: "Explain top-level shell concerns such as headers, status areas, and page-level workflow hints.",
                notes: "Keep the examples conceptual so the page remains useful on every desktop backend.",
                category: "Application Shell",
            },
            Self::CrossPageData => PageMeta {
                id: Self::CrossPageData,
                title: "Cross-page Data",
                description: "Demonstrate how one shared idea can be referenced from multiple pages without duplicating logic.",
                notes: "Use this page when introducing shared Rust state, callback wiring, and future page communication.",
                category: "Application Shell",
            },
        }
    }
}

/// Human-readable page metadata that can drive navigation, headers, or future search/filter UI.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PageMeta {
    pub id: PageId,
    pub title: &'static str,
    pub description: &'static str,
    pub notes: &'static str,
    pub category: &'static str,
}

pub const PAGE_REGISTRY: [PageMeta; 7] = [
    PageId::Dashboard.meta(),
    PageId::Layouts.meta(),
    PageId::ButtonsAndInputs.meta(),
    PageId::ListsAndModels.meta(),
    PageId::StylingAndThemes.meta(),
    PageId::WindowManagement.meta(),
    PageId::CrossPageData.meta(),
];

pub const fn page_registry() -> &'static [PageMeta; 7] {
    &PAGE_REGISTRY
}

pub const fn page_title(page: PageId) -> &'static str {
    page.meta().title
}

pub const fn page_description(page: PageId) -> &'static str {
    page.meta().description
}

pub const fn page_category(page: PageId) -> &'static str {
    page.meta().category
}

/// Mutable shell selection state stays tiny so it is straightforward to unit test.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct NavigationState {
    pub current_page: PageId,
}

impl Default for NavigationState {
    fn default() -> Self {
        Self {
            current_page: PageId::Dashboard,
        }
    }
}

impl NavigationState {
    pub fn select_page_by_index(&mut self, index: i32) {
        if let Some(page) = PageId::from_index(index) {
            self.current_page = page;
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::{
        page_category, page_description, page_registry, page_title, NavigationState, PageId,
    };

    #[test]
    fn all_page_ids_are_present_in_the_registry() {
        let registry_ids = page_registry()
            .iter()
            .map(|meta| meta.id)
            .collect::<BTreeSet<_>>();
        let declared_ids = PageId::ALL.into_iter().collect::<BTreeSet<_>>();

        assert_eq!(registry_ids, declared_ids);
    }

    #[test]
    fn page_titles_and_descriptions_are_non_empty() {
        for meta in page_registry() {
            assert!(
                !meta.title.trim().is_empty(),
                "missing title for {:?}",
                meta.id
            );
            assert!(
                !meta.description.trim().is_empty(),
                "missing description for {:?}",
                meta.id
            );
            assert!(
                !meta.notes.trim().is_empty(),
                "missing notes for {:?}",
                meta.id
            );
        }
    }

    #[test]
    fn default_page_ordering_is_stable() {
        let ordered_ids = page_registry().map(|meta| meta.id);

        assert_eq!(ordered_ids, PageId::ALL);
        assert_eq!(page_registry()[0].title, "Dashboard");
        assert_eq!(page_registry()[6].title, "Cross-page Data");
    }

    #[test]
    fn page_helpers_map_identifiers_to_expected_labels_and_categories() {
        assert_eq!(page_title(PageId::Layouts), "Layouts");
        assert_eq!(
            page_description(PageId::ButtonsAndInputs),
            "Explore common form controls such as buttons, checkboxes, switches, text input, and selectors."
        );
        assert_eq!(page_category(PageId::CrossPageData), "Application Shell");
    }

    #[test]
    fn default_navigation_state_uses_dashboard_page() {
        let navigation = NavigationState::default();

        assert_eq!(navigation.current_page, PageId::Dashboard);
    }

    #[test]
    fn page_enum_round_trips_through_ui_indexes() {
        for page in PageId::ALL {
            assert_eq!(PageId::from_index(page.as_index()), Some(page));
        }

        assert_eq!(PageId::from_index(99), None);
    }
}
