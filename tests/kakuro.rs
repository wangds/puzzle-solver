//! Kakuro.
//!
//! https://en.wikipedia.org/wiki/Kakuro

extern crate puzzle_solver;

use puzzle_solver::{LinExpr,Puzzle,Solution,Val,VarToken};

const SIZE: usize = 7;
const X: Val = 0;
type Board = [[Val; SIZE]; SIZE];
type KakuroVars = Vec<Vec<Option<VarToken>>>;

enum Rule {
    H{ total: Val, y: usize, x1: usize, x2: usize },
    V{ total: Val, x: usize, y1: usize, y2: usize },
}

fn make_kakuro(board: &[Rule]) -> (Puzzle, KakuroVars) {
    let mut sys = Puzzle::new();
    let mut vars = vec![vec![None; SIZE]; SIZE];

    for rule in board.iter() {
        let (total, x1, y1, x2, y2) = match rule {
            &Rule::H{total, y, x1, x2} => (total, x1, y, x2, y),
            &Rule::V{total, x, y1, y2} => (total, x, y1, x, y2),
        };

        let mut vec = Vec::new();
        for y in y1..(y2 + 1) {
            for x in x1..(x2 + 1) {
                let var = vars[y][x].unwrap_or_else(|| {
                    let var = sys.new_var_with_candidates(&[1,2,3,4,5,6,7,8,9]);
                    vars[y][x] = Some(var);
                    var
                });

                vec.push(var);
            }
        }

        sys.all_different(&vec);
        sys.equals(total, vec.iter().fold(LinExpr::from(0), |sum, &var| sum + var));
    }

    (sys, vars)
}

fn print_kakuro(dict: &Solution, vars: &KakuroVars) {
    for y in 0..SIZE {
        for x in 0..SIZE {
            if let Some(var) = vars[y][x] {
                print!(" {}", dict[var]);
            } else {
                print!(" .");
            }
        }
        println!();
    }
}

fn verify_kakuro(dict: &Solution, vars: &KakuroVars, expected: &Board) {
    for y in 0..SIZE {
        for x in 0..SIZE {
            let val = vars[y][x].map_or(X, |var| dict[var]);
            assert_eq!(val, expected[y][x]);
        }
    }
}

#[test]
fn kakuro_wikipedia() {
    let puzzle = [
        Rule::H{ total:16, y:0, x1:0, x2:1 }, Rule::H{ total:24, y:0, x1:4, x2:6},
        Rule::H{ total:17, y:1, x1:0, x2:1 }, Rule::H{ total:29, y:1, x1:3, x2:6},
        Rule::H{ total:35, y:2, x1:0, x2:4 },
        Rule::H{ total: 7, y:3, x1:1, x2:2 }, Rule::H{ total: 8, y:3, x1:4, x2:5},
        Rule::H{ total:16, y:4, x1:2, x2:6 },
        Rule::H{ total:21, y:5, x1:0, x2:3 }, Rule::H{ total: 5, y:5, x1:5, x2:6},
        Rule::H{ total: 6, y:6, x1:0, x2:2 }, Rule::H{ total: 3, y:6, x1:5, x2:6},

        Rule::V{ total:23, x:0, y1:0, y2:2 }, Rule::V{ total:11, x:0, y1:5, y2:6},
        Rule::V{ total:30, x:1, y1:0, y2:3 }, Rule::V{ total:10, x:1, y1:5, y2:6},
        Rule::V{ total:15, x:2, y1:2, y2:6 },
        Rule::V{ total:17, x:3, y1:1, y2:2 }, Rule::V{ total: 7, x:3, y1:4, y2:5},
        Rule::V{ total:27, x:4, y1:0, y2:4 },
        Rule::V{ total:12, x:5, y1:0, y2:1 }, Rule::V{ total:12, x:5, y1:3, y2:6},
        Rule::V{ total:16, x:6, y1:0, y2:1 }, Rule::V{ total: 7, x:6, y1:4, y2:6},
    ];

    let expected = [
        [ 9, 7, X, X, 8, 7, 9 ],
        [ 8, 9, X, 8, 9, 5, 7 ],
        [ 6, 8, 5, 9, 7, X, X ],
        [ X, 6, 1, X, 2, 6, X ],
        [ X, X, 4, 6, 1, 3, 2 ],
        [ 8, 9, 3, 1, X, 1, 4 ],
        [ 3, 1, 2, X, X, 2, 1 ] ];

    let (mut sys, vars) = make_kakuro(&puzzle);
    let dict = sys.solve_any().expect("solution");
    print_kakuro(&dict, &vars);
    verify_kakuro(&dict, &vars, &expected);
    println!("kakuro_wikipedia: {} guesses", sys.num_guesses());
}
