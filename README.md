
Puzzle Solver [![Version][version-img]][version-url] [![Status][travis-ci-img]][travis-ci-url]
=============


About
-----

Solve logic puzzles by simply describing the puzzle's rules as
constraints.  This is suitable for solving puzzles with integer
variables such as Sudoku, Killer Sudoku, Kakuro, and Zebra puzzles.

The puzzle solver maintains a list of candidates for each puzzle
variable.  It solves puzzles by eliminating candidates that would lead
to a contradiction and taking any forced moves that were exposed in
the process.  This is repeated until it gets stuck, whereupon it will
perform a backtracking search -- it will assign a single variable and
continue with the candidate elimination step again.


Examples
--------

A few example programs are provided in the `tests/` directory:

* _Hidato_ - https://en.wikipedia.org/wiki/Hidato
* _Kakuro_ - https://en.wikipedia.org/wiki/Kakuro
* _Killer Sudoku_ - https://en.wikipedia.org/wiki/Killer_sudoku
* _Magic Square_ - https://en.wikipedia.org/wiki/Magic_square
* _N-queens problem_ - https://en.wikipedia.org/wiki/Eight_queens_puzzle
* _Send More Money_ - https://en.wikipedia.org/wiki/Verbal_arithmetic
* _Sudoku_ - https://en.wikipedia.org/wiki/Sudoku
  * _Samurai Sudoku_
* _Sujiko_ - https://en.wikipedia.org/wiki/Sujiko
* _Zebra puzzle (Einstein's riddle)_ - https://en.wikipedia.org/wiki/Zebra_Puzzle

To clone this repository, run:

```sh
git clone https://github.com/wangds/puzzle-solver.git
```

Then build the library and run the test programs using Cargo.

```sh
cargo test --test sudoku -- --nocapture
```


Basic Usage
-----------

We will demonstrate how to solve the equation "SEND + MORE = MONEY".
Add Puzzle Solver as a dependency to your project's Cargo.toml:

```toml
[dependencies]
puzzle-solver = "0.2"
```

Import the library in your project, e.g.:

```rust
extern crate puzzle_solver;

use puzzle_solver::Puzzle;
```

First, we create a puzzle object and the 8 puzzle variables
`(S,E,N,D,M,O,R,Y)`.

```rust
let mut puzzle = Puzzle::new();
let vars = puzzle.new_vars_with_candidates_1d(8, &[0,1,2,3,4,5,6,7,8,9]);
let (s, e, n, d) = (vars[0], vars[1], vars[2], vars[3]);
let (m, o, r, y) = (vars[4], vars[5], vars[6], vars[7]);
```

All eight puzzle variables have been initialised to be any number
between 0 and 9.  However, we know that the numbers are not allowed to
begin with zero, so we remove the choices of S = 0 and M = 0.

```rust
puzzle.remove_candidates(s, &[0]);
puzzle.remove_candidates(m, &[0]);
```

We add the constraint that the variables should be all different:

```rust
puzzle.all_different(&vars);
```

We write the equation as another puzzle constraint:

```rust
puzzle.equals(
    (1000 * s + 100 * e + 10 * n + d) + (1000 * m + 100 * o + 10 * r + e),
    10000 * m + 1000 * o + 100 * n + 10 * e + y);
```

And we solve!

```rust
let solution = puzzle.solve_any().expect("solution");
assert_eq!(solution[o], 0);
assert_eq!(solution[m], 1);
assert_eq!(solution[y], 2);
assert_eq!(solution[e], 5);
assert_eq!(solution[n], 6);
assert_eq!(solution[d], 7);
assert_eq!(solution[r], 8);
assert_eq!(solution[s], 9);
```


Documentation
-------------

* [Documentation][documentation].


Author
------

David Wang


[documentation]: https://docs.rs/puzzle-solver/
[travis-ci-img]: https://travis-ci.org/wangds/puzzle-solver.svg?branch=master
[travis-ci-url]: https://travis-ci.org/wangds/puzzle-solver
[version-img]: https://img.shields.io/crates/v/puzzle-solver.svg
[version-url]: https://crates.io/crates/puzzle-solver
