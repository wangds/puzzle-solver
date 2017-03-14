//! Equality implementation.

use std::rc::Rc;
use num_rational::Ratio;
use num_traits::Zero;

use ::{Constraint,LinExpr,PsResult,PuzzleSearch,Val,VarToken};

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
            -> PsResult<()> {
        let mut sum = self.eqn.constant;
        let mut unassigned_var = None;

        for (&var, &coef) in self.eqn.coef.iter() {
            if let Some(val) = search.get_assigned(var) {
                sum = sum + coef * Ratio::from_integer(val);
            } else {
                // If we find more than one unassigned variable,
                // cannot assign any other variables.
                if unassigned_var.is_some() {
                    return Ok(());
                } else {
                    unassigned_var = Some((var, coef));
                }
            }
        }

        // If we have exactly one unassigned variable, assign it.
        if let Some((var, coef)) = unassigned_var {
            // sum + coef * var = 0.
            let val = -sum / coef;
            if val.is_integer() {
                try!(search.set_candidate(var, val.to_integer()));
            } else {
                return Err(());
            }
        } else {
            if !sum.is_zero() {
                return Err(());
            }
        }

        Ok(())
    }

    fn on_updated(&self, search: &mut PuzzleSearch) -> PsResult<()> {
        let mut sum_min = self.eqn.constant;
        let mut sum_max = self.eqn.constant;

        for (&var, &coef) in self.eqn.coef.iter() {
            let (min_val, max_val) = try!(search.get_min_max(var));
            if coef > Ratio::zero() {
                sum_min = sum_min + coef * Ratio::from_integer(min_val);
                sum_max = sum_max + coef * Ratio::from_integer(max_val);
            } else {
                sum_min = sum_min + coef * Ratio::from_integer(max_val);
                sum_max = sum_max + coef * Ratio::from_integer(min_val);
            }
        }

        // Minimum (maximum) of var can be bounded by summing the
        // maximum (minimum) of everything else.  Repeat until no
        // changes further changes to the extremes found.
        let mut iters = self.eqn.coef.len();
        let mut iter = self.eqn.coef.iter().cycle();
        while iters > 0 {
            iters = iters - 1;
            if !(sum_min <= Ratio::zero() && Ratio::zero() <= sum_max) {
                return Err(());
            }

            let (&var, &coef) = iter.next().expect("cycle");
            if search.is_assigned(var) {
                continue;
            }

            let (min_val, max_val) = try!(search.get_min_max(var));
            let (min_bnd, max_bnd);

            if coef > Ratio::zero() {
                min_bnd = ((coef * Ratio::from_integer(max_val) - sum_max) / coef).ceil().to_integer();
                max_bnd = ((coef * Ratio::from_integer(min_val) - sum_min) / coef).floor().to_integer();
            } else {
                min_bnd = ((coef * Ratio::from_integer(max_val) - sum_min) / coef).ceil().to_integer();
                max_bnd = ((coef * Ratio::from_integer(min_val) - sum_max) / coef).floor().to_integer();
            }

            if min_val < min_bnd || max_bnd < max_val {
                let (new_min, new_max)
                    = try!(search.bound_candidate_range(var, min_bnd, max_bnd));

                if coef > Ratio::zero() {
                    sum_min = sum_min + coef * Ratio::from_integer(new_min - min_val);
                    sum_max = sum_max + coef * Ratio::from_integer(new_max - max_val);
                } else {
                    sum_min = sum_min + coef * Ratio::from_integer(new_max - max_val);
                    sum_max = sum_max + coef * Ratio::from_integer(new_min - min_val);
                }

                iters = self.eqn.coef.len();
            }
        }

        Ok(())
    }

    fn substitute(&self, from: VarToken, to: VarToken)
            -> PsResult<Rc<Constraint>> {
        let mut eqn = self.eqn.clone();
        if let Some(coef) = eqn.coef.remove(&from) {
            eqn = eqn + coef * to;
        }

        Ok(Rc::new(Equality{ eqn: eqn }))
    }
}

#[cfg(test)]
mod tests {
    use ::{Puzzle,Val};

    #[test]
    fn test_contradiction() {
        let mut puzzle = Puzzle::new();
        let v0 = puzzle.new_var_with_candidates(&[3]);
        let v1 = puzzle.new_var_with_candidates(&[0,1]);

        puzzle.equals(v0 + 2 * v1, 4);

        let search = puzzle.step();
        assert!(search.is_none());
    }

    #[test]
    fn test_assign() {
        let mut puzzle = Puzzle::new();
        let v0 = puzzle.new_var_with_candidates(&[1]);
        let v1 = puzzle.new_var_with_candidates(&[1,2,3]);

        puzzle.equals(v0 + v1, 4);

        let search = puzzle.step().expect("contradiction");
        assert_eq!(search[v0], 1);
        assert_eq!(search[v1], 3);
    }

    #[test]
    fn test_reduce_range() {
        let mut puzzle = Puzzle::new();
        let v0 = puzzle.new_var_with_candidates(&[1,2,3]);
        let v1 = puzzle.new_var_with_candidates(&[3,4,5]);

        puzzle.equals(v0 + v1, 5);

        let search = puzzle.step().expect("contradiction");
        assert_eq!(search.get_unassigned(v0).collect::<Vec<Val>>(), &[1,2]);
        assert_eq!(search.get_unassigned(v1).collect::<Vec<Val>>(), &[3,4]);
    }
}
