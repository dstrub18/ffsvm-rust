mod linear;
mod poly;
mod rbf;
mod sigmoid;

use crate::{
    f32s,
    sparse::{SparseMatrix, SparseVector},
};
use simd_aligned::{MatrixD, Rows, VectorD};

pub use self::{linear::*, poly::*, rbf::*, sigmoid::*};

/// Base trait for kernels
#[doc(hidden)]
pub trait KernelDense
where
    Self: Send + Sync,
{
    fn compute(&self, vectors: &MatrixD<f32s, Rows>, feature: &VectorD<f32s>, output: &mut [f64]);
}

/// Base trait for kernels
#[doc(hidden)]
pub trait KernelSparse
where
    Self: Send + Sync,
{
    fn compute(&self, vectors: &SparseMatrix<f32>, feature: &SparseVector<f32>, output: &mut [f64]);
}
