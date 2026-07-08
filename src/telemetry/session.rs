//! Telemetry session state for the cluster UI.

use sigma_racer_telemetry::{TelemetryClient, VehicleState};

/// UI tick period (~30 Hz).
pub const TICK: std::time::Duration = std::time::Duration::from_millis(33);
/// Retry `connect()` roughly once per second while the service is unavailable.
pub const RECONNECT_TICKS: u32 = 30;
/// Hide live values when no validated telemetry arrived recently.
pub const TELEMETRY_STALE: std::time::Duration = std::time::Duration::from_secs(2);

pub struct Session {
    pub client: Option<TelemetryClient>,
    pub state: VehicleState,
    pub ticks_since_attempt: u32,
    pub last_msg_at: Option<std::time::Instant>,
    pub current_window: i32,
}

impl Session {
    pub fn new(client: Option<TelemetryClient>) -> Self {
        Self {
            client,
            state: VehicleState::idle(),
            ticks_since_attempt: 0,
            last_msg_at: None,
            current_window: 0,
        }
    }
}
