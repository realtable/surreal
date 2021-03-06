//! Implementation of J. H. Conway's surreal numbers, as explained in the book
//! *[Surreal Numbers](https://www.amazon.com/dp/0201038129)* by Donald Knuth. This crate provides
//! an interface to the rules and theorems in the book, as well as a comprehensive surreal type.

mod conv;
mod inf;
mod repr;

pub use conv::ftos;
pub use conv::stof;
pub use inf::SurrealInf;
pub use repr::is_divisible;
pub use repr::is_pseudo;
pub use repr::Surreal;

#[cfg(test)]
mod tests;
