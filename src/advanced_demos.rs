//! Rust-side helper state for the more advanced teaching pages.
//! Module responsibility: keep data transformation, selection state, and small registries out of
//! the `.slint` files so the examples can compare declarative UI with testable Rust helpers.

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LessonRecord {
    pub id: &'static str,
    pub title: &'static str,
    pub detail: &'static str,
    pub category: &'static str,
    pub recommended: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ListRowViewData {
    pub title: String,
    pub detail: String,
    pub badge: String,
    pub status: String,
    pub selected: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ThemeChoice {
    pub id: &'static str,
    pub label: &'static str,
    pub summary: &'static str,
    pub accent_hex: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ComponentMeta {
    pub id: &'static str,
    pub title: &'static str,
    pub category: &'static str,
    pub default_visible: bool,
    pub recommended: bool,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PatternNote {
    pub title: &'static str,
    pub category: &'static str,
    pub recommendation: &'static str,
    pub less_ideal: &'static str,
}

pub const COMPONENT_REGISTRY: [ComponentMeta; 4] = [
    ComponentMeta {
        id: "stat-card",
        title: "Stat card",
        category: "dashboard",
        default_visible: true,
        recommended: true,
    },
    ComponentMeta {
        id: "list-row",
        title: "List row",
        category: "lists",
        default_visible: true,
        recommended: true,
    },
    ComponentMeta {
        id: "theme-chip",
        title: "Theme chip",
        category: "styling",
        default_visible: true,
        recommended: true,
    },
    ComponentMeta {
        id: "legacy-panel",
        title: "Legacy panel",
        category: "comparison",
        default_visible: false,
        recommended: false,
    },
];

pub const PERFORMANCE_NOTES: [PatternNote; 4] = [
    PatternNote {
        title: "Reuse visual building blocks",
        category: "components",
        recommendation: "Prefer one reusable row/card component so style and spacing fixes happen in one place.",
        less_ideal: "Copy-pasting rectangles across pages makes style drift and maintenance churn more likely.",
    },
    PatternNote {
        title: "Centralize shared state when multiple pages care",
        category: "state",
        recommendation: "Keep truly shared selections, filters, or theme choices in Rust so every page sees the same source of truth.",
        less_ideal: "Duplicating near-identical page-local state makes reset flows and cross-page consistency harder.",
    },
    PatternNote {
        title: "Avoid unnecessary UI churn",
        category: "updates",
        recommendation: "Refresh only the parts of a model that conceptually changed and prefer bindings for derived labels.",
        less_ideal: "Rebuilding every label or toggling unrelated properties on each click creates noise during learning and in real apps.",
    },
    PatternNote {
        title: "Keep page logic explainable",
        category: "maintainability",
        recommendation: "Move transformation helpers and registries into small Rust helpers with unit tests.",
        less_ideal: "Embedding all branching and data prep inline in `.slint` pages becomes harder to review as demos grow.",
    },
];

pub fn default_lessons() -> Vec<LessonRecord> {
    vec![
        LessonRecord {
            id: "intro-bindings",
            title: "Property bindings keep labels derived",
            detail: "The UI can derive status text from canonical item data instead of imperatively editing every label.",
            category: "Recommended",
            recommended: true,
        },
        LessonRecord {
            id: "selection-state",
            title: "Selection belongs in one place",
            detail: "Rust tracks the selected row so multiple widgets can react without each row inventing its own truth.",
            category: "Recommended",
            recommended: true,
        },
        LessonRecord {
            id: "refresh-pattern",
            title: "Rust-backed refresh updates model data",
            detail: "A button can simulate new backend data by swapping statuses and counts while the list view stays declarative.",
            category: "Recommended",
            recommended: true,
        },
        LessonRecord {
            id: "copy-paste-warning",
            title: "Duplicated row markup is harder to maintain",
            detail: "This less-ideal pattern is shown as a teaching note, not something to benchmark or emulate for performance claims.",
            category: "Tradeoff",
            recommended: false,
        },
    ]
}

pub fn transform_lessons_for_ui(
    records: &[LessonRecord],
    selected_index: Option<usize>,
) -> Vec<ListRowViewData> {
    records
        .iter()
        .enumerate()
        .map(|(index, record)| ListRowViewData {
            title: format!("{}. {}", index + 1, record.title),
            detail: record.detail.to_string(),
            badge: record.category.to_string(),
            status: if Some(index) == selected_index {
                format!("Focused example • {}", record.id)
            } else if record.recommended {
                "Reusable/data-driven pattern".to_string()
            } else {
                "Less-ideal comparison example".to_string()
            },
            selected: Some(index) == selected_index,
        })
        .collect()
}

pub fn default_theme_choices() -> Vec<ThemeChoice> {
    vec![
        ThemeChoice {
            id: "studio-light",
            label: "Studio Light",
            summary: "Balanced blues and slate neutrals for teaching hierarchy.",
            accent_hex: "#2563eb",
        },
        ThemeChoice {
            id: "forest-notes",
            label: "Forest Notes",
            summary: "A softer green accent that shows how spacing can stay stable while colors change.",
            accent_hex: "#15803d",
        },
        ThemeChoice {
            id: "sunset-contrast",
            label: "Sunset Contrast",
            summary: "A warm accent variant that helps compare semantic emphasis without changing layout.",
            accent_hex: "#c2410c",
        },
    ]
}

pub fn visible_component_count(meta: &[ComponentMeta]) -> usize {
    meta.iter().filter(|entry| entry.default_visible).count()
}

pub fn component_categories(meta: &[ComponentMeta]) -> Vec<&'static str> {
    let mut categories = meta.iter().map(|entry| entry.category).collect::<Vec<_>>();
    categories.sort_unstable();
    categories.dedup();
    categories
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AdvancedDemosState {
    pub selected_list_index: usize,
    pub refresh_count: usize,
    pub selected_theme_id: String,
    pub playground_show_details: bool,
    pub playground_loading: bool,
}

impl Default for AdvancedDemosState {
    fn default() -> Self {
        Self {
            selected_list_index: 0,
            refresh_count: 0,
            selected_theme_id: default_theme_choices()[0].id.to_string(),
            playground_show_details: true,
            playground_loading: false,
        }
    }
}

impl AdvancedDemosState {
    pub fn select_list_index(&mut self, index: i32) {
        if let Ok(index) = usize::try_from(index) {
            self.selected_list_index = index.min(default_lessons().len().saturating_sub(1));
        }
    }

    pub fn refresh_lessons(&mut self) {
        self.refresh_count = self.refresh_count.saturating_add(1);
    }

    pub fn lesson_rows(&self) -> Vec<ListRowViewData> {
        let mut rows = transform_lessons_for_ui(&default_lessons(), Some(self.selected_list_index));
        for (index, row) in rows.iter_mut().enumerate() {
            if index == self.selected_list_index {
                row.status = format!(
                    "Focused example • refresh #{}, recommended data flow remains centralized in Rust",
                    self.refresh_count + 1
                );
            }
        }
        rows
    }

    pub fn select_theme(&mut self, id: &str) {
        if default_theme_choices().iter().any(|choice| choice.id == id) {
            self.selected_theme_id = id.to_string();
        }
    }

    pub fn selected_theme(&self) -> ThemeChoice {
        default_theme_choices()
            .into_iter()
            .find(|choice| choice.id == self.selected_theme_id)
            .unwrap_or_else(|| default_theme_choices()[0].clone())
    }

    pub fn toggle_playground_details(&mut self) {
        self.playground_show_details = !self.playground_show_details;
    }

    pub fn toggle_playground_loading(&mut self) {
        self.playground_loading = !self.playground_loading;
    }

    pub fn loading_label(&self) -> &'static str {
        if self.playground_loading {
            "Mock data status: loading placeholder is visible."
        } else {
            "Mock data status: placeholder is hidden and content is ready."
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{
        component_categories, default_theme_choices, transform_lessons_for_ui,
        visible_component_count, AdvancedDemosState, COMPONENT_REGISTRY,
    };

    #[test]
    fn list_transformation_marks_selected_row_and_tradeoff_copy() {
        let rows = transform_lessons_for_ui(&super::default_lessons(), Some(2));

        assert_eq!(rows.len(), 4);
        assert!(rows[2].selected);
        assert!(rows[2].status.contains("Focused example"));
        assert_eq!(rows[3].status, "Less-ideal comparison example");
    }

    #[test]
    fn theme_selection_only_accepts_registered_options() {
        let mut state = AdvancedDemosState::default();
        state.select_theme("forest-notes");
        assert_eq!(state.selected_theme().label, "Forest Notes");

        state.select_theme("missing-theme");
        assert_eq!(state.selected_theme().label, "Forest Notes");
        assert_eq!(default_theme_choices().len(), 3);
    }

    #[test]
    fn component_registry_exposes_expected_metadata_defaults() {
        assert_eq!(COMPONENT_REGISTRY.len(), 4);
        assert_eq!(visible_component_count(&COMPONENT_REGISTRY), 3);
        assert!(COMPONENT_REGISTRY
            .iter()
            .any(|entry| !entry.default_visible));
        assert!(COMPONENT_REGISTRY.iter().any(|entry| entry.recommended));
    }

    #[test]
    fn helper_logic_groups_categories_and_default_visibility() {
        let categories = component_categories(&COMPONENT_REGISTRY);
        assert_eq!(
            categories,
            vec!["comparison", "dashboard", "lists", "styling"]
        );

        let state = AdvancedDemosState::default();
        assert!(state.playground_show_details);
        assert!(!state.playground_loading);
        assert!(state.loading_label().contains("hidden"));
    }
}
