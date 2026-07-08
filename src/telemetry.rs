//! vehicle.service telemetry → sigma-racer-cluster UI bindings.

use sigma_instrumentation::{gauge, heading, set_speed_readout, SigmaDashboard};
use sigma_racer_wingman_telemetry::{TelemetryClient, VehicleState};
use slint::ComponentHandle;
use slint::SharedString;
use std::cell::{Cell, RefCell};
use std::rc::Rc;
use std::time::{Duration, Instant};

/// UI tick period (~30 Hz).
const TICK: Duration = Duration::from_millis(33);
/// Retry `connect()` roughly once per second while the service is unavailable.
const RECONNECT_TICKS: u32 = 30;
/// Hide live values when no validated telemetry arrived recently.
const TELEMETRY_STALE: Duration = Duration::from_secs(2);

struct Session {
    client: Option<TelemetryClient>,
    state: VehicleState,
    ticks_since_attempt: u32,
    last_msg_at: Option<Instant>,
    current_window: i32,
}

/// Subscribe to vehicle.service and drive Slint properties from VSS snapshots.
pub fn attach(ui: &SigmaDashboard) {
    let initial = TelemetryClient::connect();
    if initial.is_some() {
        eprintln!("sigma-racer-cluster: subscribed to vehicle telemetry");
    } else {
        eprintln!("sigma-racer-cluster: vehicle telemetry unavailable — retrying in background");
    }

    let session = Rc::new(RefCell::new(Session {
        client: initial,
        state: VehicleState::idle(),
        ticks_since_attempt: 0,
        last_msg_at: None,
        current_window: 0,
    }));

    wire_navigation(ui, &session);

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
                    eprintln!("sigma-racer-cluster: subscribed to vehicle telemetry");
                }
            }
            ui.set_telemetry_live(false);
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
                session.last_msg_at = Some(Instant::now());
            }
        }

        let live = session
            .last_msg_at
            .is_some_and(|t| t.elapsed() < TELEMETRY_STALE);
        ui.set_telemetry_live(live);
        if live {
            apply_state(&ui, &session.state);
        }
    });
}

fn wire_navigation(ui: &SigmaDashboard, session: &Rc<RefCell<Session>>) {
    let stopped = Rc::new(Cell::new(true));

    {
        let session = session.clone();
        let stopped = stopped.clone();
        let ui_weak = ui.as_weak();
        ui.on_nav_next(move || {
            if let Some(ui) = ui_weak.upgrade() {
                nav_step(&ui, session.clone(), stopped.get(), 1);
            }
        });
    }
    {
        let session = session.clone();
        let stopped = stopped.clone();
        let ui_weak = ui.as_weak();
        ui.on_nav_prev(move || {
            if let Some(ui) = ui_weak.upgrade() {
                nav_step(&ui, session.clone(), stopped.get(), -1);
            }
        });
    }
    {
        let session = session.clone();
        let ui_weak = ui.as_weak();
        ui.on_nav_home(move || {
            if let Some(ui) = ui_weak.upgrade() {
                session.borrow_mut().current_window = 0;
                ui.set_current_window(0);
            }
        });
    }
    {
        let session = session.clone();
        let stopped = stopped.clone();
        let ui_weak = ui.as_weak();
        ui.on_nav_select(move |idx| {
            if let Some(ui) = ui_weak.upgrade() {
                select_window(&ui, session.clone(), stopped.get(), idx);
            }
        });
    }

    let session_tick = session.clone();
    let stopped_tick = stopped.clone();
    let ui_weak = ui.as_weak();
    slint::Timer::default().start(slint::TimerMode::Repeated, TICK, move || {
        if let Some(ui) = ui_weak.upgrade() {
            stopped_tick.set(ui.get_speed() == 0);
            ui.set_nav_blocked_hint(!stopped_tick.get() && session_tick.borrow().current_window >= 4);
        }
    });
}

fn nav_step(ui: &SigmaDashboard, session: Rc<RefCell<Session>>, stopped: bool, delta: i32) {
    let mut session = session.borrow_mut();
    let max = if stopped { 8 } else { 3 };
    let next = (session.current_window + delta).clamp(0, max);
    session.current_window = next;
    ui.set_current_window(next);
}

fn select_window(ui: &SigmaDashboard, session: Rc<RefCell<Session>>, stopped: bool, idx: i32) {
    let max = if stopped { 8 } else { 3 };
    let next = idx.clamp(0, max);
    session.borrow_mut().current_window = next;
    ui.set_current_window(next);
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
    ui.set_abs_active(state.abs_active);
    ui.set_tc_active(state.tc_active);
    ui.set_heading(state.heading);
    ui.set_heading_label(SharedString::from(heading::heading_label(state.heading)));
    ui.set_elevation(state.elevation);

    ui.set_swept_path(gauge::swept_path(rpm));
    let (nl, ns, nr, no) = gauge::needle_paths(rpm);
    ui.set_needle_left(nl);
    ui.set_needle_spine(ns);
    ui.set_needle_right(nr);
    ui.set_needle_outline(no);
}
