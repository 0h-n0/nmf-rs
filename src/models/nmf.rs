//! Online Nonnegative Matrix Factorization with Outliers(https://arxiv.org/abs/1604.02634]
//!
use super::model::Model;
use super::params::Params;
use nalgebra::{Dyn, OMatrix};
use nalgebra_sparse::csc::CscMatrix;

type Mat2d = OMatrix<f64, Dyn, Dyn>;

#[derive(Debug)]
pub struct NMF {
    nmf_parameters: Params,
    v: CscMatrix<f64>,
    W: Mat2d,
    h: Mat2d,
}

impl NMF {
    pub fn new(nmf_parameters: Params, v: CscMatrix<f64>) -> Self {
        let A = Mat2d::zeros(10, 10);
        let B = Mat2d::zeros(10, 10);
        let W = NMF::setup_w();
        let h = NMF::setup_w();
        NMF {
            nmf_parameters,
            v,
            W,
            h,
        }
    }
    fn setup_w() -> Mat2d {
        Mat2d::zeros(10, 10)
    }
    fn solve_proj(self, v: CscMatrix<f64>, W: Mat2d) {}
    fn update(&mut self) {}
}

impl Model for NMF {
    fn fit(&self) {}
    fn save(&self) {}
    fn load(&self) {}
}

mod tests {
    use super::*;
    use nalgebra_sparse::coo::CooMatrix;

    #[test]
    fn init_test() {
        let mut coo = CooMatrix::<f64>::new(3, 3);
        coo.push(2, 0, 1.0);
        coo.push(2, 1, 1.0);
        let csc = CscMatrix::from(&coo);
    }
}
