//! All different implementation.

use std::collections::HashMap;
use std::rc::Rc;

use ::{Constraint,PsResult,PuzzleSearch,Val,VarToken};

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
    pub fn new<'a, I>(vars: I) -> Self
            where I: IntoIterator<Item=&'a VarToken> {
        AllDifferent {
            vars: vars.into_iter().cloned().collect(),
        }
    }
}

impl Constraint for AllDifferent {
    fn vars<'a>(&'a self) -> Box<Iterator<Item=&'a VarToken> + 'a> {
        Box::new(self.vars.iter())
    }

    fn on_assigned(&self, search: &mut PuzzleSearch, var: VarToken, val: Val)
            -> PsResult<()> {
        for &var2 in self.vars.iter().filter(|&v| *v != var) {
            try!(search.remove_candidate(var2, val));
        }

        Ok(())
    }

    fn on_updated(&self, search: &mut PuzzleSearch) -> PsResult<()> {
        // Build a table of which values can be assigned to which variables.
        let mut num_unassigned = 0;
        let mut all_candidates = HashMap::new();

        for &var in self.vars.iter().filter(|&var| !search.is_assigned(*var)) {
            num_unassigned = num_unassigned + 1;

            for val in search.get_unassigned(var) {
                if all_candidates.contains_key(&val) {
                    all_candidates.insert(val, None);
                } else {
                    all_candidates.insert(val, Some(var));
                }
            }
        }

        if num_unassigned > all_candidates.len() {
            // More unassigned variables than candidates, contradiction.
            return Err(());
        } else if num_unassigned == all_candidates.len() {
            // As many as variables as candidates.
            for (&val, &opt) in all_candidates.iter() {
                if let Some(var) = opt {
                    try!(search.set_candidate(var, val));
                }
            }
        }

        Ok(())
    }

    fn substitute(&self, from: VarToken, to: VarToken)
            -> PsResult<Rc<Constraint>> {
        if let Some(idx) = self.vars.iter().position(|&var| var == from) {
            if !self.vars.contains(&to) {
                let mut new_vars = self.vars.clone();
                new_vars[idx] = to;
                return Ok(Rc::new(AllDifferent{ vars: new_vars }));
            }
        }

        Err(())
    }
}

#[cfg(test)]
mod tests {
    use ::{Puzzle,Val};

    #[test]
    fn test_contradiction() {
        let mut puzzle = Puzzle::new();
        let v0 = puzzle.new_var_with_candidates(&[1]);
        let v1 = puzzle.new_var_with_candidates(&[1]);
        let v2 = puzzle.new_var_with_candidates(&[1,2,3]);

        puzzle.all_different(&[v0,v1,v2]);

        let solution = puzzle.solve_any();
        assert!(solution.is_none());
    }

    #[test]
    fn test_elimination() {
        let mut puzzle = Puzzle::new();
        let v0 = puzzle.new_var_with_candidates(&[1]);
        let v1 = puzzle.new_var_with_candidates(&[1,2,3]);
        let v2 = puzzle.new_var_with_candidates(&[1,2,3]);

        puzzle.all_different(&[v0,v1,v2]);

        let search = puzzle.step().expect("contradiction");
        assert_eq!(search[v0], 1);
        assert_eq!(search.get_unassigned(v1).collect::<Vec<Val>>(), &[2,3]);
        assert_eq!(search.get_unassigned(v2).collect::<Vec<Val>>(), &[2,3]);
    }

    #[test]
    fn test_contradiction_by_length() {
        let mut puzzle = Puzzle::new();
        let v0 = puzzle.new_var_with_candidates(&[1,2]);
        let v1 = puzzle.new_var_with_candidates(&[1,2]);
        let v2 = puzzle.new_var_with_candidates(&[1,2]);

        puzzle.all_different(&[v0,v1,v2]);

        let search = puzzle.step();
        assert!(search.is_none());
    }

    #[test]
    fn test_constrain_by_value() {
        let mut puzzle = Puzzle::new();
        let v0 = puzzle.new_var_with_candidates(&[1,2]);
        let v1 = puzzle.new_var_with_candidates(&[1,2]);
        let v2 = puzzle.new_var_with_candidates(&[1,2,3]);

        puzzle.all_different(&[v0,v1,v2]);

        let search = puzzle.step().expect("contradiction");
        assert_eq!(search.get_unassigned(v0).collect::<Vec<Val>>(), &[1,2]);
        assert_eq!(search.get_unassigned(v1).collect::<Vec<Val>>(), &[1,2]);
        assert_eq!(search[v2], 3);
    }
}
