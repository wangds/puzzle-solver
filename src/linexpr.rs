//! Linear expressions.

use std::collections::HashMap;
use std::collections::hash_map::Entry;
use std::convert::From;
use std::ops::{Add,Mul,Neg,Sub};

use ::{LinExpr,VarToken};

macro_rules! impl_commutative_op {
    ($LHS:ident + $RHS:ident) => {
        impl Add<$RHS> for $LHS {
            type Output = LinExpr;
            fn add(self, rhs: $RHS) -> Self::Output { rhs + self }
        }
    };
    ($LHS:ident * $RHS:ident) => {
        impl Mul<$RHS> for $LHS {
            type Output = LinExpr;
            fn mul(self, rhs: $RHS) -> Self::Output { rhs * self }
        }
    };
}

macro_rules! impl_subtract_op {
    ($LHS:ident - $RHS:ident) => {
        impl Sub<$RHS> for $LHS {
            type Output = LinExpr;
            fn sub(self, rhs: $RHS) -> Self::Output { self + (-rhs) }
        }
    }
}

/*--------------------------------------------------------------*/

impl From<i32> for LinExpr {
    fn from(constant: i32) -> Self {
        LinExpr {
            constant: constant,
            coef: HashMap::new(),
        }
    }
}

impl From<VarToken> for LinExpr {
    fn from(var: VarToken) -> Self {
        let mut coef = HashMap::new();
        coef.insert(var, 1);

        LinExpr {
            constant: 0,
            coef: coef,
        }
    }
}

/*--------------------------------------------------------------*/
/* Var-Coef                                                     */
/*--------------------------------------------------------------*/

impl Neg for VarToken {
    type Output = LinExpr;
    fn neg(self) -> Self::Output {
        -LinExpr::from(self)
    }
}

impl Add<i32> for VarToken {
    type Output = LinExpr;
    fn add(self, rhs: i32) -> Self::Output {
        LinExpr::from(self) + rhs
    }
}

impl_commutative_op!(i32 + VarToken);

impl_subtract_op!(VarToken - i32);
impl_subtract_op!(i32 - VarToken);

impl Mul<i32> for VarToken {
    type Output = LinExpr;
    fn mul(self, rhs: i32) -> Self::Output {
        LinExpr::from(self) * rhs
    }
}

impl_commutative_op!(i32 * VarToken);

/*--------------------------------------------------------------*/
/* Var-Var                                                      */
/*--------------------------------------------------------------*/

impl Add for VarToken {
    type Output = LinExpr;
    fn add(self, rhs: VarToken) -> Self::Output {
        LinExpr::from(self) + LinExpr::from(rhs)
    }
}

impl_subtract_op!(VarToken - VarToken);

/*--------------------------------------------------------------*/
/* Expr-Coef                                                    */
/*--------------------------------------------------------------*/

impl Neg for LinExpr {
    type Output = LinExpr;
    fn neg(self) -> Self::Output {
        -1 * self
    }
}

impl Add<i32> for LinExpr {
    type Output = LinExpr;
    fn add(mut self, rhs: i32) -> Self::Output {
        self.constant = self.constant + rhs;
        self
    }
}

impl_commutative_op!(i32 + LinExpr);

impl_subtract_op!(LinExpr - i32);
impl_subtract_op!(i32 - LinExpr);

impl Mul<i32> for LinExpr {
    type Output = LinExpr;
    fn mul(mut self, rhs: i32) -> Self::Output {
        if rhs == 0 {
            self.constant = 0;
            self.coef = HashMap::new();
        } else if rhs != 1 {
            self.constant = self.constant * rhs;
            for coef in self.coef.values_mut() {
                *coef *= rhs;
            }
        }

        self
    }
}

impl_commutative_op!(i32 * LinExpr);

/*--------------------------------------------------------------*/
/* Expr-Var                                                     */
/*--------------------------------------------------------------*/

impl Add<VarToken> for LinExpr {
    type Output = LinExpr;
    fn add(self, rhs: VarToken) -> Self::Output {
        self + LinExpr::from(rhs)
    }
}

impl_commutative_op!(VarToken + LinExpr);

impl_subtract_op!(LinExpr - VarToken);
impl_subtract_op!(VarToken - LinExpr);

/*--------------------------------------------------------------*/
/* Expr-Expr                                                    */
/*--------------------------------------------------------------*/

impl Add for LinExpr {
    type Output = LinExpr;
    fn add(mut self, mut rhs: LinExpr) -> Self::Output {
        self.constant = self.constant + rhs.constant;

        for (x2, a2) in rhs.coef.drain() {
            match self.coef.entry(x2) {
                Entry::Vacant(e) => {
                    e.insert(a2);
                },
                Entry::Occupied(mut e) => {
                    if *e.get() + a2 == 0 {
                        e.remove();
                    } else {
                        *e.get_mut() += a2;
                    }
                },
            }
        }

        self
    }
}

impl_subtract_op!(LinExpr - LinExpr);

/*--------------------------------------------------------------*/

#[cfg(test)]
mod tests {
    use ::Puzzle;

    #[test]
    fn test_ops() {
        let mut puzzle = Puzzle::new();
        let x = puzzle.new_var();
        let y = puzzle.new_var();

        // expr = var + const;
        let _ = x + 1;
        let _ = x - 1;
        let _ = x * 1;

        // expr = const + var;
        let _ = 1 + x;
        let _ = 1 - x;
        let _ = 1 * x;

        // expr = var + var;
        let _ = -x;
        let _ = x + y;
        let _ = x - y;

        // expr = expr + const;
        let _ = (x + y) + 1;
        let _ = (x + y) - 1;
        let _ = (x + y) * 1;

        // expr = const + expr;
        let _ = 1 + (x + y);
        let _ = 1 - (x + y);
        let _ = 1 * (x + y);

        // expr = expr + var;
        let _ = (x + 1) + y;
        let _ = (x + 1) - y;

        // expr = var + expr;
        let _ = x + (y + 1);
        let _ = x - (y + 1);

        // expr = expr + expr;
        let _ = -(x + y);
        let _ = (x + y) + (x + y);
        let _ = (x + y) - (x + y);
    }

    #[test]
    fn test_coef_zero() {
        let mut puzzle = Puzzle::new();
        let x = puzzle.new_var();
        let y = puzzle.new_var();

        let expr = x * 0;
        assert_eq!(expr.coef.len(), 0);

        let expr = x - x;
        assert_eq!(expr.coef.len(), 0);

        let expr = (x + y) * 0;
        assert_eq!(expr.coef.len(), 0);

        let expr = (x + y) - (x + y);
        assert_eq!(expr.coef.len(), 0);
    }
}
