//! Killer Sudoku.
//!
//! https://en.wikipedia.org/wiki/Killer_sudoku

extern crate puzzle_solver;

use puzzle_solver::{LinExpr,Puzzle,Solution,Val,VarToken};

const SQRT_SIZE: usize = 3;
const SIZE: usize = 9;
type Board = [[Val; SIZE]; SIZE];
type Point = (usize, usize);

fn make_killer_sudoku(board: &[(Val, Vec<Point>)]) -> (Puzzle, Vec<Vec<VarToken>>) {
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

    for &(total, ref points) in board.iter() {
        sys.equals(total, points.iter().fold(LinExpr::from(0), |sum, &(x,y)| sum + vars[y][x]));
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
fn killersudoku_wikipedia() {
    let puzzle = [
        ( 3, vec![(0,0), (1,0)]),
        (15, vec![(2,0), (3,0), (4,0)]),
        (22, vec![(5,0), (4,1), (5,1), (4,2)]),
        ( 4, vec![(6,0), (6,1)]),
        (16, vec![(7,0), (7,1)]),
        (15, vec![(8,0), (8,1), (8,2), (8,3)]),
        (25, vec![(0,1), (1,1), (0,2), (1,2)]),
        (17, vec![(2,1), (3,1)]),
        ( 9, vec![(2,2), (3,2), (3,3)]),
        ( 8, vec![(5,2), (5,3), (5,4)]),
        (20, vec![(6,2), (7,2), (6,3)]),
        ( 6, vec![(0,3), (0,4)]),
        (14, vec![(1,3), (2,3)]),
        (17, vec![(4,3), (4,4), (4,5)]),
        (17, vec![(7,3), (6,4), (7,4)]),
        (13, vec![(1,4), (2,4), (1,5)]),
        (20, vec![(3,4), (3,5), (3,6)]),
        (12, vec![(8,4), (8,5)]),
        (27, vec![(0,5), (0,6), (0,7), (0,8)]),
        ( 6, vec![(2,5), (1,6), (2,6)]),
        (20, vec![(5,5), (5,6), (6,6)]),
        ( 6, vec![(6,5), (7,5)]),
        (10, vec![(4,6), (3,7), (4,7), (3,8)]),
        (14, vec![(7,6), (8,6), (7,7), (8,7)]),
        ( 8, vec![(1,7), (1,8)]),
        (16, vec![(2,7), (2,8)]),
        (15, vec![(5,7), (6,7)]),
        (13, vec![(4,8), (5,8), (6,8)]),
        (17, vec![(7,8), (8,8)]),
    ];

    let expected = [
        [ 2,1,5,  6,4,7,  3,9,8 ],
        [ 3,6,8,  9,5,2,  1,7,4 ],
        [ 7,9,4,  3,8,1,  6,5,2 ],

        [ 5,8,6,  2,7,4,  9,3,1 ],
        [ 1,4,2,  5,9,3,  8,6,7 ],
        [ 9,7,3,  8,1,6,  4,2,5 ],

        [ 8,2,1,  7,3,9,  5,4,6 ],
        [ 6,5,9,  4,2,8,  7,1,3 ],
        [ 4,3,7,  1,6,5,  2,8,9 ] ];

    let (mut sys, vars) = make_killer_sudoku(&puzzle);
    let dict = sys.solve_any().expect("solution");
    print_sudoku(&dict, &vars);
    verify_sudoku(&dict, &vars, &expected);
    println!("killersudoku_wikipedia: {} guesses", sys.num_guesses());
}
