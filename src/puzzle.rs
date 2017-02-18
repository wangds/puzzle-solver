//! The puzzle's state and rules.

/// The puzzle to be solved.
#[allow(dead_code)]
pub struct Puzzle {
    // The number of variables in the puzzle.
    num_vars: usize,
}

impl Puzzle {
    /// Allocate a new puzzle.
    ///
    /// # Examples
    ///
    /// ```
    /// puzzle_solver::Puzzle::new();
    /// ```
    pub fn new() -> Self {
        Puzzle {
            num_vars: 0,
        }
    }
}
