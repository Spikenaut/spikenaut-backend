//! Lightweight GPU telemetry struct used by the backend trait.

use serde::{Deserialize, Serialize};

/// Real-time GPU sensor readings passed to `TraderBackend::process_signals`.
///
/// Fields mirror the NVML/sysfs sensors of an RTX 5080 production deployment.
/// All fields default to zero (sensor absent / software-only mode).
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct GpuTelemetry {
    /// GPU core voltage (V). Expect ~0.7 V idle, ~1.05 V load.
    pub vddcr_gfx_v: f32,
    /// GPU junction temperature (°C).
    pub gpu_temp_c: f32,
    /// Dynex/mining hashrate (MH/s). 0 = not mining.
    pub hashrate_mh: f32,
    /// Board power draw (W).
    pub power_w: f32,
    /// GPU core clock (MHz). 0 = GPU absent.
    pub gpu_clock_mhz: f32,
    /// GDDR memory clock (MHz).
    pub mem_clock_mhz: f32,
    /// Fan speed (%).
    pub fan_speed_pct: f32,
    /// Ocean Predictoor signal normalised to [0.0, 1.0]. 0 = no data.
    #[serde(default)]
    pub ocean_intel: f32,
}
