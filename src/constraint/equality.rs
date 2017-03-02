//! Equality implementation.

use ::{Constraint,LinExpr,PuzzleSearch,Val,VarToken};
use intdiv::IntDiv;

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

    fn on_updated(&self, search: &mut PuzzleSearch) -> bool {
        let mut sum_min = self.eqn.constant;
        let mut sum_max = self.eqn.constant;

        for (&var, &coef) in self.eqn.coef.iter() {
            if let Some((min_val, max_val)) = search.get_min_max(var) {
                if coef > 0 {
                    sum_min += coef * min_val;
                    sum_max += coef * max_val;
                } else {
                    sum_min += coef * max_val;
                    sum_max += coef * min_val;
                }
            } else {
                return false;
            }
        }

        // Minimum (maximum) of var can be bounded by summing the
        // maximum (minimum) of everything else.  Repeat until no
        // changes further changes to the extremes found.
        let mut iters = self.eqn.coef.len();
        let mut iter = self.eqn.coef.iter().cycle();
        while iters > 0 {
            iters = iters - 1;
            if !(sum_min <= 0 && 0 <= sum_max) {
                return false;
            }

            let (&var, &coef) = iter.next().expect("cycle");
            if search.is_assigned(var) {
                continue;
            }

            if let Some((min_val, max_val)) = search.get_min_max(var) {
                let min_bnd;
                let max_bnd;

                if coef > 0 {
                    min_bnd = (coef * max_val - sum_max).div_round_up(coef);
                    max_bnd = (coef * min_val - sum_min).div_round_down(coef);
                } else {
                    min_bnd = (coef * max_val - sum_min).div_round_up(coef);
                    max_bnd = (coef * min_val - sum_max).div_round_down(coef);
                }

                if min_val < min_bnd || max_bnd < max_val {
                    search.bound_candidate_range(var, min_bnd, max_bnd);

                    if let Some((new_min, new_max)) = search.get_min_max(var) {
                        if coef > 0 {
                            sum_min = sum_min + coef * (new_min - min_val);
                            sum_max = sum_max + coef * (new_max - max_val);
                        } else {
                            sum_min = sum_min + coef * (new_max - max_val);
                            sum_max = sum_max + coef * (new_min - min_val);
                        }
                    } else {
                        return false;
                    }

                    iters = self.eqn.coef.len();
                }
            } else {
                return false;
            }
        }

        true
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
