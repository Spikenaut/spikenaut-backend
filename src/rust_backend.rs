//! Pure-Rust native backend — no external dependencies.

use crate::{BackendError, GpuTelemetry, TraderBackend};

/// Rust-native SNN backend.
///
/// Implements a simple push-pull encoding: each of the 8 input channels is
/// split into a bull/bear pair. Channel `i` → `output[i*2]` (positive),
/// channel `i` → `output[i*2+1]` (negative magnitude).
///
/// Useful as a smoke-test stub and software fallback when Julia / ZMQ is
/// unavailable.
pub struct RustBackend {
    initialized: bool,
}

impl RustBackend {
    pub fn new() -> Self {
        Self { initialized: false }
    }
}

impl Default for RustBackend {
    fn default() -> Self {
        Self::new()
    }
}

impl TraderBackend for RustBackend {
    fn process_signals(
        &mut self,
        normalized_inputs: &[f32; 8],
        _inhibition_signal: f32,
        _telemetry: &GpuTelemetry,
    ) -> Result<Vec<f32>, BackendError> {
        if !self.initialized {
            return Err(BackendError::InitializationError(
                "RustBackend not initialized — call initialize() first".to_string(),
            ));
        }

        // Push-pull encoding: positive → bull channel; negative → bear channel.
        let mut output = vec![0.0f32; 16];
        for i in 0..8 {
            let val = normalized_inputs[i];
            if val > 0.0 {
                output[i * 2]     = val;
                output[i * 2 + 1] = 0.0;
            } else {
                output[i * 2]     = 0.0;
                output[i * 2 + 1] = val.abs();
            }
        }
        Ok(output)
    }

    fn initialize(&mut self, _model_path: Option<&str>) -> Result<(), BackendError> {
        self.initialized = true;
        Ok(())
    }

    fn save_state(&self, _model_path: &str) -> Result<(), BackendError> {
        Ok(()) // no state to persist
    }

    fn get_spike_states(&self) -> [bool; 16] {
        [false; 16]
    }

    fn reset(&mut self) -> Result<(), BackendError> {
        // No internal state to reset.
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_telem() -> GpuTelemetry { GpuTelemetry::default() }

    #[test]
    fn process_before_init_returns_error() {
        let mut b = RustBackend::new();
        assert!(b.process_signals(&[0.0; 8], 0.0, &make_telem()).is_err());
    }

    #[test]
    fn positive_input_goes_to_bull_channel() {
        let mut b = RustBackend::new();
        b.initialize(None).unwrap();
        let mut inputs = [0.0f32; 8];
        inputs[0] = 0.8;
        let out = b.process_signals(&inputs, 0.0, &make_telem()).unwrap();
        assert!((out[0] - 0.8).abs() < 1e-5);
        assert_eq!(out[1], 0.0);
    }

    #[test]
    fn negative_input_goes_to_bear_channel() {
        let mut b = RustBackend::new();
        b.initialize(None).unwrap();
        let mut inputs = [0.0f32; 8];
        inputs[2] = -0.5;
        let out = b.process_signals(&inputs, 0.0, &make_telem()).unwrap();
        assert_eq!(out[4], 0.0);          // bull channel for ch2
        assert!((out[5] - 0.5).abs() < 1e-5); // bear channel
    }

    #[test]
    fn output_has_16_elements() {
        let mut b = RustBackend::new();
        b.initialize(None).unwrap();
        let out = b.process_signals(&[0.1; 8], 0.0, &make_telem()).unwrap();
        assert_eq!(out.len(), 16);
    }

    #[test]
    fn spike_states_all_false() {
        let b = RustBackend::new();
        assert_eq!(b.get_spike_states(), [false; 16]);
    }
}
