pub trait Model {
    fn fit(&self);
    fn save(&self);
    fn load(&self);
}
