//! Fallback values until the first Snapshot arrives from sigma-racer-vehicle.

use sigma_instrumentation::{gauge, set_speed_readout, SigmaDashboard};
use slint::SharedString;

use crate::vehicle::XSR900_GP;

pub fn push(ui: &SigmaDashboard) {
    let profile = XSR900_GP;

    ui.set_rpm(profile.idle_rpm);
    set_speed_readout(ui, 0);
    ui.set_gear(0);
    ui.set_at_redline(false);
    ui.set_side_stand(true);
    ui.set_swept_path(gauge::swept_path(profile.idle_rpm));
    let (nl, ns, nr, no) = gauge::needle_paths(profile.idle_rpm);
    ui.set_needle_left(nl);
    ui.set_needle_spine(ns);
    ui.set_needle_right(nr);
    ui.set_needle_outline(no);
    ui.set_fuel_pct(0.62);
    ui.set_lean_angle(0.0);
    ui.set_gforce(0.0);
    ui.set_coolant_c(42);
    ui.set_oil_c(52);
    ui.set_battery_v(13.1);
    ui.set_can_load(8);
    ui.set_dtc(0);
    ui.set_abs_active(false);
    ui.set_tc_active(false);
    ui.set_telemetry_live(false);
    ui.set_heading(0.0);
    ui.set_heading_label(SharedString::from("N"));
    ui.set_elevation(667);
    ui.set_nav_blocked_hint(false);
}
