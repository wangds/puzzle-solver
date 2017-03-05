//! Samurai Sudoku.
//!
//! https://en.wikipedia.org/wiki/Sudoku#Variants
//! http://www.samurai-sudoku.com/#ai

extern crate puzzle_solver;

use puzzle_solver::{Puzzle,Solution,VarToken};

const SQRT_SIZE: usize = 3;
const SIZE: usize = 9;
const X: i32 = -1;
type Board = [[i32; SIZE + SQRT_SIZE + SIZE]; SIZE + SQRT_SIZE + SIZE];
type SudokuVars = Vec<Vec<VarToken>>;
type SamuraiVars = (SudokuVars, SudokuVars, SudokuVars, SudokuVars, SudokuVars);

fn make_sudoku(sys: &mut Puzzle) -> SudokuVars {
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

    vars
}

fn make_samurai_sudoku(board: &Board) -> (Puzzle, SamuraiVars) {
    let set = |sys: &mut Puzzle, var, val| if val != 0 { sys.set_value(var, val) };

    let mut sys = Puzzle::new();
    let tl = make_sudoku(&mut sys);
    let tr = make_sudoku(&mut sys);
    let bl = make_sudoku(&mut sys);
    let br = make_sudoku(&mut sys);
    let mid = make_sudoku(&mut sys);

    for y in 0..SQRT_SIZE {
        for x in 0..SQRT_SIZE {
            sys.equals(mid[0 * SQRT_SIZE + y][0 * SQRT_SIZE + x], tl[2 * SQRT_SIZE + y][2 * SQRT_SIZE + x]);
            sys.equals(mid[0 * SQRT_SIZE + y][2 * SQRT_SIZE + x], tr[2 * SQRT_SIZE + y][0 * SQRT_SIZE + x]);
            sys.equals(mid[2 * SQRT_SIZE + y][0 * SQRT_SIZE + x], bl[0 * SQRT_SIZE + y][2 * SQRT_SIZE + x]);
            sys.equals(mid[2 * SQRT_SIZE + y][2 * SQRT_SIZE + x], br[0 * SQRT_SIZE + y][0 * SQRT_SIZE + x]);
        }
    }

    for y in 0..SIZE {
        for x in 0..SIZE {
            set(&mut sys, tl[y][x], board[y][x]);
            set(&mut sys, tr[y][x], board[y][SIZE + SQRT_SIZE + x]);
            set(&mut sys, bl[y][x], board[SIZE + SQRT_SIZE + y][x]);
            set(&mut sys, br[y][x], board[SIZE + SQRT_SIZE + y][SIZE + SQRT_SIZE + x]);
            set(&mut sys, mid[y][x], board[2 * SQRT_SIZE + y][2 * SQRT_SIZE + x]);
        }
    }

    (sys, (tl, tr, bl, br, mid))
}

fn print_samurai_sudoku(dict: &Solution, vars: &SamuraiVars) {
    let &(ref tl, ref tr, ref bl, ref br, ref mid) = vars;
    let pr3 = |a: &[VarToken], j| print!(" {}{}{}", dict[a[j]], dict[a[j + 1]], dict[a[j + 2]]);
    let pr9 = |a| { pr3(a, 0); pr3(a, 3); pr3(a, 6); };
    let gap = || print!("    ");

    for i in 0..SIZE {
        pr9(&tl[i]);
        if 2 * SQRT_SIZE <= i {
            pr3(&mid[i - 2 * SQRT_SIZE], 3);
        } else {
            gap();
        }
        pr9(&tr[i]);
        println!();
    }

    for i in SQRT_SIZE..(2 * SQRT_SIZE) {
        gap();
        gap();
        pr9(&mid[i]);
        println!();
    }

    for i in 0..SIZE {
        pr9(&bl[i]);
        if i < SQRT_SIZE {
            pr3(&mid[2 * SQRT_SIZE + i], 3);
        } else {
            gap();
        }
        pr9(&br[i]);
        println!();
    }
}

fn verify_samurai_sudoku(dict: &Solution, vars: &SamuraiVars, expected: &Board) {
    let &(ref tl, ref tr, ref bl, ref br, ref mid) = vars;
    for i in 0..SIZE {
        for j in 0..SIZE {
            assert_eq!(dict[tl[i][j]], expected[i][j]);
            assert_eq!(dict[tr[i][j]], expected[i][SIZE + SQRT_SIZE + j]);
            assert_eq!(dict[bl[i][j]], expected[SIZE + SQRT_SIZE + i][j]);
            assert_eq!(dict[br[i][j]], expected[SIZE + SQRT_SIZE + i][SIZE + SQRT_SIZE + j]);
            assert_eq!(dict[mid[i][j]], expected[2 * SQRT_SIZE + i][2 * SQRT_SIZE + j]);
        }
    }
}

#[test]
fn samuraisudoku_easy() {
    let puzzle = [
        [ 0,0,3,  0,0,0,  2,0,0,  X,X,X,  0,0,6,  0,0,0,  2,0,0 ],
        [ 0,2,0,  4,0,8,  0,3,0,  X,X,X,  0,3,0,  4,0,2,  0,8,0 ],
        [ 8,0,0,  0,9,0,  0,0,4,  X,X,X,  8,0,0,  0,1,0,  0,0,4 ],

        [ 0,5,0,  6,0,1,  0,2,0,  X,X,X,  0,2,0,  1,0,7,  0,9,0 ],
        [ 0,0,8,  0,0,0,  6,0,0,  X,X,X,  0,0,9,  0,0,0,  8,0,0 ],
        [ 0,7,0,  8,0,4,  0,1,0,  X,X,X,  0,8,0,  5,0,9,  0,4,0 ],

        [ 1,0,0,  0,7,0,  0,0,0,  0,0,0,  0,0,0,  0,7,0,  0,0,5 ],
        [ 0,4,0,  1,0,2,  0,0,0,  0,0,0,  0,0,0,  2,0,8,  0,7,0 ],
        [ 0,0,9,  0,0,0,  0,0,0,  0,6,0,  0,0,0,  0,0,0,  1,0,0 ],

        [ X,X,X,  X,X,X,  0,0,0,  5,0,1,  0,0,0,  X,X,X,  X,X,X ],
        [ X,X,X,  X,X,X,  0,0,9,  0,0,0,  6,0,0,  X,X,X,  X,X,X ],
        [ X,X,X,  X,X,X,  0,0,0,  3,0,6,  0,0,0,  X,X,X,  X,X,X ],

        [ 0,0,8,  0,0,0,  0,0,0,  0,7,0,  0,0,0,  0,0,0,  4,0,0 ],
        [ 0,4,0,  5,0,1,  0,0,0,  0,0,0,  0,0,0,  9,0,5,  0,7,0 ],
        [ 6,0,0,  0,2,0,  0,0,0,  0,0,0,  0,0,0,  0,6,0,  0,0,9 ],

        [ 0,9,0,  1,0,3,  0,7,0,  X,X,X,  0,7,0,  5,0,1,  0,9,0 ],
        [ 0,0,5,  0,0,0,  1,0,0,  X,X,X,  0,0,3,  0,0,0,  6,0,0 ],
        [ 0,1,0,  6,0,8,  0,9,0,  X,X,X,  0,2,0,  8,0,6,  0,1,0 ],

        [ 5,0,0,  0,7,0,  0,0,6,  X,X,X,  7,0,0,  0,2,0,  0,0,5 ],
        [ 0,2,0,  3,0,5,  0,1,0,  X,X,X,  0,9,0,  6,0,4,  0,3,0 ],
        [ 0,0,6,  0,0,0,  2,0,0,  X,X,X,  0,0,4,  0,0,0,  1,0,0 ] ];

    let expected = [
        [ 4,9,3,  7,1,5,  2,6,8,  X,X,X,  9,4,6,  8,3,5,  2,1,7 ],
        [ 5,2,7,  4,6,8,  9,3,1,  X,X,X,  1,3,7,  4,9,2,  5,8,6 ],
        [ 8,6,1,  2,9,3,  5,7,4,  X,X,X,  8,5,2,  7,1,6,  9,3,4 ],

        [ 9,5,4,  6,3,1,  8,2,7,  X,X,X,  5,2,4,  1,8,7,  6,9,3 ],
        [ 3,1,8,  9,2,7,  6,4,5,  X,X,X,  7,1,9,  6,4,3,  8,5,2 ],
        [ 2,7,6,  8,5,4,  3,1,9,  X,X,X,  6,8,3,  5,2,9,  7,4,1 ],

        [ 1,8,2,  3,7,9,  4,5,6,  7,1,3,  2,9,8,  3,7,1,  4,6,5 ],
        [ 6,4,5,  1,8,2,  7,9,3,  8,5,2,  4,6,1,  2,5,8,  3,7,9 ],
        [ 7,3,9,  5,4,6,  1,8,2,  9,6,4,  3,7,5,  9,6,4,  1,2,8 ],

        [ X,X,X,  X,X,X,  6,4,8,  5,9,1,  7,2,3,  X,X,X,  X,X,X ],
        [ X,X,X,  X,X,X,  3,1,9,  2,8,7,  6,5,4,  X,X,X,  X,X,X ],
        [ X,X,X,  X,X,X,  2,7,5,  3,4,6,  1,8,9,  X,X,X,  X,X,X ],

        [ 1,7,8,  9,3,6,  5,2,4,  1,7,8,  9,3,6,  2,8,7,  4,5,1 ],
        [ 2,4,3,  5,8,1,  9,6,7,  4,3,5,  8,1,2,  9,4,5,  3,7,6 ],
        [ 6,5,9,  4,2,7,  8,3,1,  6,2,9,  5,4,7,  1,6,3,  8,2,9 ],

        [ 8,9,2,  1,4,3,  6,7,5,  X,X,X,  6,7,8,  5,3,1,  2,9,4 ],
        [ 4,6,5,  7,9,2,  1,8,3,  X,X,X,  1,5,3,  4,9,2,  6,8,7 ],
        [ 3,1,7,  6,5,8,  4,9,2,  X,X,X,  4,2,9,  8,7,6,  5,1,3 ],

        [ 5,8,1,  2,7,9,  3,4,6,  X,X,X,  7,6,1,  3,2,8,  9,4,5 ],
        [ 9,2,4,  3,6,5,  7,1,8,  X,X,X,  2,9,5,  6,1,4,  7,3,8 ],
        [ 7,3,6,  8,1,4,  2,5,9,  X,X,X,  3,8,4,  7,5,9,  1,6,2 ] ];

    let (mut sys, vars) = make_samurai_sudoku(&puzzle);
    let dict = sys.solve_any().expect("solution");
    print_samurai_sudoku(&dict, &vars);
    verify_samurai_sudoku(&dict, &vars, &expected);
    println!("samuraisudoku_easy: {} guesses", sys.num_guesses());
}
