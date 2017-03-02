//! Send More Money.
//!
//! https://en.wikipedia.org/wiki/Verbal_arithmetic

extern crate puzzle_solver;

use puzzle_solver::{Puzzle,Solution,VarToken};

fn make_send_more_money() -> (Puzzle, Vec<VarToken>) {
    let mut sys = Puzzle::new();
    let vars = sys.new_vars_with_candidates_1d(8, &[0,1,2,3,4,5,6,7,8,9]);
    let (s, e, n, d) = (vars[0], vars[1], vars[2], vars[3]);
    let (m, o, r, y) = (vars[4], vars[5], vars[6], vars[7]);

    sys.remove_candidates(s, &[0]);
    sys.remove_candidates(m, &[0]);

    sys.all_different(&vars);

    let send = 1000 * s + 100 * e + 10 * n + d;
    let more = 1000 * m + 100 * o + 10 * r + e;
    let money = 10000 * m + 1000 * o + 100 * n + 10 * e + y;
    sys.equals(send + more, money);

    (sys, vars)
}

fn print_send_more_money(dict: &Solution, vars: &Vec<VarToken>) {
    let (s, e, n, d) = (vars[0], vars[1], vars[2], vars[3]);
    let (m, o, r, y) = (vars[4], vars[5], vars[6], vars[7]);

    println!("   {} {} {} {}", dict[s], dict[e], dict[n], dict[d]);
    println!(" + {} {} {} {}", dict[m], dict[o], dict[r], dict[e]);
    println!("----------");
    println!(" {} {} {} {} {}", dict[m], dict[o], dict[n], dict[e], dict[y]);
}

fn verify_send_more_money(dict: &Solution, vars: &Vec<VarToken>) {
    let (s, e, n, d) = (vars[0], vars[1], vars[2], vars[3]);
    let (m, o, r, y) = (vars[4], vars[5], vars[6], vars[7]);

    assert_eq!(dict[o], 0);
    assert_eq!(dict[m], 1);
    assert_eq!(dict[y], 2);
    assert_eq!(dict[e], 5);
    assert_eq!(dict[n], 6);
    assert_eq!(dict[d], 7);
    assert_eq!(dict[r], 8);
    assert_eq!(dict[s], 9);
}

#[test]
fn sendmoremoney_carry() {
    let carry = [0,1];

    let (mut sys, vars) = make_send_more_money();
    let (s, e, n, d) = (vars[0], vars[1], vars[2], vars[3]);
    let (m, o, r, y) = (vars[4], vars[5], vars[6], vars[7]);
    let c1 = sys.new_var_with_candidates(&carry);
    let c2 = sys.new_var_with_candidates(&carry);
    let c3 = sys.new_var_with_candidates(&carry);
    sys.intersect_candidates(m, &carry); // c4 == m.

    sys.equals(     d + e, 10 * c1 + y);
    sys.equals(c1 + n + r, 10 * c2 + e);
    sys.equals(c2 + e + o, 10 * c3 + n);
    sys.equals(c3 + s + m, 10 *  m + o);

    let dict = sys.solve_unique().expect("solution");
    print_send_more_money(&dict, &vars);
    verify_send_more_money(&dict, &vars);
    println!("sendmoremoney_carry: {} guesses", sys.num_guesses());
}

#[test]
fn sendmoremoney_naive() {
    let (mut sys, vars) = make_send_more_money();
    let dict = sys.solve_unique().expect("solution");
    print_send_more_money(&dict, &vars);
    verify_send_more_money(&dict, &vars);
    println!("sendmoremoney_naive: {} guesses", sys.num_guesses());
}
