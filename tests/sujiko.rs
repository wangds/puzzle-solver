//! Sujiko.
//!
//! https://en.wikipedia.org/wiki/Sujiko
//! https://www.simetric.co.uk/sujiko/index.htm

extern crate puzzle_solver;

use puzzle_solver::{Puzzle,Solution,Val,VarToken};

const SIZE: usize = 3;
type Board = [[Val; SIZE]; SIZE];

fn make_sujiko(board: &Board, tl: Val, tr: Val, bl: Val, br: Val)
        -> (Puzzle, Vec<Vec<VarToken>>) {
    let mut sys = Puzzle::new();
    let vars = sys.new_vars_with_candidates_2d(3, 3, &[1,2,3,4,5,6,7,8,9]);

    sys.all_different(vars.iter().flat_map(|it| it));

    sys.equals(tl, vars[0][0] + vars[0][1] + vars[1][0] + vars[1][1]);
    sys.equals(tr, vars[0][1] + vars[0][2] + vars[1][1] + vars[1][2]);
    sys.equals(bl, vars[1][0] + vars[1][1] + vars[2][0] + vars[2][1]);
    sys.equals(br, vars[1][1] + vars[1][2] + vars[2][1] + vars[2][2]);

    sys.equals(tl + tr + bl + br - (1..(9 + 1)).sum::<Val>(),
            vars[0][1] + vars[1][0] + 3 * vars[1][1] + vars[1][2] + vars[2][1]);

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

fn print_sujiko(dict: &Solution, vars: &Vec<Vec<VarToken>>) {
    for y in 0..SIZE {
        for x in 0..SIZE {
            print!(" {}", dict[vars[y][x]]);
        }
        println!();
    }
}

fn verify_sujiko(dict: &Solution, vars: &Vec<Vec<VarToken>>, expected: &Board) {
    for y in 0..SIZE {
        for x in 0..SIZE {
            assert_eq!(dict[vars[y][x]], expected[y][x]);
        }
    }
}

#[test]
fn sujiko_simetric() {
    let puzzle   = [ [ 6,0,9 ], [ 0,0,0 ], [ 5,0,0 ] ];
    let expected = [ [ 6,2,9 ], [ 8,1,3 ], [ 5,4,7 ] ];

    let (mut sys, vars) = make_sujiko(&puzzle, 17, 15, 18, 15);
    let dict = sys.solve_unique().expect("solution");
    print_sujiko(&dict, &vars);
    verify_sujiko(&dict, &vars, &expected);
    println!("sujiko_simetric: {} guesses", sys.num_guesses());
}
