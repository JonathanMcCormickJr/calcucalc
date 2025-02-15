#![forbid(unsafe_code)]

/// A monomial is a polynomial with only one term. It is a product of a coefficient and a power of x.
#[derive(Clone, Debug, PartialEq)]
struct Monomial {
    c: i16, // Coefficient
    e: i16, // Exponent
}

impl Monomial {
    /// Add one monomial to another, if they have the same power of x.
    fn add_monomial_of_same_power(&self, other: Monomial) -> Monomial {
        if self.e != other.e {
            panic!("Cannot add monomials with different powers of x.");
        };
        Monomial {
            c: self.c + other.c,
            e: self.e,
        }
    }

    /// Multiply one monomial by another.
    fn multiply_monomial(&self, other: Monomial) -> Monomial {
        Monomial {
            c: self.c * other.c,
            e: self.e + other.e,
        }
    }
}

/// A polynomial is a sum of monomials.
#[derive(Debug, PartialEq)]
struct Polynomial(Vec<Monomial>);

impl Polynomial {
    /// Create new Polynomial
    fn new() -> Polynomial {
        Polynomial(vec![])
    }

    /// Combine elements of same powers
    fn simplify_by_combining_alike_powers(&self) -> Polynomial {
        let elements = &self.0;
        let mut simplified_elements = Polynomial::new();

        let mut first_element_transferred = false;

        for element in elements {
            if !first_element_transferred {
                simplified_elements.0.push(element.clone());
                first_element_transferred = true;
                continue;
            }

            let mut found_match = false;
            for simplified_element in &mut simplified_elements.0 {
                if simplified_element.e == element.e {
                    *simplified_element =
                        simplified_element.add_monomial_of_same_power(element.clone());
                    found_match = true;
                    break;
                }
            }
            if !found_match {
                simplified_elements.0.push(element.clone());
            }
        }
        simplified_elements
    }

    // Add one polynomial to another.
    // CONTINUE HERE...and add a slash to the end of the line above.
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests for the Monomial struct.

    #[test]
    fn test_monomial_identity() {
        assert_eq!(Monomial { c: 1, e: 1 }, Monomial { c: 1, e: 1 });
        assert_ne!(Monomial { c: 1, e: 1 }, Monomial { c: 0, e: 1 });
        assert_ne!(Monomial { c: 1, e: 1 }, Monomial { c: 1, e: 0 });
    }

    #[test]
    fn test_add_monomial_of_same_power() {
        let m1 = Monomial { c: 1, e: 1 };
        let m2 = Monomial { c: 2, e: 1 };
        let m3 = Monomial { c: 3, e: 1 };
        assert_eq!(m3, m1.add_monomial_of_same_power(m2));

        let m4 = Monomial { c: 45, e: 250 };
        let m5 = Monomial { c: 57, e: 250 };
        let m6 = Monomial { c: 102, e: 250 };
        assert_eq!(m6, m4.add_monomial_of_same_power(m5));

        let m7 = Monomial { c: 1, e: -11 };
        let m8 = Monomial { c: 2, e: -11 };
        let m9 = Monomial { c: 3, e: -11 };
        assert_eq!(m9, m7.add_monomial_of_same_power(m8));

        let m10 = Monomial { c: 1, e: 0 };
        let m11 = Monomial { c: 2, e: 0 };
        let m12 = Monomial { c: 3, e: 0 };
        assert_eq!(m12, m10.add_monomial_of_same_power(m11));
    }

    #[test]
    #[should_panic(expected = "Cannot add monomials with different powers of x.")]
    fn test_panic_on_adding_monomial_of_different_power0() {
        let m1 = Monomial { c: 1, e: 1 };
        let m2 = Monomial { c: 2, e: 2 };
        m1.add_monomial_of_same_power(m2);
    }

    #[test]
    #[should_panic(expected = "Cannot add monomials with different powers of x.")]
    fn test_panic_on_adding_monomial_of_different_power1() {
        let m3 = Monomial { c: 1, e: 0 };
        let m4 = Monomial { c: 2, e: 1 };
        m3.add_monomial_of_same_power(m4);
    }

    #[test]
    #[should_panic(expected = "Cannot add monomials with different powers of x.")]
    fn test_panic_on_adding_monomial_of_different_power2() {
        let m5 = Monomial { c: 1, e: -1 };
        let m6 = Monomial { c: 2, e: 0 };
        m5.add_monomial_of_same_power(m6);
    }

    #[test]
    #[should_panic(expected = "Cannot add monomials with different powers of x.")]
    fn test_panic_on_adding_monomial_of_different_power3() {
        let m7 = Monomial { c: 1, e: -1 };
        let m8 = Monomial { c: 2, e: 1 };
        m7.add_monomial_of_same_power(m8);
    }

    #[test]
    fn test_multiply_monomial() {
        let m1 = Monomial { c: 2, e: 1 };
        let m2 = Monomial { c: 3, e: 2 };
        let m3 = Monomial { c: 6, e: 3 };
        assert_eq!(m3, m1.multiply_monomial(m2));

        let m4 = Monomial { c: 45, e: 250 };
        let m5 = Monomial { c: 57, e: 250 };
        let m6 = Monomial { c: 2565, e: 500 };
        assert_eq!(m6, m4.multiply_monomial(m5));

        let m7 = Monomial { c: 1, e: -11 };
        let m8 = Monomial { c: 2, e: -11 };
        let m9 = Monomial { c: 2, e: -22 };
        assert_eq!(m9, m7.multiply_monomial(m8));

        let m10 = Monomial { c: 1, e: 0 };
        let m11 = Monomial { c: 2, e: 0 };
        let m12 = Monomial { c: 2, e: 0 };
        assert_eq!(m12, m10.multiply_monomial(m11));

        let m13 = Monomial { c: 1, e: 0 };
        let m14 = Monomial { c: 0, e: 500 };
        let m15 = Monomial { c: 0, e: 500 };
        assert_eq!(m15, m13.multiply_monomial(m14));

        let m16 = Monomial { c: 0, e: 0 };
        let m17 = Monomial { c: 0, e: 0 };
        let m18 = Monomial { c: 0, e: 0 };
        assert_eq!(m18, m16.multiply_monomial(m17));
    }

    // Tests for the Polynomial struct.

    #[test]
    fn test_polynomial_identity() {
        let p1 = Polynomial(vec![Monomial { c: 1, e: 1 }]);
        let p2 = Polynomial(vec![Monomial { c: 1, e: 1 }]);
        assert_eq!(p1, p2);

        let p3 = Polynomial(vec![Monomial { c: 1, e: 1 }]);
        let p4 = Polynomial(vec![Monomial { c: 1, e: 0 }]);
        assert_ne!(p3, p4);

        let p5 = Polynomial(vec![Monomial { c: 1, e: 1 }]);
        let p6 = Polynomial(vec![Monomial { c: 0, e: 1 }]);
        assert_ne!(p5, p6);

        let p7 = Polynomial(vec![Monomial { c: 1, e: 1 }]);
        let p8 = Polynomial(vec![Monomial { c: 0, e: 0 }]);
        assert_ne!(p7, p8);

        let p9 = Polynomial(vec![Monomial { c: 1, e: 1 }]);
        let p10 = Polynomial(vec![Monomial { c: 1, e: 1 }, Monomial { c: 1, e: 1 }]);
        assert_ne!(p9, p10);
    }

    #[test]
    fn test_new_polynomial() {
        let p1 = Polynomial::new();
        let p2 = Polynomial(vec![]);
        assert_eq!(p1, p2);

        assert_eq!(p1.0.len(), 0);
        assert_eq!(p2.0.len(), 0);
        assert_eq!(p1.0, p2.0);

        assert_eq!(24, std::mem::size_of_val(&p1));
        assert_eq!(24, std::mem::size_of_val(&p2));
    }

    #[test]
    fn test_simplify_by_combining_alike_powers() {
        let p1 = Polynomial(vec![Monomial { c: 1, e: 1 }, Monomial { c: 2, e: 1 }]);
        let p2 = Polynomial(vec![Monomial { c: 3, e: 1 }]);
        assert_eq!(p2, p1.simplify_by_combining_alike_powers());

        let p3 = Polynomial(vec![
            Monomial { c: 1, e: 1 },
            Monomial { c: 2, e: 1 },
            Monomial { c: 3, e: 1 },
        ]);
        let p4 = Polynomial(vec![Monomial { c: 6, e: 1 }]);
        assert_eq!(p4, p3.simplify_by_combining_alike_powers());

        let p5 = Polynomial(vec![
            Monomial { c: -1, e: 1 },
            Monomial { c: -2, e: 1 },
            Monomial { c: -3, e: 1 },
            Monomial { c: 4, e: 1 },
        ]);
        let p6 = Polynomial(vec![Monomial { c: -2, e: 1 }]);
        assert_eq!(p6, p5.simplify_by_combining_alike_powers());

        let p7 = Polynomial(vec![
            Monomial { c: 1, e: 1 },
            Monomial { c: 2, e: 1 },
            Monomial { c: 3, e: 1 },
            Monomial { c: 4, e: 1 },
            Monomial { c: 5, e: 1 },
        ]);
        let p8 = Polynomial(vec![Monomial { c: 15, e: 1 }]);
        assert_eq!(p8, p7.simplify_by_combining_alike_powers());

        let p9 = Polynomial(vec![
            Monomial { c: 1, e: 1 },
            Monomial { c: 2, e: 1 },
            Monomial { c: 3, e: 1 },
            Monomial { c: 4, e: 1 },
            Monomial { c: 5, e: 1 },
            Monomial { c: 6, e: 1 },
        ]);
        let p10 = Polynomial(vec![Monomial { c: 21, e: 1 }]);
        assert_eq!(p10, p9.simplify_by_combining_alike_powers());

        let p11 = Polynomial(vec![
            Monomial { c: 1, e: 1 },
            Monomial { c: 2, e: 1 },
            Monomial { c: 3, e: 1 },
            Monomial { c: 4, e: 1 },
            Monomial { c: 5, e: 1 },
            Monomial { c: 6, e: 1 },
            Monomial { c: 7, e: 1 },
        ]);
        let p12 = Polynomial(vec![Monomial { c: 28, e: 1 }]);
        assert_eq!(p12, p11.simplify_by_combining_alike_powers());

        // ADD MORE CASES HERE...
    }
    // RESUME HERE BY IMPLEMENTING TESTS FOR THE POLYNOMIAL STRUCT.
}
