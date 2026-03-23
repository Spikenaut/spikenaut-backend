//! `TraderBackend` trait and `BackendType` enumeration.

use crate::{BackendError, GpuTelemetry, RustBackend};

/// Unified interface for SNN signal processing backends.
///
/// Abstracts over the neural processing layer, allowing different backend
/// implementations (Rust-native, Julia jlrs, ZMQ IPC) to be used
/// interchangeably.
///
/// # Output contract
///
/// `process_signals` returns a `Vec<f32>`:
///
/// - Most backends return **16 elements** — one per lobe/neuron readout.
/// - [`ZmqBrainBackend`](crate::ZmqBrainBackend) returns **20 elements**:
///   `[0..16]` lobe readout, `[16..20]` NERO scores
///   `[dopamine, cortisol, acetylcholine, tempo]`.
///
/// Callers that only need the readout should index `output[..16]` and access
/// extended fields via `output.get(i).copied().unwrap_or(0.0)`.
pub trait TraderBackend: Send + Sync {
    /// Process 8-channel normalised market signals through the SNN.
    ///
    /// # Arguments
    /// - `normalized_inputs` — 8 Z-scored market signal channels
    /// - `inhibition_signal` — thermal inhibition scalar (GPU temperature proxy)
    /// - `telemetry` — live GPU telemetry for neuromodulation
    fn process_signals(
        &mut self,
        normalized_inputs: &[f32; 8],
        inhibition_signal: f32,
        telemetry: &GpuTelemetry,
    ) -> Result<Vec<f32>, BackendError>;

    /// Initialise backend (load model weights, connect to IPC socket, etc.).
    ///
    /// Must be called before `process_signals`. Idempotent on success.
    fn initialize(&mut self, model_path: Option<&str>) -> Result<(), BackendError>;

    /// Persist current model state to `model_path`.
    fn save_state(&self, model_path: &str) -> Result<(), BackendError>;

    /// Return per-neuron spike states (true = spiked on last tick).
    fn get_spike_states(&self) -> [bool; 16];

    /// Reset internal network state (membrane potentials, caches).
    fn reset(&mut self) -> Result<(), BackendError>;
}

/// Backend implementation selector.
#[derive(Debug, Clone, Copy, Default)]
pub enum BackendType {
    /// Pure-Rust native backend (always available, no external deps).
    #[default]
    Rust,
    /// Julia IPC backend via ZMQ SUB socket (requires feature `zmq`).
    #[cfg(feature = "zmq")]
    ZmqBrain,
}

/// Factory for creating `TraderBackend` instances.
pub struct BackendFactory;

impl BackendFactory {
    pub fn create(backend_type: BackendType) -> Box<dyn TraderBackend> {
        match backend_type {
            BackendType::Rust => Box::new(RustBackend::new()),
            #[cfg(feature = "zmq")]
            BackendType::ZmqBrain => Box::new(crate::ZmqBrainBackend::new()),
        }
    }
}
