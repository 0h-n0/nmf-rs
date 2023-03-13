#[derive(Debug)]
pub struct Params {
    pub(crate) param: PLSAParams,
}

#[derive(Default, Debug)]
pub struct PLSAParams {
    pub(crate) n_components: usize,
}

impl Params {
    pub fn new() -> Self {
        let nmfparams = PLSAParams {
            ..Default::default()
        };
        let mut param = Self { param: nmfparams };
        // set default values
        param.n_components(2);
        param
    }
    pub fn n_components(&mut self, value: usize) -> &mut Self {
        self.param.n_components = value;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new() {
        let param = Params::new();        
    }
}
