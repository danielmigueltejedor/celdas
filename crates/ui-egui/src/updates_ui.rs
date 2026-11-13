//! One background check at startup (and on Help ▸ Check for Updates…) and the modal windows that
//! offer the new release and install it. The flow and the windows are `nodalia-update`'s, shared
//! with the other Nodalia apps; this file is what is this app's own.

use nodalia_update::egui::{Action, Updater};

use crate::SheetApp;

pub(crate) struct Runtime {
    updater: Updater,
    /// The check at start was asked for; `tick` starts it with the context.
    startup: bool,
    /// Help ▸ Check for Updates… was chosen; the next `tick` starts it.
    manual: bool,
}

impl Default for Runtime {
    fn default() -> Self {
        Self { updater: Updater::new(crate::UPDATE_APP).with_accent([0x27, 0x86, 0x4b]), startup: false, manual: false }
    }
}

/// Ask for a check; the next frame's `tick` starts it. A manual one reports every outcome.
pub(crate) fn start(app: &mut SheetApp, manual: bool) {
    if manual {
        app.updates_rt.manual = true;
    } else {
        app.updates_rt.startup = true;
    }
}

pub(crate) fn tick(app: &mut SheetApp, ctx: &egui::Context) {
    let rt = &mut app.updates_rt;
    if std::mem::take(&mut rt.manual) {
        rt.updater.check_now(ctx);
    } else if rt.startup {
        rt.updater.check_at_start(ctx);
    }
    let action = rt.updater.poll();
    act(app, ctx, action);
}

/// Carries out what the update windows asked for.
fn act(app: &mut SheetApp, ctx: &egui::Context, action: Action) {
    match action {
        Action::OpenUrl(url) => {
            if let Some(open) = &app.services.open_url {
                open(&url);
            }
        }
        Action::Exit => {
            // The new version is starting, or a helper waits for this process to end. Quit closes
            // every workbook window and saves the preferences; the install only ran once nothing
            // was unsaved. Exit anyway if the app has not ended soon.
            app.request_quit(ctx);
            std::thread::spawn(|| {
                std::thread::sleep(std::time::Duration::from_secs(5));
                std::process::exit(0);
            });
        }
        Action::None => {}
    }
}

pub(crate) fn show(app: &mut SheetApp, ctx: &egui::Context) {
    // An installation waits until no workbook holds unsaved work.
    let clean = !app.session.documents().iter().any(|d| d.is_dirty());
    let language = nodalia_update::Language::from_code(crate::i18n::current(ctx).code());
    let action = app.updates_rt.updater.show(ctx, language, clean);
    act(app, ctx, action);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_releases_are_this_apps_own() {
        assert_eq!(crate::UPDATE_APP.repo, "getnodalia/celdas");
        assert_eq!(crate::UPDATE_APP.version, env!("CARGO_PKG_VERSION"));
    }

    #[test]
    fn a_check_at_start_is_silent_until_it_finds_something() {
        let mut rt = Runtime::default();
        assert!(!rt.updater.is_open());
        rt.updater.set_status(nodalia_update::egui::Status::Checking { user: false });
        assert!(!rt.updater.is_open());
        rt.updater.set_status(nodalia_update::egui::Status::Checking { user: true });
        assert!(rt.updater.is_open());
    }
}
