#[derive(Debug)]
pub struct Params {
    param: NMFParams,
}

#[derive(Default, Debug)]
pub struct NMFParams {
    n_components: u64,
    batch_size: u64,
    kappa: f64,
    minimum_probability: f64,
    w_max_iter: u64,
    w_stop_condtions: f64,
    h_max_iter: u64,
    h_stop_condtions: f64,
    eval_every: u64,
    normalize: bool,
    random_state: Option<i32>,
}

impl Params {
    pub fn new() -> Self {
        let mut nmfparams = NMFParams {
            ..Default::default()
        };
        let mut param = Self { param: nmfparams };
        // set default values
        param.n_components(3).batch_size(100);
        param
    }
    pub fn n_components(&mut self, value: u64) -> &mut Self {
        self.param.n_components = value;
        self
    }
    pub fn batch_size(&mut self, value: u64) -> &mut Self {
        self.param.batch_size = value;
        self
    }
    pub fn kappa(&mut self, value: f64) -> &mut Self {
        self.param.kappa = value;
        self
    }
    pub fn minimum_probability(&mut self, value: f64) -> &mut Self {
        self.param.minimum_probability = value;
        self
    }
    pub fn w_max_iter(&mut self, value: u64) -> &mut Self {
        self.param.w_max_iter = value;
        self
    }
    pub fn w_stop_condtions(&mut self, value: f64) -> &mut Self {
        self.param.w_stop_condtions = value;
        self
    }
    pub fn h_max_iter(&mut self, value: u64) -> &mut Self {
        self.param.h_max_iter = value;
        self
    }
    pub fn h_stop_condtions(&mut self, value: f64) -> &mut Self {
        self.param.h_stop_condtions = value;
        self
    }
    pub fn eval_every(&mut self, value: u64) -> &mut Self {
        self.param.eval_every = value;
        self
    }
    pub fn normalize(&mut self, value: bool) -> &mut Self {
        self.param.normalize = value;
        self
    }
    pub fn random_state(&mut self, value: Option<i32>) -> &mut Self {
        self.param.random_state = value;
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn new() {
        let param = Params::new();
        assert_eq!(param.param.eval_every, 0);
    }
}
