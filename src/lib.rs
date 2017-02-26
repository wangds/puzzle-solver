//! This crate searches for the solutions to logic puzzles.
//! The puzzle rules are expressed as constraints.

extern crate bit_set;

use std::collections::HashMap;
use std::ops::Index;

pub use constraint::Constraint;
pub use puzzle::Puzzle;
pub use puzzle::PuzzleSearch;

/// A puzzle variable token.
#[derive(Copy,Clone,Debug,Eq,Hash,PartialEq)]
pub struct VarToken(usize);

/// The type of a puzzle variable's value (i.e. the candidate type).
pub type Val = i32;

/// The type of the coefficients in a linear expression.
pub type Coef = i32;

/// A linear expression.
///
/// ```text
///   constant + coef1 * var1 + coef2 * var2 + ...
/// ```
#[derive(Clone)]
pub struct LinExpr {
    constant: Coef,

    // The non-zero coefficients in the linear expression.  If, after
    // some manipulations, the coefficient is 0, then it must be
    // removed from the map.
    coef: HashMap<VarToken, Coef>,
}

/// A dictionary mapping puzzle variables to the solution value.
#[derive(Debug)]
pub struct Solution {
    vars: Vec<Val>,
}

pub mod constraint;

mod linexpr;
mod puzzle;

impl Index<VarToken> for Solution {
    type Output = Val;
    fn index(&self, var: VarToken) -> &Val {
        let VarToken(idx) = var;
        &self.vars[idx]
    }
}
