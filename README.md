
Puzzle Solver [![Version][version-img]][version-url] [![Status][travis-ci-img]][travis-ci-url]
=============


About
-----

Solve logic puzzles by simply describing the puzzle's rules as
constraints.  This is suitable for solving puzzles with integer
variables such as Sudoku.


Examples
--------

A few example programs are provided in the `tests/` directory:

* _Sudoku_ - https://en.wikipedia.org/wiki/Sudoku
* _N-queens problem_ - https://en.wikipedia.org/wiki/Eight_queens_puzzle

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

Add Puzzle Solver as a dependency to your project's Cargo.toml:

```toml
[dependencies]
puzzle-solver = "0.1"
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
