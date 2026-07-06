//! Yamaha XSR900 GP vehicle profile — Sigma Racer product constants.

/// Tuned triple calibration for the Sigma Racer demo / product line.
///
/// These are published product-spec constants. Only `idle_rpm` drives the boot
/// placeholder today; `rev_limit_rpm`, `max_speed_kmh`, and `redline_rpm` are
/// part of the vehicle profile surface and are retained for gauge/limit wiring.
#[allow(dead_code)]
pub struct VehicleProfile {
    pub idle_rpm: f32,
    pub rev_limit_rpm: f32,
    pub max_speed_kmh: f32,
    pub redline_rpm: f32,
}

pub const XSR900_GP: VehicleProfile = VehicleProfile {
    idle_rpm: 1_200.0,
    rev_limit_rpm: 11_500.0,
    max_speed_kmh: 235.0,
    redline_rpm: 11_250.0,
};
