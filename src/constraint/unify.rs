//! Unify implementation.

use std::iter;
use std::rc::Rc;

use ::{Constraint,PsResult,PuzzleSearch,VarToken};

pub struct Unify {
    var1: VarToken,
    var2: VarToken,
}

impl Unify {
    /// Allocate a new Unify constraint.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut send_more_money = puzzle_solver::Puzzle::new();
    /// let carry = send_more_money.new_vars_with_candidates_1d(4, &[0,1]);
    /// let vars = send_more_money.new_vars_with_candidates_1d(8,
    ///         &[0,1,2,3,4,5,6,7,8,9]);
    ///
    /// let m = vars[4];
    /// puzzle_solver::constraint::Unify::new(m, carry[3]);
    /// ```
    pub fn new(var1: VarToken, var2: VarToken) -> Self {
        Unify {
            var1: var1,
            var2: var2,
        }
    }
}

impl Constraint for Unify {
    fn vars<'a>(&'a self) -> Box<Iterator<Item=&'a VarToken> + 'a> {
        if self.var1 != self.var2 {
            Box::new(iter::once(&self.var1).chain(iter::once(&self.var2)))
        } else {
            Box::new(iter::empty())
        }
    }

    fn on_updated(&self, search: &mut PuzzleSearch) -> PsResult<()> {
        if self.var1 != self.var2 {
            search.unify(self.var1, self.var2)
        } else {
            Ok(())
        }
    }

    fn substitute(&self, from: VarToken, to: VarToken)
            -> PsResult<Rc<Constraint>> {
        let var1 = if self.var1 == from { to } else { self.var1 };
        let var2 = if self.var2 == from { to } else { self.var2 };
        Ok(Rc::new(Unify{ var1: var1, var2: var2 }))
    }
}

#[cfg(test)]
mod tests {
    use ::Puzzle;
    use super::Unify;

    #[test]
    fn test_unify_alldifferent() {
        let mut puzzle = Puzzle::new();
        let v0 = puzzle.new_var_with_candidates(&[1,2]);
        let v1 = puzzle.new_var_with_candidates(&[1,2]);

        puzzle.all_different(&[v0, v1]);
        puzzle.add_constraint(Unify::new(v0, v1));

        let search = puzzle.step();
        assert!(search.is_none());
    }

    #[test]
    fn test_unify_equality() {
        let mut puzzle = Puzzle::new();
        let v0 = puzzle.new_var_with_candidates(&[1,2,3,4]);
        let v1 = puzzle.new_var_with_candidates(&[1,2,3,4]);
        let v2 = puzzle.new_var_with_candidates(&[1,2,3,4]);

        puzzle.equals(v0 + 2 * v1 + v2, 6);
        puzzle.add_constraint(Unify::new(v0, v1));

        let search = puzzle.step().expect("contradiction");
        assert_eq!(search[v0], 1);
        assert_eq!(search[v1], 1);
        assert_eq!(search[v2], 3);
    }

    #[test]
    fn test_unify_unify() {
        let mut puzzle = Puzzle::new();
        let v0 = puzzle.new_var_with_candidates(&[1]);
        let v1 = puzzle.new_var_with_candidates(&[1,2,3,4]);
        let v2 = puzzle.new_var_with_candidates(&[1,2,3,4]);

        puzzle.add_constraint(Unify::new(v0, v1));
        puzzle.add_constraint(Unify::new(v1, v2));

        let search = puzzle.step().expect("contradiction");
        assert_eq!(search[v0], 1);
        assert_eq!(search[v1], 1);
        assert_eq!(search[v2], 1);
    }
}
