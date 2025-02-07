//! # CPU Prover Builder
//!
//! This module provides a builder for the [`CpuProver`].

use sp1_prover::SP1Prover;

use super::CudaProver;

/// A builder for the [`CudaProver`].
///
/// The builder is used to configure the [`CudaProver`] before it is built.
pub struct CudaProverBuilder {
    gpu_number: Option<u32>,
}

impl CudaProverBuilder {
    /// Create default CudaProverBuilder
    pub fn new() -> Self {
        CudaProverBuilder {
            gpu_number: None,
        }
    }

    /// Set the GPU flag for Docker run.
    pub fn with_gpu_number(mut self, flag: u32) -> Self {
        self.gpu_number = Some(flag);
        self
    }

    /// Builds a [`CudaProver`].
    ///
    /// # Details
    /// This method will build a [`CudaProver`] with the given parameters. In particular, it will
    /// build a mock prover if the `mock` flag is set.
    ///
    /// # Example
    /// ```rust,no_run
    /// use sp1_sdk::ProverClient;
    ///
    /// let prover = ProverClient::builder().cuda().build();
    /// ```
    #[must_use]
    pub fn build(self) -> CudaProver {
        CudaProver::new(SP1Prover::new(), self.gpu_number)
    }
}
