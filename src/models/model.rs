pub trait Model {
    fn fit(&mut self) -> &mut Self;
    fn save(&self);
    fn load(&self);
}
