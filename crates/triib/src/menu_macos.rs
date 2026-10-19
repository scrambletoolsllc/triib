//! The macOS menu bar: the app menu, then File, Edit, View, Window and
//! Help, a basic set after Hive's. Each item does what its toolbar tool
//! does, and View checks the view and panels shown.
//!
//! Cut, Copy, Paste and Select All are the system's own items, which hand
//! their keys on to triib's text fields (see scramble-ui's winit patch).

use std::cell::RefCell;

use muda::accelerator::{Accelerator, Code, Modifiers};
use muda::{CheckMenuItem, Menu, MenuEvent, MenuItem, PredefinedMenuItem, Submenu};

use crate::fl;
use crate::settings::{Settings, View};

/// What a menu item asks of the app.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Item {
    Settings,
    Quit,
    Presets,
    CloseWindow,
    View(View),
    Log,
    Inspector,
    Website,
    ReleaseNotes,
}

/// Menu item id and what it asks.
const ITEMS: &[(&str, Item)] = &[
    ("settings", Item::Settings),
    ("quit", Item::Quit),
    ("presets", Item::Presets),
    ("close", Item::CloseWindow),
    ("connections", Item::View(View::Matrix)),
    ("network", Item::View(View::Network)),
    ("entities", Item::View(View::Entities)),
    ("log", Item::Log),
    ("inspector", Item::Inspector),
    ("website", Item::Website),
    ("release-notes", Item::ReleaseNotes),
];

/// The menu bar in use, which AppKit only borrows, and its checked items.
struct Bar {
    menu: Menu,
    views: [(View, CheckMenuItem); 3],
    log: CheckMenuItem,
    inspector: CheckMenuItem,
}

thread_local! {
    static BAR: RefCell<Option<Bar>> = const { RefCell::new(None) };
}

/// Sends each item chosen to the app.
pub fn set_handler() {
    MenuEvent::set_event_handler(Some(|event: MenuEvent| {
        if let Some((_, item)) = ITEMS.iter().find(|(id, _)| *id == event.id.as_ref()) {
            crate::post(crate::External::Menu(*item));
        }
    }));
}

/// Builds the menu bar in the interface's language and puts it up, its
/// checks as `settings` has them. Called at start and when the language
/// changes; on the main thread.
pub fn install(settings: &Settings) {
    match build(settings) {
        Ok(bar) => {
            bar.menu.init_for_nsapp();
            BAR.with(|slot| *slot.borrow_mut() = Some(bar));
        }
        Err(error) => eprintln!("triib: no menu bar: {error}"),
    }
}

/// Checks the view and panels `settings` shows, wherever they changed.
pub fn check(settings: &Settings) {
    BAR.with(|slot| {
        let Some(bar) = &*slot.borrow() else {
            return;
        };
        let set = |item: &CheckMenuItem, checked: bool| {
            if item.is_checked() != checked {
                item.set_checked(checked);
            }
        };
        for (view, item) in &bar.views {
            set(item, *view == settings.view);
        }
        set(&bar.log, settings.log);
        set(&bar.inspector, settings.inspector);
    });
}

fn item(id: &str, label: String, key: Option<Accelerator>) -> MenuItem {
    MenuItem::with_id(id, label, true, key)
}

fn checked(id: &str, label: String, on: bool) -> CheckMenuItem {
    CheckMenuItem::with_id(id, label, true, on, None)
}

fn command(code: Code) -> Option<Accelerator> {
    Some(Accelerator::new(Modifiers::META, code))
}

fn build(settings: &Settings) -> muda::Result<Bar> {
    let separator = PredefinedMenuItem::separator;
    let app = Submenu::with_items(
        "triib",
        true,
        &[
            &PredefinedMenuItem::about(Some(&fl!("menu-about")), None),
            &separator(),
            &item("settings", fl!("menu-settings"), command(Code::Comma)),
            &separator(),
            &PredefinedMenuItem::services(Some(&fl!("menu-services"))),
            &separator(),
            &PredefinedMenuItem::hide(Some(&fl!("menu-hide"))),
            &PredefinedMenuItem::hide_others(Some(&fl!("menu-hide-others"))),
            &PredefinedMenuItem::show_all(Some(&fl!("menu-show-all"))),
            &separator(),
            &item("quit", fl!("menu-quit"), command(Code::KeyQ)),
        ],
    )?;
    let file = Submenu::with_items(
        fl!("menu-file"),
        true,
        &[
            &item("presets", fl!("menu-presets"), None),
            &separator(),
            &item("close", fl!("menu-close-window"), command(Code::KeyW)),
        ],
    )?;
    let edit = Submenu::with_items(
        fl!("menu-edit"),
        true,
        &[
            &PredefinedMenuItem::cut(Some(&fl!("menu-cut"))),
            &PredefinedMenuItem::copy(Some(&fl!("menu-copy"))),
            &PredefinedMenuItem::paste(Some(&fl!("menu-paste"))),
            &PredefinedMenuItem::select_all(Some(&fl!("menu-select-all"))),
        ],
    )?;
    let views = [
        (View::Matrix, "connections", fl!("toolbar-connections")),
        (View::Network, "network", fl!("toolbar-network")),
        (View::Entities, "entities", fl!("toolbar-entities")),
    ]
    .map(|(view, id, label)| (view, checked(id, label, settings.view == view)));
    let log = checked("log", fl!("toolbar-log"), settings.log);
    let inspector = checked("inspector", fl!("toolbar-inspector"), settings.inspector);
    let view = Submenu::with_items(
        fl!("menu-view"),
        true,
        &[
            &views[0].1,
            &views[1].1,
            &views[2].1,
            &separator(),
            &log,
            &inspector,
            &separator(),
            &PredefinedMenuItem::fullscreen(Some(&fl!("menu-full-screen"))),
        ],
    )?;
    let window = Submenu::with_items(
        fl!("menu-window"),
        true,
        &[
            &PredefinedMenuItem::minimize(Some(&fl!("menu-minimize"))),
            &PredefinedMenuItem::maximize(Some(&fl!("menu-zoom"))),
            &separator(),
            &PredefinedMenuItem::bring_all_to_front(Some(&fl!("menu-bring-all-to-front"))),
        ],
    )?;
    let help = Submenu::with_items(
        fl!("menu-help"),
        true,
        &[
            &item("website", fl!("menu-website"), None),
            &item("release-notes", fl!("menu-release-notes"), None),
        ],
    )?;
    let menu = Menu::new();
    for submenu in [&app, &file, &edit, &view, &window, &help] {
        menu.append(submenu)?;
    }
    window.set_as_windows_menu_for_nsapp();
    help.set_as_help_menu_for_nsapp();
    Ok(Bar {
        menu,
        views,
        log,
        inspector,
    })
}
