//! Boot splash visibility timer.

use sigma_instrumentation::SigmaDashboard;
use slint::ComponentHandle;
use std::time::Duration;

const BOOT_SPLASH_MS: u64 = 1200;

pub fn schedule_hide(ui: &SigmaDashboard) {
    ui.set_boot_visible(true);
    let ui_weak = ui.as_weak();
    slint::Timer::single_shot(Duration::from_millis(BOOT_SPLASH_MS), move || {
        if let Some(ui) = ui_weak.upgrade() {
            ui.set_boot_visible(false);
        }
    });
}
