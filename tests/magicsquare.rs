//! Magic Square.
//!
//! https://en.wikipedia.org/wiki/Magic_square

extern crate puzzle_solver;

use puzzle_solver::{LinExpr,Puzzle,Solution,Val,VarToken};

fn make_magic_square(n: usize) -> (Puzzle, Vec<Vec<VarToken>>, VarToken) {
    let mut sys = Puzzle::new();
    let digits: Vec<Val> = (1..(n * n + 1) as Val).collect();
    let vars = sys.new_vars_with_candidates_2d(n, n, &digits);

    // Calculate the range of the total (in a dumb way).
    let min = digits.iter().take(n).sum();
    let max = digits.iter().rev().take(n).sum();
    let total = sys.new_var_with_candidates(&(min..max).collect::<Vec<Val>>());

    sys.all_different(vars.iter().flat_map(|it| it));

    for y in 0..n {
        sys.equals(total, vars[y].iter().fold(LinExpr::from(0), |sum, &x| sum + x));
    }

    for x in 0..n {
        sys.equals(total, vars.iter().fold(LinExpr::from(0), |sum, row| sum + row[x]));
    }

    {
        sys.equals(total, (0..n).fold(LinExpr::from(0), |sum, i| sum + vars[i][i]));
        sys.equals(total, (0..n).fold(LinExpr::from(0), |sum, i| sum + vars[i][n - i - 1]));
    }

    // Sum of all digits = sum of all rows (columns) = total * n.
    sys.equals(total * (n as Val), digits.iter().sum::<Val>());

    (sys, vars, total)
}

fn print_magic_square(dict: &Solution, vars: &Vec<Vec<VarToken>>) {
    for row in vars.iter() {
        for &var in row.iter() {
            print!(" {:2}", dict[var]);
        }
        println!();
    }
}

#[test]
fn magicsquare_3x3() {
    let (mut sys, vars, total) = make_magic_square(3);
    let solutions = sys.solve_all();
    assert_eq!(solutions.len(), 8);

    print_magic_square(&solutions[0], &vars);
    for dict in solutions.iter() {
        assert_eq!(dict[total], 15);
    }
    println!("magicsquare_3x3: {} guesses", sys.num_guesses());
}

#[test]
fn magicsquare_4x4() {
    let (mut sys, vars, total) = make_magic_square(4);
    let dict = sys.solve_any().expect("solution");
    print_magic_square(&dict, &vars);
    assert_eq!(dict[total], 34);
}
