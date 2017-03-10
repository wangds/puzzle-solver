//! N-queens problem.
//!
//! https://en.wikipedia.org/wiki/Eight_queens_puzzle

extern crate puzzle_solver;

use std::rc::Rc;
use puzzle_solver::*;

struct NoDiagonal {
    vars: Vec<VarToken>,
}

impl Constraint for NoDiagonal {
    fn vars<'a>(&'a self) -> Box<Iterator<Item=&'a VarToken> + 'a> {
        Box::new(self.vars.iter())
    }

    fn on_assigned(&self, search: &mut PuzzleSearch, var: VarToken, val: Val)
            -> PsResult<()> {
        let y1 = self.vars.iter().position(|&v| v == var).expect("unreachable");
        for (y2, &var2) in self.vars.iter().enumerate() {
            if !search.is_assigned(var2) {
                let x1 = val;
                let dy = (y1 as Val) - (y2 as Val);
                try!(search.remove_candidate(var2, x1 - dy));
                try!(search.remove_candidate(var2, x1 + dy));
            }
        }

        Ok(())
    }

    fn substitute(&self, _from: VarToken, _to: VarToken)
            -> PsResult<Rc<Constraint>> {
        unimplemented!();
    }
}

fn make_queens(n: usize) -> (Puzzle, Vec<VarToken>) {
    let mut sys = Puzzle::new();
    let pos: Vec<Val> = (0..n as Val).collect();
    let vars = sys.new_vars_with_candidates_1d(n, &pos);

    sys.all_different(&vars);
    sys.add_constraint(NoDiagonal{ vars: vars.clone() });
    (sys, vars)
}

fn print_queens(dict: &Solution, vars: &Vec<VarToken>) {
    let n = vars.len() as Val;
    for &var in vars.iter() {
        for i in 0..n {
            print!(" {}", if i == dict[var] { "Q" } else { "." });
        }
        println!();
    }
}

#[test]
fn queens_4x4() {
    let (mut sys, vars) = make_queens(4);
    let dict = sys.solve_all();
    assert_eq!(dict.len(), 2);
    print_queens(&dict[0], &vars);
    println!("queens_4x4: {} guesses", sys.num_guesses());
}

#[test]
fn queens_5x5() {
    let (mut sys, vars) = make_queens(5);
    let dict = sys.solve_all();
    assert_eq!(dict.len(), 10);
    print_queens(&dict[0], &vars);
    println!("queens_5x5: {} guesses", sys.num_guesses());
}

#[test]
fn queens_6x6() {
    let (mut sys, vars) = make_queens(6);
    let dict = sys.solve_all();
    assert_eq!(dict.len(), 4);
    print_queens(&dict[0], &vars);
    println!("queens_6x6: {} guesses", sys.num_guesses());
}

#[test]
fn queens_7x7() {
    let (mut sys, vars) = make_queens(7);
    let dict = sys.solve_all();
    assert_eq!(dict.len(), 40);
    print_queens(&dict[0], &vars);
    println!("queens_7x7: {} guesses", sys.num_guesses());
}

#[test]
fn queens_8x8() {
    let (mut sys, vars) = make_queens(8);
    let dict = sys.solve_all();
    assert_eq!(dict.len(), 92);
    print_queens(&dict[0], &vars);
    println!("queens_8x8: {} guesses", sys.num_guesses());
}
