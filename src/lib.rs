pub mod models;
pub use models::nmf::NMF;
pub use models::nmf::NMFInputs;
pub use models::params::Params;

#[cfg(test)]
mod tests {
    use super::*;
}
