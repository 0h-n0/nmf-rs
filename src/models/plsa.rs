//! Bayesian Reasoning and Machine Learning(Dabid Barver) 2012

use super::model::Model;
use crate::parameters::onmfo::Params;
use log::{info, trace, debug};
use nalgebra::{DMatrix, DVector};
use nalgebra_sparse::csc::CscMatrix;
use rand::prelude::*;
use rand::{thread_rng, Rng, SeedableRng};
use rand_distr::{Standard, Distribution};

type Mat2d = DMatrix<f64>;

#[derive(Debug)]
pub struct PLSA {
    pub(crate) params: Params,
    pub(crate) csc: CscMatrix<f64>,
    pub(crate) px_z: Mat2d,
    pub(crate) py_z: Mat2d,
    pub(crate) pz: DVector<f64>,
}


impl PLSA {
    pub fn new(params: Params, csc: CscMatrix<f64>) -> Self {
        let mut px_z = Mat2d::new_random(params.param.n_components, csc.ncols());
        let mut py_z = Mat2d::new_random(csc.nrows(), params.param.n_components);
        let mut pz = DVector::new_random(params.param.n_components);        
        PLSA::normalize_columns(&mut px_z);
        PLSA::normalize_columns(&mut py_z);
        pz /= pz.sum();
        PLSA {
            params,
            csc,
            px_z,
            py_z,
            pz,
        }
    }
    fn normalize_columns(mat: &mut Mat2d) {
        let a = &mat.column_sum();
        println!("{}, {}", mat.nrows(), mat.ncols());
        println!("{:?}", a);
        for i in 0..mat.nrows() {
            for j in 0..mat.ncols() {
                println!("{}, {}", i, j);
                mat[(i, j)] = mat[(i, j)] / a[i];
            }
        }
    }
    fn e_step(&mut self) {
        let pz_xy = Mat2d::zeros(self.n_com)
        //* self.py_z.clone();
        println!("{:?}", pz_xy.shape());
        
    }
    fn m_step(&mut self) {
        
    }
    fn llh(&self) -> f64{

        0.0
    }

}

impl Model for PLSA {
    fn fit(&mut self) -> &mut Self {        
        let mut prev_llh = 1e6;
        let steps = 200;
        let tolerance = 1.0e-7;
        for _ in 0..steps {
            self.e_step();
            self.m_step();
            let llh = self.llh();            
            if ((llh - prev_llh) / prev_llh).abs() < tolerance {
                break
            }
            prev_llh = llh;
        }
        self
    }
    fn load(&self) {        
    }
    fn save(&self) {        
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use nalgebra_sparse::coo::CooMatrix;
    use std::{println as debug};
    
    #[test]
    fn init() {
        let mut params = Params::new();   
        params.n_components(2);
        let n_tokens = 3;
        let n_documents = 10;
        let mut coo = CooMatrix::<f64>::new(n_tokens, n_documents);
        coo.push(0, 0, 1.0);
        coo.push(1, 1, 2.0);
        coo.push(2, 1, 4.0);
        coo.push(2, 5, 3.0);
        coo.push(2, 9, 10.0);
        let csc = CscMatrix::from(&coo);    
        let mut plsa = PLSA::new(params, csc);
        plsa.fit();
    }


    #[test]
    fn mat_dist() {
        let mut m = Mat2d::new_random(10, 5);
        let a = &m.column_sum();
        for i in 0..m.nrows() {
            for j in 0..m.ncols() {
               m[(i, j)] = m[(i, j)] / a[j];
            }
        }
    }
}