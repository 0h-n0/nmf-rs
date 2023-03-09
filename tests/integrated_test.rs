use nalgebra_sparse::coo::CooMatrix;
use nalgebra_sparse::csc::CscMatrix;

use matrix_factorization_rs;

#[test]
fn integrated_test() {
    let mut coo = CooMatrix::<f64>::new(3, 3);
    coo.push(2, 0, 1.0);
    coo.push(2, 1, 1.0);
    let csc = CscMatrix::from(&coo);
    let mut params = matrix_factorization_rs::models::params::Params::new();
    let model = matrix_factorization_rs::models::nmf::NMF::new(params, csc);
    assert_eq!(1, 1);
}
