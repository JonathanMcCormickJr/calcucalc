use crate::Monomial;

#[test]
fn test_monomial_identity() {
    assert_eq!(
        Monomial { c: 1_f64, e: 1_f64 },
        Monomial { c: 1_f64, e: 1_f64 }
    );
    assert_ne!(
        Monomial { c: 1_f64, e: 1_f64 },
        Monomial { c: 0_f64, e: 1_f64 }
    );
    assert_ne!(
        Monomial { c: 1_f64, e: 1_f64 },
        Monomial { c: 1_f64, e: 0_f64 }
    );
}

#[test]
fn test_monomial_fields_access() {
    let m0 = Monomial { c: 1_f64, e: 1_f64 };
    assert_eq!(m0.c, 1_f64);
    assert_eq!(m0.e, 1_f64);
    
    let m1 = Monomial { c: -34.2_f64, e: 389651.6516_f64 };
    assert_eq!(m1.c, -34.2_f64);
    assert_eq!(m1.e, 389651.6516_f64);
}

#[test]
fn test_value_monomial() {
    let m1 = Monomial { c: 1_f64, e: 1_f64 };
    assert_eq!(m1.value(2_f64), 2_f64);
    assert_eq!(m1.value(3_f64), 3_f64);
    assert_eq!(m1.value(4_f64), 4_f64);
    assert_eq!(m1.value(5_f64), 5_f64);

    let m2 = Monomial { c: 2_f64, e: 2_f64 };
    assert_eq!(m2.value(2_f64), 8_f64);
    assert_eq!(m2.value(3_f64), 18_f64);
    assert_eq!(m2.value(4_f64), 32_f64);
    assert_eq!(m2.value(5_f64), 50_f64);

    let m3 = Monomial { c: 0.5, e: -1.0 };
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
    let m1 = Monomial { c: 1_f64, e: 1_f64 };
    let m2 = Monomial { c: 2_f64, e: 1_f64 };
    let m3 = Monomial { c: 3_f64, e: 1_f64 };
    assert_eq!(m3, m1.add_monomial_of_same_power(m2));

    let m4 = Monomial {
        c: 45_f64,
        e: 250_f64,
    };
    let m5 = Monomial {
        c: 57_f64,
        e: 250_f64,
    };
    let m6 = Monomial {
        c: 102_f64,
        e: 250_f64,
    };
    assert_eq!(m6, m4.add_monomial_of_same_power(m5));

    let m7 = Monomial {
        c: 1_f64,
        e: -11_f64,
    };
    let m8 = Monomial {
        c: 2_f64,
        e: -11_f64,
    };
    let m9 = Monomial {
        c: 3_f64,
        e: -11_f64,
    };
    assert_eq!(m9, m7.add_monomial_of_same_power(m8));

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
