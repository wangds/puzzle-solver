//! This crate searches for the solutions to logic puzzles.
//! The puzzle rules are expressed as constraints.

extern crate bit_set;

use std::ops::Index;

pub use puzzle::Puzzle;
pub use puzzle::PuzzleSearch;

/// A puzzle variable token.
#[derive(Copy,Clone,Debug,Eq,PartialEq)]
pub struct VarToken(usize);

/// The type of a puzzle variable's value (i.e. the candidate type).
pub type Val = i32;

/// A dictionary mapping puzzle variables to the solution value.
#[derive(Debug)]
pub struct Solution {
    vars: Vec<Val>,
}

mod puzzle;

impl Index<VarToken> for Solution {
    type Output = Val;
    fn index(&self, var: VarToken) -> &Val {
        let VarToken(idx) = var;
        &self.vars[idx]
    }
}
