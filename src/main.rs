//! Sigma Racer — production instrument cluster for Wingman / i.MX 8M Plus.
//!
//! Live vehicle data replaces the idle loop via CAN-FD from the M7 safety core.

mod app;
mod telemetry;
mod vehicle;

use sigma_instrumentation::{
    configure_window, init_gauge_art, theme, DisplayConfig, SigmaDashboard,
};
use slint::ComponentHandle;

fn main() -> Result<(), slint::PlatformError> {
    let ui = SigmaDashboard::new()?;

    app::schedule_hide(&ui);
    theme::init_from_env(&ui);
    configure_window(
        &ui,
        DisplayConfig::embedded(cfg!(sigma_racer_wingman_embedded)),
    );
    init_gauge_art(&ui);

    ui.set_current_window(0);
    app::push_idle(&ui);
    telemetry::attach(&ui);
    app::start_clock(&ui);

    ui.run()
}
