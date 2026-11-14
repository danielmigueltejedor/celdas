use egui::{Align, Layout, RichText, TextureHandle, vec2};

use super::Dialog;
use crate::theme::{self, Tokens};

/// Celdas's icon for the About dialog, decoded once.
fn icon(ctx: &egui::Context) -> Option<TextureHandle> {
    let id = egui::Id::unique("celdas.about.icon");
    if let Some(t) = ctx.data(|d| d.get_temp::<TextureHandle>(id)) {
        return Some(t);
    }
    let bytes = include_bytes!("../../../../assets/app-icon/hicolor/128x128/apps/dev.danielmiguelt.Celdas.png");
    let rgba = image::load_from_memory(bytes).ok()?.to_rgba8();
    let size = [rgba.width() as usize, rgba.height() as usize];
    let image = egui::ColorImage::from_rgba_unmultiplied(size, rgba.as_raw());
    let texture = ctx.load_texture("celdas-about-icon", image, egui::TextureOptions::LINEAR);
    ctx.data_mut(|d| d.insert_temp(id, texture.clone()));
    Some(texture)
}

/// Help ▸ About Celdas, as every Nodalia app shows it: the icon, the name and version, the suite,
/// and links to the website and the licence. Work by other authors is credited in NOTICE.
pub(super) fn about(ui: &mut egui::Ui, _d: &mut Dialog) {
    let t = Tokens::get(ui.ctx());
    let lang = crate::i18n::current(ui.ctx());
    ui.with_layout(Layout::top_down(Align::Center), |ui| {
        ui.add_space(8.0);
        if let Some(tex) = icon(ui.ctx()) {
            ui.add(egui::Image::new(&tex).fit_to_exact_size(vec2(96.0, 96.0)));
        }
        ui.add_space(10.0);
        ui.label(RichText::new("Celdas").font(theme::ui_bold(22.0)).color(t.text));
        ui.label(RichText::new(format!("{} {}", lang.tr("Version"), env!("CARGO_PKG_VERSION"))).font(theme::ui_font(13.0)).color(t.text_dim));
        ui.add_space(6.0);
        ui.label(RichText::new(lang.tr("The spreadsheet of Nodalia's office apps.")).font(theme::ui_font(13.5)).color(t.text));
        ui.label(RichText::new(lang.tr("Part of Nodalia")).font(theme::ui_font(12.5)).color(t.text_dim));
        ui.add_space(10.0);
        ui.horizontal(|ui| {
            // Centre the two links as one row.
            let w = 230.0;
            ui.add_space(((ui.available_width() - w) / 2.0).max(0.0));
            ui.hyperlink_to(lang.tr("Website"), "https://getnodalia.com/celdas");
            ui.label(RichText::new("·").color(t.text_dim));
            ui.hyperlink_to(lang.tr("Licence: MIT or Apache-2.0"), "https://github.com/getnodalia/celdas/blob/main/LICENSE-MIT");
        });
        ui.add_space(10.0);
        ui.label(
            RichText::new(lang.tr("Includes work by the ArtCraft Team and other open-source authors (see NOTICE)."))
                .font(theme::ui_font(11.5))
                .color(t.text_dim),
        );
        ui.add_space(6.0);
    });
}
