//! Nodalia's window controls on Linux and the BSDs, where the window has no OS decorations
//! ([`SheetApp::ndl_window_controls`]): the title bar is the window's own. Its free stretch drags
//! the window and a double-click there maximizes it (`ribbon::title_bar`); nodalia-design's round
//! buttons minimize, maximize or restore, and close; thin invisible zones along the window's edges
//! resize it. macOS and Windows keep their native controls, and File › Options › General › Window
//! controls brings the desktop's own back on Linux.

use egui::{CursorIcon, Id, LayerId, Order, PointerButton, Rect, ResizeDirection, Sense, Ui, ViewportCommand, pos2, vec2};
use nodalia_theme::window_controls::{self as ndl, Action, Labels};

use crate::SheetApp;
use crate::theme::{self, Tokens};

/// Resize zone thickness along an edge, and the side of a corner zone.
const EDGE: f32 = 6.0;
const CORNER: f32 = 12.0;
/// The gap between the last button and the window's edge.
const END: f32 = 8.0;

/// Which window controls Celdas draws on Linux and the BSDs (File › Options › General).
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum WindowControls {
    /// Nodalia's round buttons in Celdas's own title bar.
    #[default]
    Nodalia,
    /// The desktop's title bar and buttons.
    System,
}

impl WindowControls {
    pub const ALL: [WindowControls; 2] = [WindowControls::Nodalia, WindowControls::System];

    pub fn code(self) -> &'static str {
        match self {
            WindowControls::Nodalia => "nodalia",
            WindowControls::System => "system",
        }
    }

    pub fn from_code(code: &str) -> Option<Self> {
        Self::ALL.into_iter().find(|c| c.code().eq_ignore_ascii_case(code.trim()))
    }

    /// The English label (translated where shown).
    pub fn label(self) -> &'static str {
        match self {
            WindowControls::Nodalia => "Nodalia",
            WindowControls::System => "Use the desktop's",
        }
    }
}

/// The width the buttons take at the title bar's right end: zero without Nodalia's controls.
pub fn reserved(app: &SheetApp) -> f32 {
    if app.ndl_window_controls { ndl::WIDTH + END } else { 0.0 }
}

fn maximized(ctx: &egui::Context) -> bool {
    ctx.input(|i| i.viewport().maximized.unwrap_or(false))
}

/// Maximize the window, or restore it when it already is.
pub fn toggle_maximize(ctx: &egui::Context) {
    ctx.send_viewport_cmd(ViewportCommand::Maximized(!maximized(ctx)));
}

/// The buttons at the right end of `bar` (the title bar's rect). Close asks the host to close the
/// window, which asks about unsaved work first.
pub fn caption_buttons(app: &SheetApp, ui: &Ui, bar: Rect) {
    if !app.ndl_window_controls {
        return;
    }
    let t = Tokens::get(ui.ctx());
    let mode = if t.dark { nodalia_theme::Mode::Dark } else { nodalia_theme::Mode::Light };
    let palette = theme::palette(mode);
    let ctx = ui.ctx().clone();
    let right = bar.right().min(ctx.content_rect().right()) - END;
    let rect = Rect::from_min_max(pos2(right - ndl::WIDTH, bar.top()), pos2(right, bar.bottom()));
    let focused = ctx.input(|i| i.viewport().focused.unwrap_or(true));
    let lang = app.ui.language;
    let labels = Labels { minimize: lang.tr("Minimize"), maximize: lang.tr("Maximize"), restore: lang.tr("Restore"), close: lang.tr("Close") };
    match ndl::show(ui, rect, &palette, mode, focused, maximized(&ctx), labels) {
        Some(Action::Minimize) => ctx.send_viewport_cmd(ViewportCommand::Minimized(true)),
        Some(Action::Maximize | Action::Restore) => toggle_maximize(&ctx),
        Some(Action::Close) => ctx.send_viewport_cmd(ViewportCommand::Close),
        None => {}
    }
}

/// A dialog's close button in the Nodalia look (the window buttons' disc and ×), `size` points
/// across, laid out in `ui`. Says whether it was clicked.
pub fn close_button(ui: &mut Ui, size: f32, label: &str) -> bool {
    let t = Tokens::get(ui.ctx());
    let mode = if t.dark { nodalia_theme::Mode::Dark } else { nodalia_theme::Mode::Light };
    let p = theme::palette(mode);
    let (rect, resp) = ui.allocate_exact_size(vec2(size, size), Sense::click());
    let resp = resp.on_hover_cursor(CursorIcon::PointingHand);
    let (fill, ink) = if resp.is_pointer_button_down_on() {
        (p.control_bg_pressed, p.text_primary)
    } else if resp.hovered() {
        (nodalia_theme::window_color::CLOSE, nodalia_theme::window_color::CLOSE_ICON)
    } else {
        (p.control_bg, p.text_secondary)
    };
    ui.painter().circle_filled(rect.center(), size / 2.0, theme::ndl(fill));
    if let Some(icon) = nodalia_theme::icons::get("window-close") {
        let glyph = Rect::from_center_size(rect.center(), vec2(size * 0.86, size * 0.86));
        nodalia_theme::icons::paint(ui.painter(), glyph, icon, nodalia_theme::icons::Style::bold(theme::ndl(ink)));
    }
    resp.widget_info(|| egui::WidgetInfo::labeled(egui::Role::Button, true, label));
    resp.clicked()
}

/// The edge and corner zones of a window with content rect `w`: (zone, direction, cursor).
/// Corners come last so they win where they overlap the edges.
fn zones(w: Rect) -> [(Rect, ResizeDirection, CursorIcon); 8] {
    use ResizeDirection::*;
    let (e, c) = (EDGE, CORNER);
    [
        (Rect::from_min_max(w.left_top(), pos2(w.right(), w.top() + e)), North, CursorIcon::ResizeNorth),
        (Rect::from_min_max(pos2(w.left(), w.bottom() - e), w.right_bottom()), South, CursorIcon::ResizeSouth),
        (Rect::from_min_max(w.left_top(), pos2(w.left() + e, w.bottom())), West, CursorIcon::ResizeWest),
        (Rect::from_min_max(pos2(w.right() - e, w.top()), w.right_bottom()), East, CursorIcon::ResizeEast),
        (Rect::from_min_size(w.left_top(), vec2(c, c)), NorthWest, CursorIcon::ResizeNwSe),
        (Rect::from_min_size(w.right_top() - vec2(c, 0.0), vec2(c, c)), NorthEast, CursorIcon::ResizeNeSw),
        (Rect::from_min_size(w.left_bottom() - vec2(0.0, c), vec2(c, c)), SouthWest, CursorIcon::ResizeNeSw),
        (Rect::from_min_size(w.right_bottom() - vec2(c, c), vec2(c, c)), SouthEast, CursorIcon::ResizeNwSe),
    ]
}

fn handed_off_id() -> Id {
    Id::unique("celdas.window_controls.handed_off")
}

/// An OS resize or move started at `pass`; `released` is the pass whose input ended the press.
#[derive(Clone, Copy, Debug)]
struct HandOff {
    pass: u64,
    released: Option<u64>,
}

fn hand_off(ctx: &egui::Context) {
    let pass = ctx.cumulative_pass_nr();
    ctx.data_mut(|d| d.insert_temp(handed_off_id(), HandOff { pass, released: None }));
}

/// Move the window from this press. The window manager keeps the release of an OS move, so the
/// press is ended by [`release_after_os_move`].
pub fn start_os_move(ctx: &egui::Context) {
    ctx.send_viewport_cmd(ViewportCommand::StartDrag);
    hand_off(ctx);
}

/// The compositor takes the button release of an OS resize or move (KWin, Mutter), so egui would
/// keep the press down. End it on the input after the hand-off, only while egui still has it down
/// and the input brings no release of its own. Called from `raw_input_hook`.
pub fn release_after_os_move(ctx: &egui::Context, raw: &mut egui::RawInput) {
    let Some(h) = ctx.data(|d| d.get_temp::<HandOff>(handed_off_id())) else { return };
    if h.released.is_some() || h.pass >= ctx.cumulative_pass_nr() {
        return;
    }
    let os_released = raw.events.iter().any(|e| matches!(e, egui::Event::PointerButton { button: PointerButton::Primary, pressed: false, .. }));
    if let Some(pos) = ctx.input(|i| i.pointer.latest_pos()).filter(|_| !os_released && ctx.input(|i| i.pointer.primary_down())) {
        raw.events.push(egui::Event::PointerButton { pos, button: PointerButton::Primary, pressed: false, modifiers: ctx.input(|i| i.modifiers) });
    }
    let pass = ctx.cumulative_pass_nr();
    ctx.data_mut(|d| d.insert_temp(handed_off_id(), HandOff { released: Some(pass), ..h }));
}

/// The compositor sets its own cursor during an OS resize, and egui-winit only re-sends a cursor
/// that changed: on the first motion after the hand-off, show the default one for a frame.
fn reshow_cursor(ctx: &egui::Context) {
    let Some(HandOff { released: Some(pass), .. }) = ctx.data(|d| d.get_temp::<HandOff>(handed_off_id())) else { return };
    if pass < ctx.cumulative_pass_nr() && ctx.input(|i| i.events.iter().any(|e| matches!(e, egui::Event::PointerMoved(_)))) {
        ctx.data_mut(|d| d.remove::<HandOff>(handed_off_id()));
        ctx.set_cursor_icon(CursorIcon::Default);
        ctx.request_repaint();
    }
}

/// Invisible resize zones along the window's edges, above everything else; a press in one starts
/// an OS resize. None while the window is maximized or full screen, or has OS decorations.
pub fn resize_zones(app: &SheetApp, ui: &mut Ui) {
    let ctx = ui.ctx().clone();
    if !app.ndl_window_controls || maximized(&ctx) || ctx.input(|i| i.viewport().fullscreen.unwrap_or(false)) {
        reshow_cursor(&ctx);
        return;
    }
    let w = ctx.content_rect();
    let layer = LayerId::new(Order::Foreground, Id::unique("celdas.window_resize"));
    let zui = ui.new_child(egui::UiBuilder::new().layer_id(layer).max_rect(w));
    for (i, (r, dir, cursor)) in zones(w).into_iter().enumerate() {
        let resp = zui.interact(r, Id::unique(("celdas.window_resize", i)), Sense::drag()).on_hover_cursor(cursor);
        if resp.drag_started_by(PointerButton::Primary) {
            ctx.send_viewport_cmd(ViewportCommand::BeginResize(dir));
            hand_off(&ctx);
        }
    }
    reshow_cursor(&ctx);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn codes_round_trip_and_nodalia_is_the_default() {
        for c in WindowControls::ALL {
            assert_eq!(WindowControls::from_code(c.code()), Some(c));
        }
        assert_eq!(WindowControls::default(), WindowControls::Nodalia);
        assert_eq!(WindowControls::from_code("Gnome"), None);
    }

    #[test]
    fn corner_zones_win_over_edges() {
        let w = Rect::from_min_size(pos2(0.0, 0.0), vec2(800.0, 600.0));
        let z = zones(w);
        let at = pos2(2.0, 2.0);
        let last = z.iter().rev().find(|(r, _, _)| r.contains(at)).map(|(_, d, _)| *d);
        assert_eq!(last, Some(ResizeDirection::NorthWest));
    }
}
