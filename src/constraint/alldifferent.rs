//! All different implementation.

use ::{Constraint,PuzzleSearch,Val,VarToken};

pub struct AllDifferent {
    vars: Vec<VarToken>,
}

impl AllDifferent {
    /// Allocate a new All Different constraint.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut send_more_money = puzzle_solver::Puzzle::new();
    /// let vars = send_more_money.new_vars_with_candidates_1d(8,
    ///         &[0,1,2,3,4,5,6,7,8,9]);
    ///
    /// puzzle_solver::constraint::AllDifferent::new(&vars);
    /// ```
    pub fn new(vars: &[VarToken]) -> Self {
        AllDifferent {
            vars: vars.to_vec(),
        }
    }
}

impl Constraint for AllDifferent {
    fn on_assigned(&self, search: &mut PuzzleSearch, var: VarToken, val: Val)
            -> bool {
        // TODO: constraints should only be called if affected variables are modified.
        if !self.vars.iter().any(|&v| v == var) {
            return true;
        }

        for &var2 in self.vars.iter() {
            if !search.is_assigned(var2) {
                search.remove_candidate(var2, val);
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use ::{Puzzle,Val};
    use super::AllDifferent;

    #[test]
    fn test_contradiction() {
        let mut puzzle = Puzzle::new();
        let v0 = puzzle.new_var_with_candidates(&[1]);
        let v1 = puzzle.new_var_with_candidates(&[1]);

        puzzle.add_constraint(Box::new(AllDifferent::new(&[v0,v1])));

        let solution = puzzle.solve_any();
        assert!(solution.is_none());
    }

    #[test]
    fn test_elimination() {
        let mut puzzle = Puzzle::new();
        let v0 = puzzle.new_var_with_candidates(&[1]);
        let v1 = puzzle.new_var_with_candidates(&[1,2,3]);
        let v2 = puzzle.new_var_with_candidates(&[1,2,3]);

        puzzle.add_constraint(Box::new(AllDifferent::new(&[v0,v1,v2])));

        let search = puzzle.step().expect("contradiction");
        assert_eq!(search[v0], 1);
        assert_eq!(search.get_unassigned(v1).collect::<Vec<Val>>(), &[2,3]);
        assert_eq!(search.get_unassigned(v2).collect::<Vec<Val>>(), &[2,3]);
    }
}
