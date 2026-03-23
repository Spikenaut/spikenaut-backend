//! Data types that flow over the SNN backend IPC wire.

use serde::{Deserialize, Serialize};

/// 4-neuromodulator snapshot decoded from the Julia brain's 88-byte NERO packet.
///
/// # Wire format (bytes 72–87 of the NERO IPC packet)
/// ```text
/// [72..76]  dopamine       f32 LE   reward / learning-rate gate
/// [76..80]  cortisol       f32 LE   stress / inhibition
/// [80..84]  acetylcholine  f32 LE   focus / signal-to-noise
/// [84..88]  tempo          f32 LE   clock-driven timing scale
/// ```
///
/// # References
///
/// - Schultz, W. (1998). Predictive reward signal of dopamine neurons.
///   *Journal of Neurophysiology*, 80(1), 1–27.
/// - Arnsten, A. F. T. (2009). Stress signalling pathways that impair
///   prefrontal cortex structure and function.
///   *Nature Reviews Neuroscience*, 10(6), 410–422.
/// - Hasselmo, M. E. (1999). Neuromodulation: acetylcholine and memory
///   consolidation. *Trends in Cognitive Sciences*, 3(9), 351–359.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NeroManifoldSnapshot {
    /// Tick counter from the Julia brain (monotonically increasing).
    pub tick: i64,
    /// Dopamine level (reward / STDP learning-rate gate). Range [0, 1].
    pub dopamine: f32,
    /// Cortisol level (thermal/power stress inhibition). Range [0, 1].
    pub cortisol: f32,
    /// Acetylcholine level (focus / signal-to-noise ratio). Range [0, 1].
    pub acetylcholine: f32,
    /// Tempo scale (clock-driven timing; 1.0 = nominal). Range [0.5, 2.0].
    pub tempo: f32,
}

impl NeroManifoldSnapshot {
    /// Parse from the 4 NERO score floats in bytes `[72..88]` of a NERO packet.
    pub fn from_scores(tick: i64, scores: &[f32; 4]) -> Self {
        Self {
            tick,
            dopamine:      scores[0],
            cortisol:      scores[1],
            acetylcholine: scores[2],
            tempo:         scores[3],
        }
    }
}
