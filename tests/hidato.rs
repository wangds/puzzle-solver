//! Hidato.
//!
//! https://en.wikipedia.org/wiki/Hidato

extern crate puzzle_solver;

use puzzle_solver::{Puzzle,Solution,Val,VarToken};

const WIDTH: usize = 8;
const HEIGHT: usize = 8;
const NA: i32 = -1;
type Board = [[i32; WIDTH]; HEIGHT];

fn make_hidato(board: &Board) -> (Puzzle, Vec<VarToken>) {
    let mut sys = Puzzle::new();
    let mut pos = Vec::new();
    let mut count = 0;

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if board[y][x] != NA {
                pos.push((WIDTH * y + x) as Val);
                count = count + 1;
            }
        }
    }

    let vars = sys.new_vars_with_candidates_1d(count, &pos);

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if board[y][x] > 0 {
                let idx = (board[y][x] - 1) as usize;
                sys.set_value(vars[idx], (WIDTH * y + x) as Val);
            }
        }
    }

    sys.all_different(&vars);

    let stride = WIDTH as Val;
    let deltas = [
        -stride - 1, -stride, -stride + 1,
        -1, 1,
        stride - 1, stride, stride + 1 ];

    for i in 1..vars.len() {
        let step = sys.new_var_with_candidates(&deltas);
        sys.equals(vars[i], vars[i - 1] + step);
    }

    (sys, vars)
}

fn print_hidato(dict: &Solution, vars: &Vec<VarToken>) {
    let mut board = [[NA; WIDTH]; HEIGHT];

    for (idx, &var) in vars.iter().enumerate() {
        let x = (dict[var] as usize) % WIDTH;
        let y = (dict[var] as usize) / WIDTH;
        board[y][x] = (idx as i32) + 1;
    }

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            if board[y][x] == NA {
                print!(" --");
            } else {
                print!(" {:2}", board[y][x]);
            }
        }
        println!();
    }
}

fn verify_hidato(dict: &Solution, vars: &Vec<VarToken>, expected: &Board) {
    for (idx, &var) in vars.iter().enumerate() {
        let x = (dict[var] as usize) % WIDTH;
        let y = (dict[var] as usize) / WIDTH;
        assert_eq!((idx as i32) + 1, expected[y][x]);
    }
}

#[test]
fn hidato_wikipedia() {
    let puzzle = [
        [  0, 33, 35,  0,  0, NA, NA, NA ],
        [  0,  0, 24, 22,  0, NA, NA, NA ],
        [  0,  0,  0, 21,  0,  0, NA, NA ],
        [  0, 26,  0, 13, 40, 11, NA, NA ],
        [ 27,  0,  0,  0,  9,  0,  1, NA ],
        [ NA, NA,  0,  0, 18,  0,  0, NA ],
        [ NA, NA, NA, NA,  0,  7,  0,  0 ],
        [ NA, NA, NA, NA, NA, NA,  5,  0 ] ];

    let expected = [
        [ 32, 33, 35, 36, 37, NA, NA, NA ],
        [ 31, 34, 24, 22, 38, NA, NA, NA ],
        [ 30, 25, 23, 21, 12, 39, NA, NA ],
        [ 29, 26, 20, 13, 40, 11, NA, NA ],
        [ 27, 28, 14, 19,  9, 10,  1, NA ],
        [ NA, NA, 15, 16, 18,  8,  2, NA ],
        [ NA, NA, NA, NA, 17,  7,  6,  3 ],
        [ NA, NA, NA, NA, NA, NA,  5,  4 ] ];

    let (mut sys, vars) = make_hidato(&puzzle);
    let dict = sys.solve_any().expect("solution");
    print_hidato(&dict, &vars);
    verify_hidato(&dict, &vars, &expected);
    println!("hidato_wikipedia: {} guesses", sys.num_guesses());
}
