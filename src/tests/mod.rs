mod test_monomial;
mod test_polynomial;

use crate::Monomial;
use std::f64::consts::{E, PI};

mod monomial_statics {
    use super::*;

    pub static MONOMIAL_0_0: Monomial = Monomial { c: 0_f64, e: 0_f64 };
    pub static MONOMIAL_0_500: Monomial = Monomial {
        c: 0_f64,
        e: 500_f64,
    };
    pub static MONOMIAL_0P5_N1: Monomial = Monomial { c: 0.5, e: -1.0 };
    pub static MONOMIAL_102_250: Monomial = Monomial {
        c: 102_f64,
        e: 250_f64,
    };
    pub static MONOMIAL_1_0: Monomial = Monomial { c: 1_f64, e: 0_f64 };
    pub static MONOMIAL_1_1: Monomial = Monomial { c: 1_f64, e: 1_f64 };
    pub static MONOMIAL_1_2: Monomial = Monomial { c: 1_f64, e: 2_f64 };
    pub static MONOMIAL_1_N1: Monomial = Monomial {
        c: 1_f64,
        e: -1_f64,
    };
    pub static MONOMIAL_1_N11: Monomial = Monomial {
        c: 1_f64,
        e: -11_f64,
    };
    pub static MONOMIAL_2_0: Monomial = Monomial { c: 2_f64, e: 0_f64 };
    pub static MONOMIAL_2_1: Monomial = Monomial { c: 2_f64, e: 1_f64 };
    pub static MONOMIAL_2_2: Monomial = Monomial { c: 2_f64, e: 2_f64 };
    pub static MONOMIAL_241346513P3452231_N3954398000P8481: Monomial = Monomial {
        c: 241346513.3452231_f64,
        e: -3954398000.8481_f64,
    };
    pub static MONOMIAL_2565_500: Monomial = Monomial {
        c: 2565_f64,
        e: 500_f64,
    };
    pub static MONOMIAL_2_N11: Monomial = Monomial {
        c: 2_f64,
        e: -11_f64,
    };
    pub static MONOMIAL_2_N22: Monomial = Monomial {
        c: 2_f64,
        e: -22_f64,
    };
    pub static MONOMIAL_3_0: Monomial = Monomial { c: 3_f64, e: 0_f64 };
    pub static MONOMIAL_3_1: Monomial = Monomial { c: 3_f64, e: 1_f64 };
    pub static MONOMIAL_3_2: Monomial = Monomial { c: 3_f64, e: 2_f64 };
    pub static MONOMIAL_3_3: Monomial = Monomial { c: 3_f64, e: 3_f64 };
    pub static MONOMIAL_3_N11: Monomial = Monomial {
        c: 3_f64,
        e: -11_f64,
    };
    pub static MONOMIAL_45_250: Monomial = Monomial {
        c: 45_f64,
        e: 250_f64,
    };
    pub static MONOMIAL_57_250: Monomial = Monomial {
        c: 57_f64,
        e: 250_f64,
    };
    pub static MONOMIAL_6_3: Monomial = Monomial { c: 6_f64, e: 3_f64 };
    pub static MONOMIAL_E_0: Monomial = Monomial { c: E, e: 0_f64 };
    pub static MONOMIAL_N34P2_389651P6516: Monomial = Monomial {
        c: -34.2_f64,
        e: 389651.6516_f64,
    };
    pub static MONOMIAL_N4_0: Monomial = Monomial {
        c: -4_f64,
        e: 0_f64,
    };
    pub static MONOMIAL_PI_0: Monomial = Monomial { c: PI, e: 0_f64 };
}
