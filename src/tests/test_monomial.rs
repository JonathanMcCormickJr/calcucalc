use std::f64::consts::{PI, E};

use crate::Monomial;

static MONOMIAL_0P5_N1: Monomial = Monomial { c: 0.5, e: -1.0 };
static MONOMIAL_102_250: Monomial = Monomial { c: 102_f64, e: 250_f64 };
static MONOMIAL_1_0: Monomial = Monomial { c: 1_f64, e: 0_f64 };
static MONOMIAL_1_1: Monomial = Monomial { c: 1_f64, e: 1_f64 };
static MONOMIAL_1_2: Monomial = Monomial { c: 1_f64, e: 2_f64 };
static MONOMIAL_1_N11: Monomial = Monomial { c: 1_f64, e: -11_f64 };
static MONOMIAL_2_1: Monomial = Monomial { c: 2_f64, e: 1_f64 };
static MONOMIAL_2_2: Monomial = Monomial { c: 2_f64, e: 2_f64 };
static MONOMIAL_241346513P3452231_N3954398000P8481: Monomial = Monomial { c: 241346513.3452231_f64, e: -3954398000.8481_f64 };
static MONOMIAL_2_N11: Monomial = Monomial { c: 2_f64, e: -11_f64 };
static MONOMIAL_3_1: Monomial = Monomial { c: 3_f64, e: 1_f64 };
static MONOMIAL_3_3: Monomial = Monomial { c: 3_f64, e: 3_f64 };
static MONOMIAL_3_N11: Monomial = Monomial { c: 3_f64, e: -11_f64 };
static MONOMIAL_45_250: Monomial =  Monomial { c: 45_f64, e: 250_f64 };
static MONOMIAL_57_250: Monomial = Monomial { c: 57_f64, e: 250_f64 };
static MONOMIAL_E_0: Monomial = Monomial { c: E, e: 0_f64 };
static MONOMIAL_N34P2_389651P6516: Monomial = Monomial { c: -34.2_f64, e: 389651.6516_f64 };
static MONOMIAL_N4_0: Monomial = Monomial { c: -4_f64, e: 0_f64 };
static MONOMIAL_PI_0: Monomial = Monomial { c: PI, e: 0_f64 };

#[test]
fn test_monomial_identity() {
    assert_eq!(
        MONOMIAL_1_2,
        Monomial { c: 1_f64, e: 2_f64 }
    );
    assert_ne!(
        MONOMIAL_2_2,
        Monomial { c: 0_f64, e: 1_f64 }
    );
    assert_ne!(
        MONOMIAL_2_2,
        MONOMIAL_3_3
    );
    assert_eq!(MONOMIAL_PI_0, MONOMIAL_PI_0);
    assert_ne!(MONOMIAL_PI_0, MONOMIAL_1_2);
}

#[test]
fn test_monomial_fields_access() {
    let m0 = &MONOMIAL_1_0;
    assert_eq!(m0.c, 1_f64);
    assert_eq!(m0.e, 0_f64);
    
    let m1 = &MONOMIAL_N34P2_389651P6516;
    assert_eq!(m1.c, -34.2_f64);
    assert_eq!(m1.e, 389651.6516_f64);

    let m2 = &MONOMIAL_N4_0;
    assert_eq!(m2.c, -4_f64);
    assert_eq!(m2.e, 0_f64);

    let m3 = &MONOMIAL_241346513P3452231_N3954398000P8481;
    assert_eq!(m3.c, 241346513.3452231_f64);
    assert_eq!(m3.e, -3954398000.8481_f64);

    assert_eq!(MONOMIAL_1_2.c, 1_f64);
    assert_eq!(MONOMIAL_1_2.e, 2_f64);
    assert_eq!(MONOMIAL_PI_0.c, 3.14159265358979323846);
    assert_eq!(MONOMIAL_PI_0.e, 0_f64);
    assert_eq!(MONOMIAL_E_0.c, 2.71828182845904523536);
    assert_eq!(MONOMIAL_E_0.e, 0_f64);
}

#[test]
fn test_values_for_monomial() {
    let m1 = &MONOMIAL_1_1;
    assert_eq!(m1.value(2_f64), 2_f64);
    assert_eq!(m1.value(3_f64), 3_f64);
    assert_eq!(m1.value(4_f64), 4_f64);
    assert_eq!(m1.value(5_f64), 5_f64);

    let m2 = &MONOMIAL_2_2;
    assert_eq!(m2.value(2_f64), 8_f64);
    assert_eq!(m2.value(3_f64), 18_f64);
    assert_eq!(m2.value(4_f64), 32_f64);
    assert_eq!(m2.value(5_f64), 50_f64);

    let m3 = &MONOMIAL_0P5_N1;
    assert_eq!(m3.value(2.0), 0.25);
    assert_eq!(m3.value(3.0), 0.16666666666666666);
    assert_eq!(m3.value(4.0), 0.125);
    assert_eq!(m3.value(5.0), 0.1);
    assert_eq!(m3.value(0.5), 1.0);
    assert_eq!(m3.value(0.25), 2.0);
    assert_eq!(m3.value(0.125), 4.0);
    assert_eq!(m3.value(0.0625), 8.0);
    assert_eq!(m3.value(0.03125), 16.0);
    assert_eq!(m3.value(0.015625), 32.0);
    assert_eq!(m3.value(-1.0), -0.5);
    assert_eq!(m3.value(-2.0), -0.25);
    assert_eq!(m3.value(-3.0), -0.16666666666666666);
    assert_eq!(m3.value(-4.0), -0.125);
}

#[test]
fn test_add_monomial_of_same_power() {  
    let m1 = &MONOMIAL_1_1;
    let m2 = &MONOMIAL_2_1;
    let m3 = &MONOMIAL_3_1;
    assert_eq!(*m3, m1.add_monomial_of_same_power(m2.clone()));

    let m4 = &MONOMIAL_45_250;
    let m5 = &MONOMIAL_57_250;
    let m6 = &MONOMIAL_102_250;
    assert_eq!(*m6, m4.add_monomial_of_same_power(m5.clone()));

    let m7 = &MONOMIAL_1_N11;
    let m8 = &MONOMIAL_2_N11;
    let m9 = &MONOMIAL_3_N11;
    assert_eq!(*m9, m7.add_monomial_of_same_power(m8.clone()));

    let m10 = Monomial { c: 1_f64, e: 0_f64 };
    let m11 = Monomial { c: 2_f64, e: 0_f64 };
    let m12 = Monomial { c: 3_f64, e: 0_f64 };
    assert_eq!(m12, m10.add_monomial_of_same_power(m11));
}

#[test]
#[should_panic(expected = "Cannot add monomials with different powers of x.")]
fn test_panic_on_adding_monomial_of_different_power0() {
    let m1 = Monomial { c: 1_f64, e: 1_f64 };
    let m2 = Monomial { c: 2_f64, e: 2_f64 };
    m1.add_monomial_of_same_power(m2);
}

#[test]
#[should_panic(expected = "Cannot add monomials with different powers of x.")]
fn test_panic_on_adding_monomial_of_different_power1() {
    let m3 = Monomial { c: 1_f64, e: 0_f64 };
    let m4 = Monomial { c: 2_f64, e: 1_f64 };
    m3.add_monomial_of_same_power(m4);
}

#[test]
#[should_panic(expected = "Cannot add monomials with different powers of x.")]
fn test_panic_on_adding_monomial_of_different_power2() {
    let m5 = Monomial {
        c: 1_f64,
        e: -1_f64,
    };
    let m6 = Monomial { c: 2_f64, e: 0_f64 };
    m5.add_monomial_of_same_power(m6);
}

#[test]
#[should_panic(expected = "Cannot add monomials with different powers of x.")]
fn test_panic_on_adding_monomial_of_different_power3() {
    let m7 = Monomial {
        c: 1_f64,
        e: -1_f64,
    };
    let m8 = Monomial { c: 2_f64, e: 1_f64 };
    m7.add_monomial_of_same_power(m8);
}

#[test]
fn test_multiply_monomial() {
    let m1 = Monomial { c: 2_f64, e: 1_f64 };
    let m2 = Monomial { c: 3_f64, e: 2_f64 };
    let m3 = Monomial { c: 6_f64, e: 3_f64 };
    assert_eq!(m3, m1.multiply_monomial(m2));

    let m4 = Monomial {
        c: 45_f64,
        e: 250_f64,
    };
    let m5 = Monomial {
        c: 57_f64,
        e: 250_f64,
    };
    let m6 = Monomial {
        c: 2565_f64,
        e: 500_f64,
    };
    assert_eq!(m6, m4.multiply_monomial(m5));

    let m7 = Monomial {
        c: 1_f64,
        e: -11_f64,
    };
    let m8 = Monomial {
        c: 2_f64,
        e: -11_f64,
    };
    let m9 = Monomial {
        c: 2_f64,
        e: -22_f64,
    };
    assert_eq!(m9, m7.multiply_monomial(m8));

    let m10 = Monomial { c: 1_f64, e: 0_f64 };
    let m11 = Monomial { c: 2_f64, e: 0_f64 };
    let m12 = Monomial { c: 2_f64, e: 0_f64 };
    assert_eq!(m12, m10.multiply_monomial(m11));

    let m13 = Monomial { c: 1_f64, e: 0_f64 };
    let m14 = Monomial {
        c: 0_f64,
        e: 500_f64,
    };
    let m15 = Monomial {
        c: 0_f64,
        e: 500_f64,
    };
    assert_eq!(m15, m13.multiply_monomial(m14));

    let m16 = Monomial { c: 0_f64, e: 0_f64 };
    let m17 = Monomial { c: 0_f64, e: 0_f64 };
    let m18 = Monomial { c: 0_f64, e: 0_f64 };
    assert_eq!(m18, m16.multiply_monomial(m17));
}



// ONCE FINISHED WITH REFACTORING, SORT THE STATICS AND RE-RUN THE TESTS