//! Push [`VehicleState`] onto Slint dashboard properties.

use sigma_instrumentation::{heading, set_needle_paths, set_speed_readout, SigmaDashboard};
use sigma_racer_telemetry::VehicleState;
use slint::SharedString;

pub fn apply_state(ui: &SigmaDashboard, state: &VehicleState) {
    let rpm = state.rpm;
    ui.set_rpm(rpm);
    set_speed_readout(ui, state.speed.round() as i32);
    ui.set_gear(state.gear as i32);
    ui.set_at_redline(state.at_redline);
    ui.set_side_stand(state.side_stand);
    ui.set_riding_mode(SharedString::from(state.riding_mode.as_str()));
    ui.set_fuel_pct(state.fuel_pct / 100.0);
    ui.set_coolant_c(state.coolant_c as i32);
    ui.set_oil_c(state.oil_c as i32);
    ui.set_odometer(state.odometer.round() as i32);
    ui.set_trip1(state.trip1);
    ui.set_trip2(state.trip2);
    ui.set_lean_angle(state.lean_angle);
    ui.set_gforce(state.gforce);
    ui.set_battery_v(state.battery_v);
    ui.set_can_load(state.can_load as i32);
    ui.set_dtc(state.dtc as i32);
    ui.set_abs_active(state.abs_active);
    ui.set_tc_active(state.tc_active);
    ui.set_heading(state.heading);
    ui.set_heading_label(SharedString::from(heading::heading_label(state.heading)));
    ui.set_elevation(state.elevation);

    set_needle_paths(ui, rpm);
}
