//! The puzzle's state and rules.

use std::cell::Cell;
use std::collections::BTreeSet;
use std::iter;
use std::mem;
use std::ops::Index;
use std::rc::Rc;
use bit_set::BitSet;

use ::{Constraint,Solution,Val,VarToken};
use constraint;

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

    // The number of guesses to solve the puzzle.
    num_guesses: Cell<u32>,

    // The list of candidates for each variable.
    candidates: Vec<Candidates>,

    // The list of puzzle constraints.
    constraints: Vec<Box<Constraint>>,

    // The list of constraints that each variable belongs to.  These
    // will be woken up when the variable's candidates are changed.
    wake: Vec<BitSet>,
}

/// Intermediate puzzle search state.
#[derive(Clone)]
pub struct PuzzleSearch<'a> {
    puzzle: &'a Puzzle,
    vars: Vec<VarState>,

    // The list of constraints that need to be re-evaluated.
    wake: BitSet,
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
            num_guesses: Cell::new(0),
            candidates: Vec::new(),
            constraints: Vec::new(),
            wake: Vec::new(),
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
        self.wake.push(BitSet::new());
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
        let cidx = self.constraints.len();
        for &VarToken(idx) in constraint.vars() {
            self.wake[idx].insert(cidx);
        }

        self.constraints.push(constraint);
    }

    /// Add an All Different constraint.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut send_more_money = puzzle_solver::Puzzle::new();
    /// let vars = send_more_money.new_vars_with_candidates_1d(8,
    ///         &[0,1,2,3,4,5,6,7,8,9]);
    ///
    /// send_more_money.all_different(&vars);
    /// ```
    pub fn all_different<'a, I>(&mut self, vars: I)
            where I: IntoIterator<Item=&'a VarToken> {
        self.add_constraint(Box::new(constraint::AllDifferent::new(vars)));
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
    pub fn solve_any(&mut self) -> Option<Solution> {
        let mut solutions = Vec::with_capacity(1);

        self.num_guesses.set(0);
        if self.num_vars > 0 {
            let mut search = PuzzleSearch::new(self);
            search.solve(1, &mut solutions);
        }

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
    pub fn solve_unique(&mut self) -> Option<Solution> {
        self.num_guesses.set(0);
        if self.num_vars > 0 {
            let mut search = PuzzleSearch::new(self);
            let mut solutions = Vec::with_capacity(2);
            search.solve(2, &mut solutions);
            if solutions.len() == 1 {
                return solutions.pop();
            }
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
    pub fn solve_all(&mut self) -> Vec<Solution> {
        let mut solutions = Vec::new();

        self.num_guesses.set(0);
        if self.num_vars > 0 {
            let mut search = PuzzleSearch::new(self);
            search.solve(::std::usize::MAX, &mut solutions);
        }

        solutions
    }

    /// Take any obvious non-choices, using the constraints to
    /// eliminate candidates.  Stops when it must start guessing.
    /// Primarily for testing.
    ///
    /// Returns the intermediate puzzle search state, or None if a
    /// contradiction was found.
    pub fn step(&mut self) -> Option<PuzzleSearch> {
        if self.num_vars > 0 {
            let mut search = PuzzleSearch::new(self);
            if search.constrain() {
                return Some(search);
            }
        }

        None
    }

    /// Get the number of guesses taken to solve the last puzzle.
    pub fn num_guesses(&self) -> u32 {
        self.num_guesses.get()
    }
}

/*--------------------------------------------------------------*/

impl<'a> PuzzleSearch<'a> {
    /// Allocate a new puzzle searcher.
    fn new(puzzle: &'a Puzzle) -> Self {
        let mut vars = Vec::with_capacity(puzzle.num_vars);
        let mut wake = BitSet::new();

        for c in puzzle.candidates.iter() {
            vars.push(VarState::Unassigned(c.clone()));
        }

        for cidx in 0..puzzle.constraints.len() {
            wake.insert(cidx);
        }

        PuzzleSearch {
            puzzle: puzzle,
            vars: vars,
            wake: wake,
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

    /// Get an iterator over the candidates to an unassigned variable.
    pub fn get_unassigned(&'a self, var: VarToken)
            -> Box<Iterator<Item=Val> + 'a> {
        let VarToken(idx) = var;
        match &self.vars[idx] {
            &VarState::Assigned(_) => Box::new(iter::empty()),
            &VarState::Unassigned(ref cs) => cs.iter(),
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
                        self.wake.union_with(&self.puzzle.wake[idx]);
                    }
                },
                &mut Candidates::Set(ref mut rc) => {
                    if rc.contains(&val) {
                        let mut set = Rc::make_mut(rc);
                        set.remove(&val);
                        self.wake.union_with(&self.puzzle.wake[idx]);
                    }
                },
            }
        }
    }

    /// Solve the puzzle, finding up to count solutions.
    fn solve(&mut self, count: usize, solutions: &mut Vec<Solution>) {
        if !self.constrain() {
            return;
        }

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
                let num_guesses = self.puzzle.num_guesses.get() + 1;
                self.puzzle.num_guesses.set(num_guesses);

                let mut new = self.clone();
                if !new.assign(idx, val) {
                    continue;
                }

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

    /// Assign a variable (given by index) to a value.
    ///
    /// Returns false if a contradiction was found.
    fn assign(&mut self, idx: usize, val: Val) -> bool {
        let var = VarToken(idx);
        self.vars[idx] = VarState::Assigned(val);
        self.wake.union_with(&self.puzzle.wake[idx]);

        for (cidx, constraint) in self.puzzle.constraints.iter().enumerate() {
            if self.puzzle.wake[idx].contains(cidx) {
                if !constraint.on_assigned(self, var, val) {
                    return false;
                }
            }
        }

        true
    }

    /// Take any obvious non-choices, using the constraints to
    /// eliminate candidates.  Stops when it must start guessing.
    ///
    /// Returns false if a contradiction was found.
    fn constrain(&mut self) -> bool {
        while !self.wake.is_empty() {
            // "Gimme" phase:
            // - abort if any variables with 0 candidates,
            // - assign variables with only 1 candidate.
            // - repeat until no more gimmes found.
            let cycle = self.vars.len();
            let mut idx = 0;
            let mut last_gimme = cycle - 1;
            loop {
                let gimme = match &self.vars[idx] {
                    &VarState::Assigned(_) => None,
                    &VarState::Unassigned(ref cs) => match cs.len() {
                        0 => return false,
                        1 => cs.iter().next(),
                        _ => None,
                    }
                };

                if let Some(val) = gimme {
                    if !self.assign(idx, val) {
                        return false;
                    }
                    last_gimme = idx;
                } else if idx == last_gimme {
                    break;
                }

                idx = if idx + 1 >= cycle { 0 } else { idx + 1 };
            }

            // Apply constraints.
            if !self.wake.is_empty() {
                let wake = mem::replace(&mut self.wake, BitSet::new());
                for cidx in wake.iter() {
                    if !self.puzzle.constraints[cidx].on_updated(self) {
                        return false;
                    }
                }
            }
        }

        true
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

#[cfg(test)]
mod tests {
    use ::Puzzle;

    #[test]
    fn test_no_vars() {
        let mut sys = Puzzle::new();
        sys.solve_any();
        sys.solve_unique();
        sys.solve_all();
        sys.step();
    }
}
