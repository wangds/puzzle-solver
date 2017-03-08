//! Constraint trait, and some common constraints.
//!
//! Note that all puzzle states visited during the solution search
//! share the same set of constraint objects.  This means that you
//! cannot store additional information about the state (e.g. caches)
//! in the constraint to reuse later.

use std::rc::Rc;

use ::{PuzzleSearch,Val,VarToken};

/// Constraint trait.
pub trait Constraint {
    /// An iterator over the variables that are involved in the constraint.
    fn vars<'a>(&'a self) -> Box<Iterator<Item=&'a VarToken> + 'a>;

    /// Applied after a variable has been assigned.
    ///
    /// Returns true if the search should continue with these variable
    /// assignments, or false if the constraint found a contradiction.
    fn on_assigned(&self, _search: &mut PuzzleSearch, _var: VarToken, _val: Val)
            -> bool {
        true
    }

    /// Applied after a variable's candidates has been modified.
    ///
    /// Returns true if the search should continue with these variable
    /// assignments, or false if the constraint found a contradiction.
    fn on_updated(&self, _search: &mut PuzzleSearch) -> bool {
        true
    }

    /// Substitute the "from" variable with the "to" variable.
    ///
    /// Returns a new constraint with all instances of "from" replaced
    /// with "to", or None if a contradiction was found in the
    /// process.
    fn substitute(&self, from: VarToken, to: VarToken)
            -> Option<Rc<Constraint>>;
}

pub use self::alldifferent::AllDifferent;
pub use self::equality::Equality;

mod alldifferent;
mod equality;
