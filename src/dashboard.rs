//! Dashboard menu metadata and dispatching for the desktop-shell lesson.
//! Module responsibility: define menu item identifiers and translate a UI action string into a
//! small Rust command plus a teaching-oriented status message.
//! UI connection: `ui/pages/dashboard.slint` emits string action ids through a callback, and
//! `src/app.rs` calls the helpers here to decide whether to navigate or just update status text.
//! Study here: keeping command routing testable in plain Rust, especially when the visual menu is
//! built declaratively in Slint.

use crate::navigation::PageId;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RecentItem {
    pub id: &'static str,
    pub label: &'static str,
}

pub const fn default_recent_items() -> [RecentItem; 2] {
    [
        RecentItem {
            id: "file.recent.sample-1",
            label: "sample-1.slint",
        },
        RecentItem {
            id: "file.recent.sample-2",
            label: "sample-2.slint",
        },
    ]
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DashboardActionMeta {
    pub id: &'static str,
    pub label: &'static str,
    pub parent: Option<&'static str>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DashboardCommand {
    ShowStatus,
    Navigate(PageId),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DispatchedDashboardAction {
    pub id: &'static str,
    pub command: DashboardCommand,
    pub status_message: &'static str,
}

pub const DASHBOARD_ACTIONS: [DashboardActionMeta; 11] = [
    DashboardActionMeta {
        id: "file.new",
        label: "New",
        parent: Some("file"),
    },
    DashboardActionMeta {
        id: "file.open",
        label: "Open",
        parent: Some("file"),
    },
    DashboardActionMeta {
        id: "file.recent.sample-1",
        label: "sample-1.slint",
        parent: Some("file.recent"),
    },
    DashboardActionMeta {
        id: "file.recent.sample-2",
        label: "sample-2.slint",
        parent: Some("file.recent"),
    },
    DashboardActionMeta {
        id: "file.export.png",
        label: "PNG",
        parent: Some("file.export"),
    },
    DashboardActionMeta {
        id: "file.export.json",
        label: "JSON",
        parent: Some("file.export"),
    },
    DashboardActionMeta {
        id: "file.export.text",
        label: "Text",
        parent: Some("file.export"),
    },
    DashboardActionMeta {
        id: "file.exit",
        label: "Exit",
        parent: Some("file"),
    },
    DashboardActionMeta {
        id: "view.layouts",
        label: "Go to Layouts",
        parent: Some("view"),
    },
    DashboardActionMeta {
        id: "view.shared-state",
        label: "Go to Cross-page Data",
        parent: Some("view"),
    },
    DashboardActionMeta {
        id: "help.about-menus",
        label: "About this menu demo",
        parent: Some("help"),
    },
];

pub fn resolve_dashboard_action(id: &str) -> Option<&'static DashboardActionMeta> {
    DASHBOARD_ACTIONS.iter().find(|action| action.id == id)
}

pub fn dispatch_dashboard_action(id: &str) -> Option<DispatchedDashboardAction> {
    let action = resolve_dashboard_action(id)?;

    // Take note: the UI only knows about string ids. This Rust match translates those strings
    // into a typed command enum so later lessons can grow behavior without pushing logic into Slint.
    let command = match action.id {
        "view.layouts" => DashboardCommand::Navigate(PageId::Layouts),
        "view.shared-state" => DashboardCommand::Navigate(PageId::CrossPageData),
        _ => DashboardCommand::ShowStatus,
    };

    let status_message = match action.id {
        "file.new" => "File > New triggered a demo workspace reset.",
        "file.open" => "File > Open pretended to launch a file picker.",
        "file.recent.sample-1" => "File > Recent > sample-1.slint loaded a mock recent document.",
        "file.recent.sample-2" => "File > Recent > sample-2.slint loaded a mock recent document.",
        "file.export.png" => "File > Export > PNG simulated a desktop export command.",
        "file.export.json" => "File > Export > JSON simulated a structured data export.",
        "file.export.text" => "File > Export > Text simulated a plain-text export.",
        "file.exit" => "File > Exit was intercepted so the teaching demo stays open.",
        "view.layouts" => "View switched the shell to the Layouts page.",
        "view.shared-state" => "View switched the shell to the Cross-page Data page.",
        "help.about-menus" => {
            "Help explained that the dashboard uses a custom Slint teaching menu instead of native desktop menus."
        }
        _ => return None,
    };

    Some(DispatchedDashboardAction {
        id: action.id,
        command,
        status_message,
    })
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::{
        default_recent_items, dispatch_dashboard_action, resolve_dashboard_action,
        DashboardCommand, DASHBOARD_ACTIONS,
    };
    use crate::navigation::PageId;

    #[test]
    fn every_dashboard_action_maps_to_the_expected_command_event() {
        let cases = [
            ("file.new", DashboardCommand::ShowStatus),
            ("file.open", DashboardCommand::ShowStatus),
            ("file.recent.sample-1", DashboardCommand::ShowStatus),
            ("file.recent.sample-2", DashboardCommand::ShowStatus),
            ("file.export.png", DashboardCommand::ShowStatus),
            ("file.export.json", DashboardCommand::ShowStatus),
            ("file.export.text", DashboardCommand::ShowStatus),
            ("file.exit", DashboardCommand::ShowStatus),
            ("view.layouts", DashboardCommand::Navigate(PageId::Layouts)),
            (
                "view.shared-state",
                DashboardCommand::Navigate(PageId::CrossPageData),
            ),
            ("help.about-menus", DashboardCommand::ShowStatus),
        ];

        for (id, expected_command) in cases {
            let dispatched = dispatch_dashboard_action(id).expect("action should resolve");
            assert_eq!(
                dispatched.command, expected_command,
                "unexpected command for {id}"
            );
            assert!(
                !dispatched.status_message.trim().is_empty(),
                "status message should describe {id}"
            );
        }
    }

    #[test]
    fn nested_submenu_identifiers_are_unique_and_resolvable() {
        let ids = DASHBOARD_ACTIONS
            .iter()
            .map(|action| action.id)
            .collect::<BTreeSet<_>>();

        assert_eq!(ids.len(), DASHBOARD_ACTIONS.len());

        for nested_id in [
            "file.recent.sample-1",
            "file.recent.sample-2",
            "file.export.png",
            "file.export.json",
            "file.export.text",
        ] {
            let action = resolve_dashboard_action(nested_id).expect("nested action should exist");
            assert!(
                action.parent.is_some(),
                "nested action should have a parent"
            );
        }
    }

    #[test]
    fn recent_items_demo_model_starts_with_expected_entries() {
        let items = default_recent_items();

        assert_eq!(items.len(), 2);
        assert_eq!(items[0].id, "file.recent.sample-1");
        assert_eq!(items[0].label, "sample-1.slint");
        assert_eq!(items[1].id, "file.recent.sample-2");
        assert_eq!(items[1].label, "sample-2.slint");
    }
}
