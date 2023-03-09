use nalgebra_sparse::coo::CooMatrix;
use nalgebra_sparse::csc::CscMatrix;

use nmf_rs;

#[test]
fn integrated_test() {
    let mut coo = CooMatrix::<f64>::new(3, 3);
    coo.push(2, 0, 1.0);
    coo.push(2, 1, 1.0);
    let csc = CscMatrix::from(&coo);
    let mut params = nmf_rs::Params::new();
    let nmfinputs = nmf_rs::NMFInputs::new(csc, 3, 3);
    let model = nmf_rs::NMF::new(params, nmfinputs);
    assert_eq!(1, 1);
}

#[test]
fn parameter_access_from_non_internal_create() {}
