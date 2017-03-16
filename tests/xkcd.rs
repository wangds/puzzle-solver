//! xkcd Knapsack Problem.
//!
//! https://en.wikipedia.org/wiki/Knapsack_problem
//! https://xkcd.com/287/

extern crate num_rational;
extern crate num_traits;
extern crate puzzle_solver;

use num_rational::Ratio;
use num_traits::ToPrimitive;
use puzzle_solver::{LinExpr,Puzzle,Val};

#[test]
fn xkcd_knapsack() {
    let menu = [
        (Ratio::new(2_15, 100), "Mixed Fruit"),
        (Ratio::new(2_75, 100), "French Fries"),
        (Ratio::new(3_35, 100), "Side Salad"),
        (Ratio::new(3_55, 100), "Hot Wings"),
        (Ratio::new(4_20, 100), "Mozzarella Sticks"),
        (Ratio::new(5_80, 100), "Sampler Plate") ];

    let mut sys = Puzzle::new();
    let mut vars = Vec::with_capacity(menu.len());
    let total = Ratio::new(15_05, 100);

    for &(cost, _) in menu.iter() {
        let num = (total / cost).floor().to_integer();
        let var = sys.new_var_with_candidates(&(0..(num + 1)).collect::<Vec<Val>>());
        vars.push(var)
    }

    sys.equals(total, vars.iter().zip(menu.iter()).fold(LinExpr::from(0),
            |sum, (&var, &(cost, _))| sum + var * cost));

    let solutions = sys.solve_all();
    assert_eq!(solutions.len(), 2);

    for dict in solutions.iter() {
        println!("");
        for (&var, &(cost, string)) in vars.iter().zip(menu.iter()) {
            let numer = cost.numer().to_f32().unwrap();
            let denom = cost.denom().to_f32().unwrap();
            println!(" {} x {:.2} {}", dict[var], numer / denom, string);
        }
    }

    println!("xkcd_knapsack: {} guesses", sys.num_guesses());
}
