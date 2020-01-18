extern crate chrono;
extern crate rayon;

pub use chrono::prelude::NaiveDate;

pub mod club;
pub mod continent;
pub mod core;
pub mod country;
pub mod league;
pub mod r#match;
pub mod people;
pub mod simulator;

pub mod shared;
pub mod utils;

mod generators;

pub use crate::core::SimulationContext;

pub use self::core::*;
pub use club::*;
pub use country::*;
pub use generators::*;
pub use league::*;
pub use people::*;
pub use r#match::*;
pub use shared::*;
pub use simulator::*;
pub use utils::*;
