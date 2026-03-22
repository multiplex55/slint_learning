//! Page registry and navigation helpers for the learning shell.
//! Module responsibility: define the stable set of pages, their metadata, and the small amount of
//! selection state the application needs.
//! UI connection: `ui/app-window.slint` emits page indices, while `src/app.rs` and the dashboard
//! translate those indices back into `PageId` values and display-friendly labels.
//! Study here: enum-to-index conversion, metadata registries, and how to keep view navigation
//! deterministic and unit-testable outside the Slint runtime.

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
    PerformanceAndBestPractices,
    Playground,
}

impl PageId {
    pub const ALL: [Self; 9] = [
        Self::Dashboard,
        Self::Layouts,
        Self::ButtonsAndInputs,
        Self::ListsAndModels,
        Self::StylingAndThemes,
        Self::WindowManagement,
        Self::CrossPageData,
        Self::PerformanceAndBestPractices,
        Self::Playground,
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
            Self::PerformanceAndBestPractices => 7,
            Self::Playground => 8,
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
            7 => Some(Self::PerformanceAndBestPractices),
            8 => Some(Self::Playground),
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
                description: "Show repeated elements, scrollable regions, and Rust-backed list/model presentation patterns.",
                notes: "Use this page to compare repeated Slint markup with data prepared by small Rust transformation helpers.",
                category: "Interactive Widgets",
            },
            Self::StylingAndThemes => PageMeta {
                id: Self::StylingAndThemes,
                title: "Styling and Themes",
                description: "Highlight colors, spacing, component reuse, and theme-like customization patterns.",
                notes: "Contrast stable layout structure with theme changes so learners can separate style choices from logic.",
                category: "Presentation",
            },
            Self::WindowManagement => PageMeta {
                id: Self::WindowManagement,
                title: "Window Management",
                description: "Open, hide, reuse, and recreate multiple top-level windows while keeping a Rust controller testable.",
                notes: "Compare modeless reused windows with recreated utility windows and discuss ownership of handles.",
                category: "Application Shell",
            },
            Self::CrossPageData => PageMeta {
                id: Self::CrossPageData,
                title: "Cross-page Data",
                description: "Demonstrate how one shared idea can be referenced from multiple pages without duplicating logic.",
                notes: "Use this page when introducing shared Rust state, callback wiring, and future page communication.",
                category: "Application Shell",
            },
            Self::PerformanceAndBestPractices => PageMeta {
                id: Self::PerformanceAndBestPractices,
                title: "Performance and Best Practices",
                description: "Teaching notes about reusable components, centralized state, and avoiding unnecessary UI churn.",
                notes: "Keep expectations grounded: these examples explain maintainability and update patterns, not precise benchmark results.",
                category: "Architecture",
            },
            Self::Playground => PageMeta {
                id: Self::Playground,
                title: "Playground",
                description: "Try bindings, imperative updates, conditional visibility, animations, and mock loading states in one place.",
                notes: "Use this page for guided experiments that compare recommended declarative patterns with small imperative examples.",
                category: "Hands-on Lab",
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

pub const PAGE_REGISTRY: [PageMeta; 9] = [
    PageId::Dashboard.meta(),
    PageId::Layouts.meta(),
    PageId::ButtonsAndInputs.meta(),
    PageId::ListsAndModels.meta(),
    PageId::StylingAndThemes.meta(),
    PageId::WindowManagement.meta(),
    PageId::CrossPageData.meta(),
    PageId::PerformanceAndBestPractices.meta(),
    PageId::Playground.meta(),
];

pub const fn page_registry() -> &'static [PageMeta; 9] {
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
    fn registered_pages_include_required_descriptive_fields() {
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
            assert!(
                !meta.category.trim().is_empty(),
                "missing category for {:?}",
                meta.id
            );
        }
    }

    #[test]
    fn default_page_ordering_is_stable() {
        let ordered_ids = page_registry().map(|meta| meta.id);

        assert_eq!(ordered_ids, PageId::ALL);
        assert_eq!(page_registry()[0].title, "Dashboard");
        assert_eq!(page_registry()[8].title, "Playground");
    }

    #[test]
    fn central_page_registry_is_internally_consistent() {
        for (expected_index, meta) in page_registry().iter().enumerate() {
            assert_eq!(
                meta.id.as_index(),
                expected_index as i32,
                "registry index mismatch for {:?}",
                meta.id
            );
            assert_eq!(
                PageId::from_index(expected_index as i32),
                Some(meta.id),
                "from_index mismatch for {:?}",
                meta.id
            );
            assert_eq!(
                *meta,
                meta.id.meta(),
                "registry metadata drifted from PageId::meta() for {:?}",
                meta.id
            );
        }
    }

    #[test]
    fn page_helpers_map_identifiers_to_expected_labels_and_categories() {
        assert_eq!(page_title(PageId::Layouts), "Layouts");
        assert_eq!(
            page_description(PageId::ButtonsAndInputs),
            "Explore common form controls such as buttons, checkboxes, switches, text input, and selectors."
        );
        assert_eq!(page_category(PageId::WindowManagement), "Application Shell");
        assert_eq!(
            page_category(PageId::PerformanceAndBestPractices),
            "Architecture"
        );
    }

    #[test]
    fn selecting_invalid_page_index_preserves_previous_selection() {
        let mut state = NavigationState::default();
        state.select_page_by_index(PageId::ListsAndModels.as_index());
        state.select_page_by_index(99);

        assert_eq!(state.current_page, PageId::ListsAndModels);
    }
}
