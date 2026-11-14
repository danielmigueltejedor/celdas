//! The desktop menu bar's menus (File, Edit, View, Insert, Format, Tools, Data, Window, Help),
//! shared by the macOS menu bar (`native_menu.rs`) and the Linux global menu (`linux_menu.rs`).
//! Items run the same commands as the ribbon.

use celdas_ui_egui::SheetApp;
use serde_json::{Value, json};

pub type Entry = (&'static str, &'static str, Option<&'static str>);

/// (label, command or `dialog:name`, accelerator). `-` is a separator.
pub fn tree() -> Vec<(&'static str, Vec<Entry>)> {
    vec![
        (
            "File",
            vec![
                ("New Workbook", "file.new", Some("CMD+N")),
                ("New from Sample…", "dialog:start", None),
                ("Open…", "file.open", Some("CMD+O")),
                ("-", "", None),
                ("Close", "file.close", Some("CMD+W")),
                ("Save", "file.save", Some("CMD+S")),
                ("Save As…", "file.saveAs", Some("CMD+SHIFT+S")),
                ("Export as PDF…", "file.exportPdf", None),
                ("Export as CSV…", "dialog:saveCopy:csv", None),
                ("Save as Web Page…", "dialog:saveCopy:html", None),
                ("-", "", None),
                ("Page Setup…", "dialog:pageSetup", None),
                ("Print…", "file.print", Some("CMD+P")),
                ("-", "", None),
                ("Properties…", "dialog:backstage:info", None),
            ],
        ),
        (
            "Edit",
            vec![
                ("Undo", "edit.undo", None),
                ("Redo", "edit.redo", None),
                ("-", "", None),
                ("Cut", "edit.cut", None),
                ("Copy", "edit.copy", None),
                ("Paste", "edit.paste", None),
                ("Paste Special…", "dialog:pasteSpecial", Some("CTRL+CMD+V")),
                ("-", "", None),
                ("Fill Down", "edit.fillDown", None),
                ("Fill Right", "edit.fillRight", None),
                ("Series…", "dialog:series", None),
                ("Flash Fill", "edit.flashFill", None),
                ("-", "", None),
                ("Clear All", "edit.clearAll", None),
                ("Clear Formats", "edit.clearFormats", None),
                ("Clear Contents", "edit.clearContents", None),
                ("-", "", None),
                ("Delete…", "dialog:deleteCells", None),
                ("Delete Sheet", "home.deleteSheet", None),
                ("-", "", None),
                ("Find…", "dialog:find", None),
                ("Replace…", "dialog:find:replace", Some("CTRL+H")),
                ("Go To…", "dialog:goTo", Some("CTRL+G")),
            ],
        ),
        (
            "View",
            vec![
                ("Normal", "view.normal", None),
                ("Page Layout", "view.pageLayout", None),
                ("Page Break Preview", "view.pageBreakPreview", None),
                ("-", "", None),
                ("Formula Bar", "view.formulaBar", None),
                ("Gridlines", "view.gridlines", None),
                ("Headings", "view.headings", None),
                ("-", "", None),
                ("Zoom…", "dialog:zoom", None),
                ("Zoom to Selection", "view.zoomToSelection", None),
                ("Freeze Panes", "view.freezePanes", None),
                ("Freeze Top Row", "view.freezeTopRow", None),
                ("Unfreeze Panes", "view.unfreezePanes", None),
                ("-", "", None),
                ("Collapse Ribbon", "view.collapseRibbon", Some("CMD+ALT+R")),
            ],
        ),
        (
            "Insert",
            vec![
                ("Cells…", "dialog:insertCells", None),
                ("Rows", "home.insertRows", None),
                ("Columns", "home.insertColumns", None),
                ("Sheet", "home.insertSheet", Some("SHIFT+F11")),
                ("-", "", None),
                ("Chart", "insert.recommendedCharts", None),
                ("Sparklines…", "dialog:sparkline", None),
                ("PivotTable", "insert.pivotTable", None),
                ("Table", "insert.table", None),
                ("-", "", None),
                ("Function…", "dialog:insertFunction", Some("SHIFT+F3")),
                ("Name…", "dialog:defineName", None),
                ("New Comment", "dialog:comment", None),
                ("New Note", "dialog:note", None),
                ("Picture…", "dialog:insertPicture", None),
                ("Text Box", "insert.textBox", None),
                ("Link…", "dialog:insertLink", None),
                ("Checkbox", "insert.checkbox", None),
            ],
        ),
        (
            "Format",
            vec![
                ("Cells…", "dialog:formatCells", None),
                ("Row Height…", "dialog:rowHeight", None),
                ("AutoFit Row Height", "home.autofitRowHeight", None),
                ("Column Width…", "dialog:columnWidth", None),
                ("AutoFit Column Width", "home.autofitColumnWidth", None),
                ("-", "", None),
                ("Hide Rows", "home.hideRows", None),
                ("Unhide Rows", "home.unhideRows", None),
                ("Hide Columns", "home.hideColumns", None),
                ("Unhide Columns", "home.unhideColumns", None),
                ("-", "", None),
                ("Rename Sheet…", "dialog:renameSheet", None),
                ("Hide Sheet", "sheet.hide", None),
                ("Unhide Sheet…", "sheet.unhide", None),
                ("-", "", None),
                ("Conditional Formatting…", "dialog:manageRules", None),
                ("Format as Table", "home.formatAsTable", None),
            ],
        ),
        (
            "Tools",
            vec![
                ("Spelling…", "dialog:spelling", None),
                ("Workbook Statistics", "dialog:statistics", None),
                ("Check Accessibility", "dialog:accessibility", None),
                ("-", "", None),
                ("Protect Sheet…", "dialog:protectSheet", None),
                ("Protect Workbook", "review.protectWorkbook", None),
                ("-", "", None),
                ("Goal Seek…", "dialog:goalSeek", None),
                ("Error Checking…", "dialog:errorChecking", None),
                ("Evaluate Formula…", "dialog:evaluateFormula", None),
                ("-", "", None),
                ("Record Actions", "automate.recordActions", None),
                ("Command Palette…", "dialog:commandSearch", Some("CMD+SHIFT+P")),
            ],
        ),
        (
            "Data",
            vec![
                ("Sort…", "dialog:sort", None),
                ("Sort A to Z", "data.sortAscending", None),
                ("Sort Z to A", "data.sortDescending", None),
                ("Filter", "data.filter", None),
                ("Clear Filter", "data.clearFilter", None),
                ("-", "", None),
                ("Text to Columns…", "dialog:textToColumns", None),
                ("Remove Duplicates…", "dialog:removeDuplicates", None),
                ("Data Validation…", "dialog:dataValidation", None),
                ("-", "", None),
                ("Group", "data.group", None),
                ("Ungroup", "data.ungroup", None),
                ("Show Detail", "data.showDetail", None),
                ("Hide Detail", "data.hideDetail", None),
                ("Subtotal…", "dialog:subtotal", None),
                ("-", "", None),
                ("Calculate Now", "formulas.calculateNow", None),
            ],
        ),
        // The open workbooks follow the separator (`NativeMenu::sync_windows`).
        (
            "Window",
            vec![
                ("New Window", "view.newWindow", None),
                ("Next Sheet", "sheet.next", None),
                ("Previous Sheet", "sheet.previous", None),
                ("-", "", None),
            ],
        ),
        (
            "Help",
            vec![
                ("Keyboard Shortcuts", "dialog:keyboardShortcuts", None),
                ("Check for Updates…", "app.checkForUpdates", None),
                ("Agent Control (MCP)…", "dialog:agents", None),
                ("Celdas on getnodalia.com", "url:https://getnodalia.com/celdas", None),
            ],
        ),
    ]
}

/// The Linux global menu's first menu, which macOS's own app menu stands for there.
pub fn app_menu() -> (&'static str, Vec<Entry>) {
    (
        "Celdas",
        vec![
            ("About Celdas", "dialog:about", None),
            ("-", "", None),
            ("Preferences…", "dialog:backstage:options", None),
            ("-", "", None),
            ("Quit", QUIT, None),
        ],
    )
}

/// The Quit item: asks about each unsaved workbook in its window first
/// (`SheetApp::request_quit`), which AppKit's own Quit (terminate:) would skip.
pub const QUIT: &str = "app.quit";

/// Runs the menu item `id` (a command, `dialog:name[:arg]` or `url:…`). Commands go to the active
/// window; `viewport` is its egui viewport, for the items that act on its input.
pub fn dispatch(app: &mut SheetApp, ctx: &egui::Context, viewport: egui::ViewportId, id: &str) {
    if id == QUIT {
        app.request_quit(ctx);
        return;
    }
    // Text selection belongs to the inline editor, not the worksheet clipboard/undo.
    if app.text_box_editor.is_some() {
        let clipboard = match id {
            "edit.copy" => Some(egui::ViewportCommand::RequestCopy),
            "edit.cut" => Some(egui::ViewportCommand::RequestCut),
            "edit.paste" => Some(egui::ViewportCommand::RequestPaste),
            _ => None,
        };
        if let Some(command) = clipboard {
            ctx.send_viewport_cmd_to(viewport, command);
            return;
        }
        if id == "edit.undo" || id == "edit.redo" {
            ctx.input_mut_for(viewport, |i| {
                i.events.push(egui::Event::Key {
                    key: egui::Key::Z,
                    physical_key: None,
                    pressed: true,
                    repeat: false,
                    modifiers: egui::Modifiers {
                        command: true,
                        mac_cmd: cfg!(target_os = "macos"),
                        ctrl: !cfg!(target_os = "macos"),
                        shift: id == "edit.redo",
                        ..Default::default()
                    },
                });
            });
            return;
        }
    }
    if let Some(url) = id.strip_prefix("url:") {
        if let Some(open) = &app.services.open_url {
            open(url);
        }
    } else if let Some(rest) = id.strip_prefix("dialog:") {
        let (name, arg) = rest.split_once(':').unwrap_or((rest, ""));
        let params: Value = match (name, arg) {
            ("saveCopy", fmt) => json!({"format": fmt}),
            ("find", "replace") => json!({"replace": true}),
            ("note", _) => json!({"threaded": false}),
            ("comment", _) => json!({"threaded": true}),
            ("about", tab) => json!({"tab": tab}),
            ("backstage", page) => json!({"page": page}),
            _ => json!({}),
        };
        let name = if name == "note" { "comment" } else { name };
        app.open_dialog(name, params);
    } else if id == "edit.copy" || id == "edit.cut" {
        app.copy_to_clipboard(ctx, id);
    } else if id == "edit.paste" {
        app.paste_from_clipboard();
    } else {
        app.run_or_alert(id, json!({}));
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn file_menu_has_export_as_pdf_dispatching_the_command() {
        let tree = super::tree();
        let (_, file) = tree.iter().find(|(title, _)| *title == "File").expect("File menu");
        let item = file.iter().find(|e| e.0 == "Export as PDF…").expect("Export as PDF… in the File menu");
        assert_eq!(item.1, "file.exportPdf");
        assert!(celdas_engine::find_command("file.exportPdf").is_some());
    }

    #[test]
    fn help_menu_checks_for_updates_through_the_registered_command() {
        let tree = super::tree();
        let (_, help) = tree.iter().find(|(title, _)| *title == "Help").expect("Help menu");
        let item = help.iter().find(|e| e.0 == "Check for Updates…").expect("Check for Updates… in the Help menu");
        assert_eq!(item.1, "app.checkForUpdates");
        let mut app = super::SheetApp::new(celdas_engine::Session::new(), Default::default());
        assert!(app.run(item.1, serde_json::json!({})).is_ok());
    }

    #[test]
    fn the_theme_is_not_in_the_menus_and_preferences_open_file_options() {
        // Nodalia Design: the light and dark choice lives only in Preferences.
        let all: Vec<_> = super::tree().into_iter().chain([super::app_menu()]).flat_map(|(_, items)| items).collect();
        assert!(all.iter().all(|e| e.1 != "view.theme"));
        let ctx = egui::Context::default();
        let mut app = super::SheetApp::new(celdas_engine::Session::new(), Default::default());
        super::dispatch(&mut app, &ctx, egui::ViewportId::ROOT, "dialog:backstage:options");
        assert!(app.backstage.is_some(), "Preferences opens File › Options");
    }
}
