//! The puzzle's state and rules.

use std::collections::BTreeSet;
use std::iter;
use std::iter::Iterator;
use std::ops::Index;
use std::rc::Rc;

use ::{Constraint,Solution,Val,VarToken};

/// A collection of candidates.
#[derive(Clone,Debug,Eq,PartialEq)]
enum Candidates {
    None,                       // A variable with no candidates.
    Value(Val),                 // A variable set to its initial value.
    Set(Rc<BTreeSet<Val>>),     // A variable with a list of candidates.
}

/// The state of a variable during the solution search.
#[derive(Clone,Debug)]
enum VarState {
    Assigned(Val),
    Unassigned(Candidates),
}

/// The puzzle to be solved.
pub struct Puzzle {
    // The number of variables in the puzzle.
    num_vars: usize,

    // The list of candidates for each variable.
    candidates: Vec<Candidates>,

    // The list of puzzle constraints.
    constraints: Vec<Box<Constraint>>,
}

/// Intermediate puzzle search state.
#[derive(Clone)]
pub struct PuzzleSearch<'a> {
    puzzle: &'a Puzzle,
    vars: Vec<VarState>,
}

/*--------------------------------------------------------------*/

impl Candidates {
    /// Count the number of candidates for a variable.
    fn len(&self) -> usize {
        match self {
            &Candidates::None => 0,
            &Candidates::Value(_) => 1,
            &Candidates::Set(ref rc) => rc.len(),
        }
    }

    /// Get an iterator over all of the candidates of a variable.
    fn iter<'a>(&'a self) -> Box<Iterator<Item=Val> + 'a> {
        match self {
            &Candidates::None => Box::new(iter::empty()),
            &Candidates::Value(val) => Box::new(iter::once(val)),
            &Candidates::Set(ref rc) => Box::new(rc.iter().map(|x| *x)),
        }
    }
}

/*--------------------------------------------------------------*/

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
            candidates: Vec::new(),
            constraints: Vec::new(),
        }
    }

    /// Allocate a new puzzle variable, without inserting any
    /// candidates.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut puzzle = puzzle_solver::Puzzle::new();
    /// puzzle.new_var();
    /// ```
    pub fn new_var(&mut self) -> VarToken {
        let var = VarToken(self.num_vars);
        self.num_vars = self.num_vars + 1;
        self.candidates.push(Candidates::None);
        var
    }

    /// Allocate a new puzzle variable, initialising it with potential
    /// candidates.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut send_more_money = puzzle_solver::Puzzle::new();
    /// send_more_money.new_var_with_candidates(&[0,1,2,3,4,5,6,7,8,9]);
    /// ```
    pub fn new_var_with_candidates(&mut self, candidates: &[Val]) -> VarToken {
        let var = self.new_var();
        self.insert_candidates(var, candidates);
        var
    }

    /// Allocate a 1d vector of puzzle variables, each initialised to
    /// have the same set of potential candidates.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut send_more_money = puzzle_solver::Puzzle::new();
    /// send_more_money.new_vars_with_candidates_1d(8, &[0,1,2,3,4,5,6,7,8,9]);
    /// ```
    pub fn new_vars_with_candidates_1d(&mut self, n: usize, candidates: &[Val])
            -> Vec<VarToken> {
        let mut vars = Vec::with_capacity(n);
        for _ in 0..n {
            vars.push(self.new_var_with_candidates(candidates));
        }
        vars
    }

    /// Allocate a 2d array of puzzle variables, each initialised to
    /// have the same set of potential candidates.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut magic_square = puzzle_solver::Puzzle::new();
    /// magic_square.new_vars_with_candidates_2d(3, 3, &[1,2,3,4,5,6,7,8,9]);
    /// ```
    pub fn new_vars_with_candidates_2d(self: &mut Puzzle,
            width: usize, height: usize, candidates: &[Val])
            -> Vec<Vec<VarToken>> {
        let mut vars = Vec::with_capacity(height);
        for _ in 0..height {
            vars.push(self.new_vars_with_candidates_1d(width, candidates));
        }
        vars
    }

    /// Set a variable to a known value.
    ///
    /// This is useful when the variable is given as part of the
    /// problem.  After this operation, any subsequent attempts to set
    /// the value will panic.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut magic_square = puzzle_solver::Puzzle::new();
    /// let vars = magic_square.new_vars_with_candidates_2d(3, 3,
    ///         &[1,2,3,4,5,6,7,8,9]);
    ///
    /// magic_square.set_value(vars[1][1], 5);
    /// ```
    pub fn set_value(&mut self, var: VarToken, value: Val) {
        let VarToken(idx) = var;

        if let Candidates::Value(val) = self.candidates[idx] {
            if val != value {
                panic!("attempt to set fixed variable");
            }
        }

        self.candidates[idx] = Candidates::Value(value);
    }

    /// Add candidates to a variable.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut send_more_money = puzzle_solver::Puzzle::new();
    /// for _ in 0..9 {
    ///     let var = send_more_money.new_var();
    ///     send_more_money.insert_candidates(var, &[0,1,2,3,4,5,6,7,8,9]);
    /// }
    /// ```
    pub fn insert_candidates(&mut self, var: VarToken, candidates: &[Val]) {
        let VarToken(idx) = var;

        match &self.candidates[idx] {
            &Candidates::Value(_) =>
                panic!("attempt to set fixed variable"),

            &Candidates::None => {
                self.candidates[idx] = Candidates::Set(Rc::new(BTreeSet::new()));
            },

            &Candidates::Set(_) => (),
        }

        // Why you dumb Rust?
        if let Candidates::Set(ref mut rc) = self.candidates[idx] {
            let cs = Rc::get_mut(rc).expect("unique");
            cs.extend(candidates);
        }
    }

    /// Remove candidates from a variable.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut send_more_money = puzzle_solver::Puzzle::new();
    /// let vars = send_more_money.new_vars_with_candidates_1d(8,
    ///         &[0,1,2,3,4,5,6,7,8,9]);
    ///
    /// let s = vars[0];
    /// let m = vars[4];
    /// send_more_money.remove_candidates(s, &[0]);
    /// send_more_money.remove_candidates(m, &[0]);
    /// ```
    pub fn remove_candidates(&mut self, var: VarToken, candidates: &[Val]) {
        let VarToken(idx) = var;

        match self.candidates[idx] {
            Candidates::Value(_) =>
                panic!("attempt to set fixed variable"),

            Candidates::None => (),

            Candidates::Set(ref mut rc) => {
                let cs = Rc::get_mut(rc).expect("unique");
                for c in candidates.iter() {
                    cs.remove(c);
                }
            },
        }
    }

    /// Set the variable's candidates to the intersection with the
    /// given list.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut send_more_money = puzzle_solver::Puzzle::new();
    /// let vars = send_more_money.new_vars_with_candidates_1d(8,
    ///         &[0,1,2,3,4,5,6,7,8,9]);
    ///
    /// let m = vars[4];
    /// send_more_money.intersect_candidates(m, &[0,1]);
    /// ```
    pub fn intersect_candidates(&mut self, var: VarToken, candidates: &[Val]) {
        let VarToken(idx) = var;

        match self.candidates[idx] {
            Candidates::Value(_) =>
                panic!("attempt to set fixed variable"),

            Candidates::None => (),

            Candidates::Set(ref mut rc) => {
                let cs = Rc::get_mut(rc).expect("unique");
                let mut set = BTreeSet::new();
                set.extend(candidates);
                *cs = cs.intersection(&set).cloned().collect();
            },
        }
    }

    /// Add a constraint to the puzzle solution.
    pub fn add_constraint(&mut self, constraint: Box<Constraint>) {
        self.constraints.push(constraint);
    }

    /// Find any solution to the given puzzle.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut puzzle = puzzle_solver::Puzzle::new();
    /// puzzle.new_var_with_candidates(&[1,2]);
    /// puzzle.new_var_with_candidates(&[3,4]);
    ///
    /// let solution = puzzle.solve_any();
    /// assert!(solution.is_some());
    /// ```
    pub fn solve_any(&self) -> Option<Solution> {
        let mut search = PuzzleSearch::new(self);
        let mut solutions = Vec::with_capacity(1);
        search.solve(1, &mut solutions);
        solutions.pop()
    }

    /// Find the solution to the given puzzle, verifying that it is
    /// unique.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut puzzle = puzzle_solver::Puzzle::new();
    /// puzzle.new_var_with_candidates(&[1,2]);
    /// puzzle.new_var_with_candidates(&[3,4]);
    ///
    /// let solution = puzzle.solve_unique();
    /// assert!(solution.is_none());
    /// ```
    pub fn solve_unique(&self) -> Option<Solution> {
        let mut search = PuzzleSearch::new(self);
        let mut solutions = Vec::with_capacity(2);
        search.solve(2, &mut solutions);
        if solutions.len() == 1 {
            return solutions.pop();
        }

        None
    }

    /// Find all solutions to the given puzzle.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut puzzle = puzzle_solver::Puzzle::new();
    /// puzzle.new_var_with_candidates(&[1,2]);
    /// puzzle.new_var_with_candidates(&[3,4]);
    ///
    /// let solutions = puzzle.solve_all();
    /// assert_eq!(solutions.len(), 4);
    /// ```
    pub fn solve_all(&self) -> Vec<Solution> {
        let mut search = PuzzleSearch::new(self);
        let mut solutions = Vec::new();
        search.solve(::std::usize::MAX, &mut solutions);
        solutions
    }
}

/*--------------------------------------------------------------*/

impl<'a> PuzzleSearch<'a> {
    /// Allocate a new puzzle searcher.
    fn new(puzzle: &'a Puzzle) -> Self {
        let mut vars = Vec::with_capacity(puzzle.num_vars);
        for c in puzzle.candidates.iter() {
            vars.push(VarState::Unassigned(c.clone()));
        }

        PuzzleSearch {
            puzzle: puzzle,
            vars: vars,
        }
    }

    /// Check if the variable has been assigned to a value.
    pub fn is_assigned(&self, var: VarToken) -> bool {
        let VarToken(idx) = var;
        match &self.vars[idx] {
            &VarState::Assigned(_) => true,
            &VarState::Unassigned(_) => false,
        }
    }

    /// Get the value assigned to a variable, or None.
    ///
    /// This should be used if the variable may potentially be
    /// unassigned.  For example, when implementing constraints.
    pub fn get_assigned(&self, var: VarToken) -> Option<Val> {
        let VarToken(idx) = var;
        match &self.vars[idx] {
            &VarState::Assigned(val) => Some(val),
            &VarState::Unassigned(_) => None,
        }
    }

    /// Remove a single candidate from an unassigned variable.
    pub fn remove_candidate(&mut self, var: VarToken, val: Val) {
        let VarToken(idx) = var;
        if let VarState::Unassigned(ref mut cs) = self.vars[idx] {
            match cs {
                &mut Candidates::None => return,
                &mut Candidates::Value(v) => {
                    if v == val {
                        *cs = Candidates::None;
                    }
                },
                &mut Candidates::Set(ref mut rc) => {
                    if rc.contains(&val) {
                        let mut set = Rc::make_mut(rc);
                        set.remove(&val);
                    }
                },
            }
        }
    }

    /// Solve the puzzle, finding up to count solutions.
    fn solve(&mut self, count: usize, solutions: &mut Vec<Solution>) {
        let next_unassigned = self.vars.iter().enumerate().min_by_key(
                |&(_, vs)| match vs {
                    &VarState::Assigned(_) => ::std::usize::MAX,
                    &VarState::Unassigned(ref cs) => cs.len(),
                });

        if let Some((idx, &VarState::Unassigned(ref cs))) = next_unassigned {
            if cs.len() == 0 {
                // Contradiction.
                return;
            }

            for val in cs.iter() {
                let mut new = self.clone();
                new.vars[idx] = VarState::Assigned(val);

                new.solve(count, solutions);
                if solutions.len() >= count {
                    // Reached desired number of solutions.
                    return;
                }
            }
        } else {
            // No unassigned variables remaining.
            let mut vars = Vec::with_capacity(self.puzzle.num_vars);
            for var in self.vars.iter() {
                match var {
                    &VarState::Assigned(val) => vars.push(val),
                    &VarState::Unassigned(_) => unreachable!(),
                }
            }
            solutions.push(Solution{ vars: vars });
        }
    }
}

impl<'a> Index<VarToken> for PuzzleSearch<'a> {
    type Output = Val;

    /// Get the value assigned to a variable.
    ///
    /// # Panics
    ///
    /// Panics if the variable has not been assigned.
    fn index(&self, var: VarToken) -> &Val {
        let VarToken(idx) = var;
        match &self.vars[idx] {
            &VarState::Assigned(ref val) => val,
            &VarState::Unassigned(_) => panic!("unassigned"),
        }
    }
}
