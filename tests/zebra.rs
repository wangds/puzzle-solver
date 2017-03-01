//! Zebra puzzle (A.K.A. Einstein's riddle).
//!
//! https://en.wikipedia.org/wiki/Zebra_Puzzle
//! https://rosettacode.org/wiki/Zebra_puzzle

#![allow(non_upper_case_globals)]

extern crate puzzle_solver;

use puzzle_solver::Puzzle;

#[derive(Clone,Copy,Debug)]
enum Nat { Dane, Englishman, German, Norwegian, Swede }

#[derive(Clone,Copy,Debug)]
enum Col { Blue, Green, Red, White, Yellow }

#[derive(Clone,Copy,Debug)]
enum Dri { Beer, Coffee, Milk, Tea, Water }

#[derive(Clone,Copy,Debug)]
enum Smo { Blend, BlueMaster, Dunhill, PallMall, Prince }

#[derive(Clone,Copy,Debug)]
enum Pet { Bird, Cat, Dog, Fish, Horse }

#[test]
fn zebra() {
    use Nat::*; use Col::*; use Dri::*; use Smo::*; use Pet::*;

    // #1: There are five houses.
    let mut sys = Puzzle::new();
    let nat_var = sys.new_vars_with_candidates_1d(5, &[1,2,3,4,5]);
    let col_var = sys.new_vars_with_candidates_1d(5, &[1,2,3,4,5]);
    let dri_var = sys.new_vars_with_candidates_1d(5, &[1,2,3,4,5]);
    let smo_var = sys.new_vars_with_candidates_1d(5, &[1,2,3,4,5]);
    let pet_var = sys.new_vars_with_candidates_1d(5, &[1,2,3,4,5]);

    let nat = |n| nat_var[n as usize];
    let col = |n| col_var[n as usize];
    let dri = |n| dri_var[n as usize];
    let smo = |n| smo_var[n as usize];
    let pet = |n| pet_var[n as usize];

    sys.all_different(&nat_var);
    sys.all_different(&col_var);
    sys.all_different(&dri_var);
    sys.all_different(&smo_var);
    sys.all_different(&pet_var);

    // #2: The Englishman lives in the red house.
    sys.equals(nat(Englishman), col(Red));

    // #3: The Swede has a dog.
    sys.equals(nat(Swede), pet(Dog));

    // #4: The Dane drinks tea.
    sys.equals(nat(Dane), dri(Tea));

    // #5: The green house is immediately to the left of the white house.
    sys.equals(col(Green), col(White) - 1);

    // #6: They drink coffee in the green house.
    sys.equals(dri(Coffee), col(Green));

    // #7: The man who smokes Pall Mall has birds.
    sys.equals(smo(PallMall), pet(Bird));

    // #8: In the yellow house they smoke Dunhill.
    sys.equals(col(Yellow), smo(Dunhill));

    // #9: In the middle house they drink milk.
    sys.equals(dri(Milk), 3);

    // #10: The Norwegian lives in the first house.
    sys.equals(nat(Norwegian), 1);

    // #11: The man who smokes Blend lives in the house next to the house with cats.
    let neighbour11 = sys.new_var_with_candidates(&[-1,1]);
    sys.equals(smo(Blend), pet(Cat) + neighbour11);

    // #12: In a house next to the house where they have a horse, they smoke Dunhill.
    let neighbour12 = sys.new_var_with_candidates(&[-1,1]);
    sys.equals(pet(Horse), smo(Dunhill) + neighbour12);

    // #13: The man who smokes Blue Master drinks beer.
    sys.equals(smo(BlueMaster), dri(Beer));

    // #14: The German smokes Prince.
    sys.equals(nat(German), smo(Prince));

    // #15: The Norwegian lives next to the blue house.
    let neighbour15 = sys.new_var_with_candidates(&[-1,1]);
    sys.equals(nat(Norwegian), col(Blue) + neighbour15);

    // #16: They drink water in a house next to the house where they smoke Blend.
    let neighbour16 = sys.new_var_with_candidates(&[-1,1]);
    sys.equals(dri(Water), smo(Blend) + neighbour16);

    let dict = sys.solve_any().expect("solution");

    let expected = [
        (Norwegian,  Yellow, Water,  Dunhill,    Cat),
        (Dane,       Blue,   Tea,    Blend,      Horse),
        (Englishman, Red,    Milk,   PallMall,   Bird),
        (German,     Green,  Coffee, Prince,     Fish),
        (Swede,      White,  Beer,   BlueMaster, Dog) ];

    for &(n,c,d,s,p) in expected.iter() {
        assert_eq!(dict[nat(n)], dict[col(c)]);
        assert_eq!(dict[nat(n)], dict[dri(d)]);
        assert_eq!(dict[nat(n)], dict[smo(s)]);
        assert_eq!(dict[nat(n)], dict[pet(p)]);
    }

    println!("zebra: {} guesses", sys.num_guesses());
}
