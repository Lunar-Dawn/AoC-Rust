use std::collections::BTreeMap;

mod runner;
mod year;

mod dyn_result;
pub mod parse;

pub mod grid;
mod impl_operators;
pub mod point2i;
pub mod vec2i;

pub use dyn_result::DynResult;
pub type Solution = fn(&str) -> (String, String);
pub type YearSolutions = BTreeMap<usize, Solution>;
