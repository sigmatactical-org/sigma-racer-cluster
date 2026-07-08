//! Subscribe to sigma-racer-vehicle and drive Slint properties from VSS snapshots.

use sigma_instrumentation::SigmaDashboard;
use sigma_racer_telemetry::TelemetryClient;
use slint::ComponentHandle;
use std::cell::RefCell;
use std::rc::Rc;

use super::binding::apply_state;
use super::navigation;
use super::session::{Session, RECONNECT_TICKS, TELEMETRY_STALE, TICK};

/// Subscribe to sigma-racer-vehicle and drive Slint properties from VSS snapshots.
pub fn attach(ui: &SigmaDashboard) {
    let initial = TelemetryClient::connect();
    if initial.is_some() {
        eprintln!("sigma-racer-cluster: subscribed to vehicle telemetry");
    } else {
        eprintln!("sigma-racer-cluster: vehicle telemetry unavailable — retrying in background");
    }

    let session = Rc::new(RefCell::new(Session::new(initial)));
    navigation::wire(ui, &session);

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
                session.last_msg_at = Some(std::time::Instant::now());
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
