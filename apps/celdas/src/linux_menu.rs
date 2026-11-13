//! The Linux desktop's global menu: Celdas's menus (`menu_tree.rs`) exported over D-Bus as a
//! `com.canonical.dbusmenu` object and registered with the desktop's
//! `com.canonical.AppMenu.Registrar`, so a top bar that shows the focused app's menus (Nodalix's)
//! shows Celdas's. The ribbon stays in the window either way: the global menu only adds to it.
//!
//! A window on Wayland has no global id, so the registration names this process instead, and only
//! a registrar that pairs a menu with its window by process can show it: Nodalix's, whose process
//! also owns `space.unmade.GlobalMenu`. With any other registrar, or none, nothing is registered.

use std::collections::HashMap;
use std::sync::{Arc, Mutex, PoisonError};

use celdas_ui_egui::SheetApp;
use zbus::blocking::Connection;
use zbus::zvariant::{OwnedValue, StructureBuilder, Value};

use crate::menu_tree::{self, Entry};

const REGISTRAR: &str = "com.canonical.AppMenu.Registrar";
const REGISTRAR_PATH: &str = "/com/canonical/AppMenu/Registrar";
const NODALIX_MENU: &str = "space.unmade.GlobalMenu";
const MENU_PATH: &str = "/com/getnodalia/GlobalMenu";

/// One row of the exported menu.
#[derive(Clone, Debug, PartialEq)]
enum Row {
    Item { label: String, id: &'static str },
    Separator,
    Submenu { label: String, children: Vec<i32> },
}

/// The menus as the desktop reads them: rows by id (0 is the root, whose children are the
/// top-level menus), and the items chosen since the app last looked.
#[derive(Default)]
struct Shared {
    rows: HashMap<i32, Row>,
    root: Vec<i32>,
    chosen: Vec<&'static str>,
}

impl Shared {
    /// Number the rows of `menus` depth first, from 1.
    fn of(menus: &[(&'static str, Vec<Entry>)]) -> Shared {
        let (mut rows, mut next, mut root) = (HashMap::new(), 1i32, Vec::new());
        for (title, entries) in menus {
            let id = next;
            next = next.saturating_add(1);
            let mut children = Vec::new();
            for &(label, command, _) in entries {
                let row = if label == "-" { Row::Separator } else { Row::Item { label: label.to_string(), id: command } };
                rows.insert(next, row);
                children.push(next);
                next = next.saturating_add(1);
            }
            rows.insert(id, Row::Submenu { label: title.to_string(), children });
            root.push(id);
        }
        Shared { rows, root, chosen: Vec::new() }
    }

    fn children(&self, id: i32) -> &[i32] {
        match self.rows.get(&id) {
            _ if id == 0 => &self.root,
            Some(Row::Submenu { children, .. }) => children,
            _ => &[],
        }
    }

    /// `id`'s dbusmenu properties.
    fn properties(&self, id: i32) -> HashMap<String, OwnedValue> {
        let mut p = HashMap::new();
        let mut put = |k: &str, v: Value<'_>| {
            if let Ok(v) = v.try_to_owned() {
                p.insert(k.to_string(), v);
            }
        };
        match self.rows.get(&id) {
            None if id == 0 => put("children-display", Value::from("submenu")),
            None => {}
            Some(Row::Separator) => put("type", Value::from("separator")),
            Some(Row::Submenu { label, .. }) => {
                put("label", Value::from(mnemonic_free(label)));
                put("children-display", Value::from("submenu"));
            }
            Some(Row::Item { label, .. }) => {
                put("label", Value::from(mnemonic_free(label)));
                put("enabled", Value::from(true));
            }
        }
        p
    }

    /// `id` and `depth` levels of its children (`-1`: all), the reply to `GetLayout`.
    fn layout_item(&self, id: i32, depth: i32) -> LayoutItem {
        let children = if depth == 0 {
            Vec::new()
        } else {
            self.children(id).iter().filter_map(|&c| self.layout(c, depth.saturating_sub(1)).try_to_owned().ok()).collect()
        };
        LayoutItem { id, properties: self.properties(id), children }
    }

    /// [`Shared::layout_item`] as a variant's value: how dbusmenu nests the children.
    fn layout(&self, id: i32, depth: i32) -> Value<'static> {
        let children: Vec<Value<'static>> =
            if depth == 0 { Vec::new() } else { self.children(id).iter().map(|&c| self.layout(c, depth.saturating_sub(1))).collect() };
        let children = zbus::zvariant::Array::from(children);
        let built = StructureBuilder::new().add_field(id).add_field(self.properties(id)).append_field(Value::Array(children)).build();
        built.map(Value::Structure).unwrap_or_else(|_| Value::from(0))
    }
}

/// A label with its underscores doubled: dbusmenu reads a single one as a mnemonic.
fn mnemonic_free(label: &str) -> String {
    label.replace('_', "__")
}

/// One item of the menu tree as dbusmenu sends it: `(ia{sv}av)`, each child a variant holding the
/// same structure.
#[derive(Debug, serde::Serialize, zbus::zvariant::Type)]
struct LayoutItem {
    id: i32,
    properties: HashMap<String, OwnedValue>,
    children: Vec<OwnedValue>,
}

/// The `com.canonical.dbusmenu` object.
struct MenuObject {
    shared: Arc<Mutex<Shared>>,
    ctx: egui::Context,
}

impl MenuObject {
    fn lock(&self) -> std::sync::MutexGuard<'_, Shared> {
        self.shared.lock().unwrap_or_else(PoisonError::into_inner)
    }

    fn chosen(&self, id: i32, kind: &str) {
        if kind != "clicked" {
            return;
        }
        let mut s = self.lock();
        if let Some(Row::Item { id: command, .. }) = s.rows.get(&id) {
            let command = *command;
            s.chosen.push(command);
            drop(s);
            self.ctx.request_repaint();
        }
    }
}

#[zbus::interface(name = "com.canonical.dbusmenu")]
impl MenuObject {
    #[zbus(property)]
    fn version(&self) -> u32 {
        3
    }
    #[zbus(property)]
    fn text_direction(&self) -> String {
        "ltr".into()
    }
    #[zbus(property)]
    fn status(&self) -> String {
        "normal".into()
    }
    #[zbus(property)]
    fn icon_theme_path(&self) -> Vec<String> {
        Vec::new()
    }

    fn get_layout(&self, parent_id: i32, recursion_depth: i32, _property_names: Vec<String>) -> (u32, LayoutItem) {
        (1, self.lock().layout_item(parent_id, recursion_depth))
    }

    fn get_group_properties(&self, ids: Vec<i32>, _property_names: Vec<String>) -> Vec<(i32, HashMap<String, OwnedValue>)> {
        let s = self.lock();
        // An empty list asks for every item (the dbusmenu contract).
        let ids = if ids.is_empty() { s.rows.keys().copied().collect() } else { ids };
        ids.into_iter().filter(|id| s.rows.contains_key(id)).map(|id| (id, s.properties(id))).collect()
    }

    fn get_property(&self, id: i32, name: String) -> OwnedValue {
        self.lock().properties(id).remove(&name).unwrap_or_else(|| OwnedValue::from(0i32))
    }

    fn event(&self, id: i32, event_id: String, _data: OwnedValue, _timestamp: u32) {
        self.chosen(id, &event_id);
    }

    fn event_group(&self, events: Vec<(i32, String, OwnedValue, u32)>) -> Vec<i32> {
        for (id, kind, _, _) in events {
            self.chosen(id, &kind);
        }
        Vec::new()
    }

    fn about_to_show(&self, _id: i32) -> bool {
        false
    }

    fn about_to_show_group(&self, ids: Vec<i32>) -> (Vec<i32>, Vec<i32>) {
        let s = self.lock();
        let unknown = ids.into_iter().filter(|id| *id != 0 && !s.rows.contains_key(id)).collect();
        (Vec::new(), unknown)
    }
}

/// The exported menu, kept alive with its connection.
pub struct GlobalMenu {
    _conn: Connection,
    shared: Arc<Mutex<Shared>>,
}

impl GlobalMenu {
    /// Runs the items chosen in the desktop's top bar. `viewport` is the active window's viewport.
    pub fn poll(&self, app: &mut SheetApp, ctx: &egui::Context, viewport: egui::ViewportId) {
        let chosen = std::mem::take(&mut self.shared.lock().unwrap_or_else(PoisonError::into_inner).chosen);
        for id in chosen {
            menu_tree::dispatch(app, ctx, viewport, id);
        }
    }
}

/// Whether the registrar on the bus is Nodalix's, the one registrar that finds a Wayland
/// window's menu by its process.
fn nodalix_registrar(conn: &Connection) -> bool {
    let Ok(dbus) = zbus::blocking::fdo::DBusProxy::new(conn) else { return false };
    let owner = |name: &str| zbus::names::BusName::try_from(name).ok().and_then(|n| dbus.get_name_owner(n).ok()).map(|o| o.to_string());
    matches!((owner(REGISTRAR), owner(NODALIX_MENU)), (Some(r), Some(n)) if r == n)
}

/// Hand the menu to the registrar, when it is Nodalix's.
fn register(conn: &Connection) {
    let path = zbus::zvariant::ObjectPath::from_static_str_unchecked(MENU_PATH);
    let accepted = nodalix_registrar(conn)
        && conn.call_method(Some(REGISTRAR), REGISTRAR_PATH, Some(REGISTRAR), "RegisterWindow", &(std::process::id(), path)).is_ok();
    log::info!("global menu: {}", if accepted { "shown in the desktop's top bar" } else { "no Nodalix top bar to show it" });
}

/// Every menu of the global menu: Celdas's own first, then the menu bar's.
fn menus() -> Vec<(&'static str, Vec<Entry>)> {
    std::iter::once(menu_tree::app_menu()).chain(menu_tree::tree()).collect()
}

/// Export the menus and register them with the desktop's registrar, now and whenever a registrar
/// appears. `None` off Wayland, or without a session bus.
pub fn install(ctx: &egui::Context, wayland: bool) -> Option<GlobalMenu> {
    if !wayland {
        return None;
    }
    let shared = Arc::new(Mutex::new(Shared::of(&menus())));
    let object = MenuObject { shared: shared.clone(), ctx: ctx.clone() };
    let conn = match zbus::blocking::connection::Builder::session().and_then(|b| b.serve_at(MENU_PATH, object)).and_then(|b| b.build()) {
        Ok(conn) => conn,
        Err(e) => {
            log::info!("global menu: no session bus ({e})");
            return None;
        }
    };
    register(&conn);
    // The registrar may start after Celdas, or restart with the shell: register whenever it appears.
    let watch = conn.clone();
    let spawned = std::thread::Builder::new().name("global-menu".into()).spawn(move || {
        let Ok(dbus) = zbus::blocking::fdo::DBusProxy::new(&watch) else { return };
        let Ok(changes) = dbus.receive_name_owner_changed() else { return };
        for change in changes {
            let Ok(args) = change.args() else { continue };
            if (args.name().as_str() == REGISTRAR || args.name().as_str() == NODALIX_MENU) && args.new_owner().is_some() {
                register(&watch);
            }
        }
    });
    if let Err(e) = spawned {
        log::warn!("global menu: couldn't watch the registrar ({e})");
    }
    Some(GlobalMenu { _conn: conn, shared })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rows_are_numbered_depth_first_with_celdas_first() {
        let s = Shared::of(&menus());
        let first = s.root.first().copied().unwrap_or_default();
        assert!(matches!(s.rows.get(&first), Some(Row::Submenu { label, .. }) if label == "Celdas"));
        assert_eq!(s.root.len(), menu_tree::tree().len() + 1);
        assert_eq!(s.children(0), s.root.as_slice());
        let p = s.properties(first + 1);
        assert_eq!(p.get("label").and_then(|v| <&str>::try_from(v).ok()), Some("About Celdas"));
        assert_eq!(s.properties(first + 2).get("type").and_then(|v| <&str>::try_from(v).ok()), Some("separator"));
        use zbus::zvariant::Type as _;
        assert_eq!(LayoutItem::SIGNATURE.to_string(), "(ia{sv}av)");
        assert_eq!(s.layout_item(0, -1).children.len(), s.root.len());
        assert!(s.layout_item(0, 0).children.is_empty());
        assert_eq!(mnemonic_free("A_B"), "A__B");
    }

    #[test]
    fn a_click_runs_the_item_and_other_events_nothing() {
        let shared = Arc::new(Mutex::new(Shared::of(&menus())));
        let object = MenuObject { shared: shared.clone(), ctx: egui::Context::default() };
        let first = shared.lock().unwrap().root[0];
        object.chosen(first + 3, "clicked");
        object.chosen(first + 3, "hovered");
        object.chosen(first + 2, "clicked");
        object.chosen(first, "clicked");
        let chosen = std::mem::take(&mut shared.lock().unwrap().chosen);
        assert_eq!(chosen, vec!["dialog:backstage:options"], "only the Preferences click, once");
        let mut app = SheetApp::new(celdas_engine::Session::new(), Default::default());
        menu_tree::dispatch(&mut app, &egui::Context::default(), egui::ViewportId::ROOT, chosen[0]);
        assert!(app.backstage.is_some(), "Preferences opens File › Options");
    }
}
