//! This crate searches for the solutions to logic puzzles.
//! The puzzle rules are expressed as constraints.

extern crate bit_set;

pub use puzzle::Puzzle;

/// A puzzle variable token.
#[derive(Copy,Clone,Debug,Eq,PartialEq)]
pub struct VarToken(usize);

/// The type of a puzzle variable's value (i.e. the candidate type).
pub type Val = i32;

mod puzzle;
