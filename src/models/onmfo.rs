//! Online Nonnegative Matrix Factorization with Outliers (https://arxiv.org/abs/1604.02634)
//!
use super::model::Model;
use crate::parameters::onmfo::Params;
use nalgebra::{DMatrix};
use nalgebra_sparse::csc::CscMatrix;
use rand::prelude::*;
use rand::{thread_rng, Rng, SeedableRng};
use rand_distr::{Standard, Distribution};

type Mat2d = DMatrix<f64>;

#[derive(Debug)]
pub struct ONMFO {
    pub(crate) params: Params,
    pub(crate) csc: CscMatrix<f64>,
    pub(crate) mat_w: Mat2d,
    pub(crate) mat_h: Mat2d,
    pub(crate) mat_a: Mat2d,
    pub(crate) mat_b: Mat2d,
}

impl ONMFO {
    pub fn new(params: Params, csc: CscMatrix<f64>) -> Self {
        let mat_a = Mat2d::zeros(params.param.n_components, params.param.n_components);
        let mat_b = Mat2d::zeros(csc.nrows(), params.param.n_components);
        let mat_w = Self::initialize_mat_w(csc.nrows(), params.param.n_components);
        let mat_h = Mat2d::zeros(params.param.n_components, csc.ncols());
        ONMFO {
            params,
            csc,
            mat_w,
            mat_h,
            mat_a,
            mat_b,
        }
    }
    fn initialize_mat_w(nrows: usize, ncols: usize) -> Mat2d {
        let mut rng = thread_rng();
        let data: Vec<f64> = Standard.sample_iter(&mut rng).take(nrows * ncols).collect();
        Mat2d::from_vec(nrows, ncols, data)
    }
    fn solve_proj(&self) {
        let wt = self.mat_w.transpose();
        let wt_w = wt * self.mat_w.clone();
        for i in 0..self.params.param.h_max_iter {
            let v = self.csc.transpose();
            let wt_v = v * &self.mat_w;
            // (w * v)^T = v^T * w^T
            
        }
    }
    fn solve_h(&self) {}
    fn update(&mut self) {}
}

impl Model for ONMFO {
    fn fit(&mut self) -> &mut Self {
        for epoch in 0..self.params.param.epochs {
            self.solve_proj();
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
        let n_tokens = 3;
        let n_documents = 10;
        let mut coo = CooMatrix::<f64>::new(n_tokens, n_documents);

        coo.push(2, 0, 1.0);
        coo.push(2, 1, 1.0);
        let csc = CscMatrix::from(&coo);
    }
    #[test]
    fn test_nmf_initialize_mat_w() {
        let w = ONMFO::initialize_mat_w(50, 3);
        assert_eq!(w.shape(), (50, 3));
    }

    #[test]
    fn test_solveproj() {
        let n_tokens = 3;
        let n_documents = 10;
        let mut coo = CooMatrix::<f64>::new(n_tokens, n_documents);
        coo.push(0, 0, 1.0);
        coo.push(1, 1, 2.0);
        coo.push(2, 1, 4.0);
        coo.push(2, 5, 3.0);
        coo.push(2, 9, 10.0);
        let csc = CscMatrix::from(&coo);
    }
    #[test]
    fn mat_mul() {
        let col_offsets = vec![0, 1, 3, 4, 5];
        let row_indices = vec![0, 0, 2, 2, 0];
        let values = vec![1.0, 2.0, 3.0, 4.0, 5.0];

        // The constructor validates the raw CSC data and returns an error if it is invalid.
        let mut csc = CscMatrix::try_from_csc_data(4, 4, col_offsets, row_indices, values)
        .unwrap();
        assert_eq!(csc.ncols(), 4);
        let z = DMatrix::<f64>::identity(4, 4);
        let w =  csc * z; //xTx * z;
        println!("{:?}", w);
        assert_eq!(w.ncols(), 4);
        assert_eq!(w.nrows(), 4);
    }
}
