//! Online Nonnegative Matrix Factorization with Outliers(https://arxiv.org/abs/1604.02634]
//!
use super::model::Model;
use super::params::Params;
use nalgebra::{Dyn, OMatrix};
use nalgebra_sparse::csc::CscMatrix;
use rand::{thread_rng, Rng};

type Mat2d = OMatrix<f64, Dyn, Dyn>;

#[derive(Debug)]
pub struct NMFInputs {
    csc_matrix: CscMatrix<f64>,
    nrows: usize,
    ncols: usize,
}

impl NMFInputs {
    pub fn new(csc_matrix: CscMatrix<f64>, nrows: usize, ncols: usize) -> Self {
        NMFInputs {
            csc_matrix,
            nrows,
            ncols,
        }
    }
}

#[derive(Debug)]
pub struct NMF {
    pub(crate) params: Params,
    pub(crate) inputs: NMFInputs,
    pub(crate) mat_w: Mat2d,
    pub(crate) mat_h: Mat2d,
    pub(crate) mat_a: Mat2d,
    pub(crate) mat_b: Mat2d,
}

impl NMF {
    pub fn new(params: Params, inputs: NMFInputs) -> Self {
        let mat_a = Mat2d::zeros(params.param.n_components, params.param.n_components);
        let mat_b = Mat2d::zeros(inputs.nrows, params.param.n_components);
        let mat_w = Self::initialize_mat_w(inputs.nrows, params.param.n_components);
        let mat_h = Mat2d::zeros(params.param.n_components, inputs.ncols);
        NMF {
            params,
            inputs,
            mat_w,
            mat_h,
            mat_a,
            mat_b,
        }
    }
    fn initialize_mat_w(nrows: usize, ncols: usize) -> Mat2d {
        Mat2d::zeros(nrows, ncols)
    }
    fn solve_proj(self, v: CscMatrix<f64>, W: Mat2d) {}
    fn update(&mut self) {}
}

impl Model for NMF {
    fn fit(&mut self) -> &mut Self {
        for epoch in 0..self.params.param.epochs {
            println!("{}", epoch);
        }
        self
    }
    fn save(&self) {}
    fn load(&self) {}
}

mod tests {
    use super::*;
    use nalgebra_sparse::coo::CooMatrix;

    #[test]
    fn nmf_new() {
        let n_tokens = 10;
        let n_documents = 100;
        let mut coo = CooMatrix::<f64>::new(n_tokens, n_documents);

        coo.push(2, 0, 1.0);
        coo.push(2, 1, 1.0);
        let csc = CscMatrix::from(&coo);
        let input = NMFInputs::new(csc, n_tokens, n_documents);
    }
}
