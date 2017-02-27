//! Integer division.

use std::i32;

pub trait IntDiv {
    fn div_round_up(self, b: Self) -> Self;
    fn div_round_down(self, b: Self) -> Self;
}

impl IntDiv for i32 {
    fn div_round_up(self, b: Self) -> Self {
        if b == 0 { panic!("attempt to divide by zero"); }
        if b == -1 && self == i32::MIN { panic!("attempt to divide with overflow"); }

        let aa = (self as i64).abs();
        let bb = (b as i64).abs();
        if (self > 0) == (b > 0) {
            ((aa + bb - 1) / bb) as i32
        } else {
            -(aa / bb) as i32
        }
    }

    fn div_round_down(self, b: Self) -> Self {
        if b == 0 { panic!("attempt to divide by zero"); }
        if b == -1 && self == i32::MIN { panic!("attempt to divide with overflow"); }

        let aa = (self as i64).abs();
        let bb = (b as i64).abs();
        if (self > 0) == (b > 0) {
            (aa / bb) as i32
        } else {
            -((aa + bb - 1) / bb) as i32
        }
    }
}

#[cfg(test)]
mod tests {
    use std::i32;
    use super::IntDiv;

    #[test]
    fn test_div_round_up() {
        // 7.0 / 5.0 = 1.4
        assert_eq!(( 0).div_round_up( 5),  0);
        assert_eq!(( 0).div_round_up(-5),  0);
        assert_eq!(( 7).div_round_up( 5),  2);
        assert_eq!(( 7).div_round_up(-5), -1);
        assert_eq!((-7).div_round_up( 5), -1);
        assert_eq!((-7).div_round_up(-5),  2);
        assert_eq!((i32::MIN).div_round_up(1), i32::MIN);
    }

    #[test]
    fn test_div_round_down() {
        // 7.0 / 5.0 = 1.4
        assert_eq!(( 0).div_round_down( 5),  0);
        assert_eq!(( 0).div_round_down(-5),  0);
        assert_eq!(( 7).div_round_down( 5),  1);
        assert_eq!(( 7).div_round_down(-5), -2);
        assert_eq!((-7).div_round_down( 5), -2);
        assert_eq!((-7).div_round_down(-5),  1);
    }

    #[test]
    #[should_panic]
    fn test_div_round_up_panic() {
        (i32::MIN).div_round_up(-1);
    }

    #[test]
    #[should_panic]
    fn test_div_round_down_panic() {
        (i32::MIN).div_round_down(-1);
    }
}
