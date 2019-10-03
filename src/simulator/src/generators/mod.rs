mod generator;

pub use crate::models::*;

pub trait Generator{
    fn generate() -> Self;
}