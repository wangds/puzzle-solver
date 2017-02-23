//! Sudoku.
//!
//! https://en.wikipedia.org/wiki/Sudoku

extern crate puzzle_solver;

use puzzle_solver::{Puzzle,Solution,Val,VarToken};

const SQRT_SIZE: usize = 3;
const SIZE: usize = 9;
type Board = [[Val; SIZE]; SIZE];

fn make_sudoku(board: &Board) -> (Puzzle, Vec<Vec<VarToken>>) {
    let mut sys = Puzzle::new();
    let vars = sys.new_vars_with_candidates_2d(SIZE, SIZE, &[1,2,3,4,5,6,7,8,9]);

    for y in 0..SIZE {
        sys.all_different(&vars[y]);
    }

    for x in 0..SIZE {
        sys.all_different(vars.iter().map(|row| &row[x]));
    }

    for block in 0..SIZE {
        let x0 = SQRT_SIZE * (block % SQRT_SIZE);
        let y0 = SQRT_SIZE * (block / SQRT_SIZE);
        sys.all_different((0..SIZE).map(|n|
                &vars[y0 + (n / SQRT_SIZE)][x0 + (n % SQRT_SIZE)]));
    }

    for y in 0..SIZE {
        for x in 0..SIZE {
            let value = board[y][x];
            if value != 0 {
                sys.set_value(vars[y][x], value);
            }
        }
    }

    (sys, vars)
}

fn print_sudoku(dict: &Solution, vars: &Vec<Vec<VarToken>>) {
    for y in 0..SIZE {
        if y % SQRT_SIZE == 0 {
            println!();
        }

        for x in 0..SIZE {
            print!("{}{}",
                   if x % SQRT_SIZE == 0 { " " } else { "" },
                   dict[vars[y][x]]);
        }
        println!();
    }
}

fn verify_sudoku(dict: &Solution, vars: &Vec<Vec<VarToken>>, expected: &Board) {
    for y in 0..SIZE {
        for x in 0..SIZE {
            assert_eq!(dict[vars[y][x]], expected[y][x]);
        }
    }
}

#[test]
fn sudoku_hardest() {
    let puzzle = [
        [ 8,0,0,  0,0,0,  0,0,0 ],
        [ 0,0,3,  6,0,0,  0,0,0 ],
        [ 0,7,0,  0,9,0,  2,0,0 ],

        [ 0,5,0,  0,0,7,  0,0,0 ],
        [ 0,0,0,  0,4,5,  7,0,0 ],
        [ 0,0,0,  1,0,0,  0,3,0 ],

        [ 0,0,1,  0,0,0,  0,6,8 ],
        [ 0,0,8,  5,0,0,  0,1,0 ],
        [ 0,9,0,  0,0,0,  4,0,0 ] ];

    let expected = [
        [ 8,1,2,  7,5,3,  6,4,9 ],
        [ 9,4,3,  6,8,2,  1,7,5 ],
        [ 6,7,5,  4,9,1,  2,8,3 ],

        [ 1,5,4,  2,3,7,  8,9,6 ],
        [ 3,6,9,  8,4,5,  7,2,1 ],
        [ 2,8,7,  1,6,9,  5,3,4 ],

        [ 5,2,1,  9,7,4,  3,6,8 ],
        [ 4,3,8,  5,2,6,  9,1,7 ],
        [ 7,9,6,  3,1,8,  4,5,2 ] ];

    let (mut sys, vars) = make_sudoku(&puzzle);
    let solution = sys.solve_any().expect("solution");
    print_sudoku(&solution, &vars);
    verify_sudoku(&solution, &vars, &expected);
    println!("sudoku_hardest: {} guesses", sys.num_guesses());
}

#[test]
fn sudoku_wikipedia() {
    let puzzle = [
        [ 5,3,0,  0,7,0,  0,0,0 ],
        [ 6,0,0,  1,9,5,  0,0,0 ],
        [ 0,9,8,  0,0,0,  0,6,0 ],

        [ 8,0,0,  0,6,0,  0,0,3 ],
        [ 4,0,0,  8,0,3,  0,0,1 ],
        [ 7,0,0,  0,2,0,  0,0,6 ],

        [ 0,6,0,  0,0,0,  2,8,0 ],
        [ 0,0,0,  4,1,9,  0,0,5 ],
        [ 0,0,0,  0,8,0,  0,7,9 ] ];

    let expected = [
        [ 5,3,4,  6,7,8,  9,1,2 ],
        [ 6,7,2,  1,9,5,  3,4,8 ],
        [ 1,9,8,  3,4,2,  5,6,7 ],

        [ 8,5,9,  7,6,1,  4,2,3 ],
        [ 4,2,6,  8,5,3,  7,9,1 ],
        [ 7,1,3,  9,2,4,  8,5,6 ],

        [ 9,6,1,  5,3,7,  2,8,4 ],
        [ 2,8,7,  4,1,9,  6,3,5 ],
        [ 3,4,5,  2,8,6,  1,7,9 ] ];

    let (mut sys, vars) = make_sudoku(&puzzle);
    let solution = sys.solve_any().expect("solution");
    print_sudoku(&solution, &vars);
    verify_sudoku(&solution, &vars, &expected);
    println!("sudoku_wikipedia: {} guesses", sys.num_guesses());
}
