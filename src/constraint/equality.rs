//! Equality implementation.

use ::{Constraint,LinExpr,PuzzleSearch,Val,VarToken};

pub struct Equality {
    // The equation: 0 = constant + coef1 * var1 + coef2 * var2 + ...
    eqn: LinExpr,
}

impl Equality {
    /// Allocate a new Equality constraint.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut magic_square = puzzle_solver::Puzzle::new();
    /// let vars = magic_square.new_vars_with_candidates_2d(3, 3,
    ///         &[1,2,3,4,5,6,7,8,9]);
    ///
    /// puzzle_solver::constraint::Equality::new(
    ///         vars[0][0] + vars[0][1] + vars[0][2] - 15);
    /// ```
    pub fn new(eqn: LinExpr) -> Self {
        Equality {
            eqn: eqn,
        }
    }
}

impl Constraint for Equality {
    fn vars<'a>(&'a self) -> Box<Iterator<Item=&'a VarToken> + 'a> {
        Box::new(self.eqn.coef.keys())
    }

    fn on_assigned(&self, search: &mut PuzzleSearch, _: VarToken, _: Val)
            -> bool {
        let mut sum = self.eqn.constant;
        let mut unassigned_var = None;

        for (&var, &coef) in self.eqn.coef.iter() {
            if let Some(val) = search.get_assigned(var) {
                sum += coef * val;
            } else {
                // If we find more than one unassigned variable,
                // cannot assign any other variables.
                if unassigned_var.is_some() {
                    return true;
                } else {
                    unassigned_var = Some((var, coef));
                }
            }
        }

        // If we have exactly one unassigned variable, assign it.
        if let Some((var, coef)) = unassigned_var {
            // sum + coef * var = 0.
            let val = -sum / coef;
            if sum + coef * val == 0 {
                search.set_candidate(var, val);
            } else {
                return false;
            }
        } else {
            if sum != 0 {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use ::Puzzle;
    use super::Equality;

    #[test]
    fn test_contradiction() {
        let mut puzzle = Puzzle::new();
        let v0 = puzzle.new_var_with_candidates(&[3]);
        let v1 = puzzle.new_var_with_candidates(&[0,1]);

        puzzle.add_constraint(Box::new(Equality::new(v0 + 2 * v1 - 4)));

        let search = puzzle.step();
        assert!(search.is_none());
    }

    #[test]
    fn test_assign() {
        let mut puzzle = Puzzle::new();
        let v0 = puzzle.new_var_with_candidates(&[1]);
        let v1 = puzzle.new_var_with_candidates(&[1,2,3]);

        puzzle.add_constraint(Box::new(Equality::new(v0 + v1 - 4)));

        let search = puzzle.step().expect("contradiction");
        assert_eq!(search[v0], 1);
        assert_eq!(search[v1], 3);
    }
}
