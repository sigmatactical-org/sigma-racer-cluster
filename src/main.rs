//! Sigma Racer — production instrument cluster for Wingman / i.MX 8M Plus.
//!
//! Live vehicle data replaces the idle loop via CAN-FD from the M7 safety core.

mod app;
mod telemetry;
mod vehicle;

use sigma_instrumentation::{
    configure_window, init_gauge_art, start_signal_blink, theme, DisplayConfig, SigmaDashboard,
};
use slint::ComponentHandle;

use crate::vehicle::XSR900_GP;

fn main() -> Result<(), slint::PlatformError> {
    let ui = SigmaDashboard::new()?;
    let gauge = XSR900_GP.gauge_scale();

    theme::init_from_env(&ui);
    configure_window(
        &ui,
        DisplayConfig::embedded(cfg!(sigma_racer_wingman_embedded)),
    );
    init_gauge_art(&ui, &gauge);

    ui.set_current_window(0);
    app::push_idle(&ui);
    telemetry::attach(&ui);
    app::start_clock(&ui);
    let _signal_blink = start_signal_blink(&ui);

    ui.run()
}
