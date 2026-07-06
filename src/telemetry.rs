//! vehicle.service telemetry → sigma-dash UI bindings.

use sigma_racer_wingman_telemetry::{TelemetryClient, VehicleState};
use sigma_instrumentation::{gauge, set_speed_readout, SigmaDashboard};
use slint::ComponentHandle;
use slint::SharedString;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

/// UI tick period (~30 Hz).
const TICK: Duration = Duration::from_millis(33);
/// Retry `connect()` roughly once per second while the service is unavailable.
const RECONNECT_TICKS: u32 = 30;

struct Session {
    /// `None` until the first successful connection; the client itself
    /// auto-reconnects once established, so this only re-arms the *initial*
    /// connection when the dash starts before vehicle.service.
    client: Option<TelemetryClient>,
    state: VehicleState,
    ticks_since_attempt: u32,
}

/// Subscribe to vehicle.service and drive Slint properties from VSS snapshots.
pub fn attach(ui: &SigmaDashboard) {
    let initial = TelemetryClient::connect();
    if initial.is_some() {
        eprintln!("sigma-dash: subscribed to vehicle telemetry");
    } else {
        eprintln!("sigma-dash: vehicle telemetry unavailable — retrying in background");
    }

    let session = Rc::new(RefCell::new(Session {
        client: initial,
        state: VehicleState::idle(),
        ticks_since_attempt: 0,
    }));

    let ui_weak = ui.as_weak();
    slint::Timer::default().start(slint::TimerMode::Repeated, TICK, move || {
        let mut session = session.borrow_mut();
        let Some(ui) = ui_weak.upgrade() else {
            return;
        };

        if session.client.is_none() {
            session.ticks_since_attempt += 1;
            if session.ticks_since_attempt >= RECONNECT_TICKS {
                session.ticks_since_attempt = 0;
                session.client = TelemetryClient::connect();
                if session.client.is_some() {
                    eprintln!("sigma-dash: subscribed to vehicle telemetry");
                }
            }
            return;
        }

        let messages: Vec<_> = session
            .client
            .as_ref()
            .map(|client| client.drain().collect())
            .unwrap_or_default();
        for msg in messages {
            if let Some(data) = msg.vss_data() {
                session.state.apply_vss_map(data);
                apply_state(&ui, &session.state);
            }
        }
    });
}

fn apply_state(ui: &SigmaDashboard, state: &VehicleState) {
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

    ui.set_swept_path(gauge::swept_path(rpm));
    let (nl, ns, nr, no) = gauge::needle_paths(rpm);
    ui.set_needle_left(nl);
    ui.set_needle_spine(ns);
    ui.set_needle_right(nr);
    ui.set_needle_outline(no);
}
