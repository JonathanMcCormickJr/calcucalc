

/// A monomial is a polynomial with only one term. It is a product of a coefficient and a power of x.
#[derive(Clone, Debug, PartialEq)]
struct Monomial {
    coefficient: i16,
    power: i16,
}

impl Monomial {

    /// Add one monomial to another, if they have the same power of x.
    fn add_monomial_of_same_power(&self, other: Monomial) -> Monomial {
        if self.power != other.power {
            panic!("Cannot add monomials with different powers of x.");
        };
        Monomial{coefficient: self.coefficient + other.coefficient, power: self.power}
    }

    /// Multiply one monomial by another.
    fn multiply_monomial(&self, other: Monomial) -> Monomial {
        Monomial{coefficient: self.coefficient * other.coefficient, power: self.power + other.power}
    }
}

/// A polynomial is a sum of monomials.
#[derive(Debug, PartialEq)]
struct Polynomial {
    monomials: Vec<Monomial>,
}

impl Polynomial {
    /// Create new Polynomial
    fn new() -> Polynomial {
        Polynomial{
            monomials: vec![],
        }
    }

    /// Combine elements of same powers
    fn simplify_by_combining_alike_powers(&self) -> Polynomial {
        let elements = &self.monomials;
        let mut simplified_elements = Polynomial::new();
        
        let mut first_element_transferred = false;

        for element in elements {
            if !first_element_transferred {
                simplified_elements.monomials.push(element.clone());
                first_element_transferred = true;
                continue;
            }

            let mut found_match = false;
            for simplified_element in &mut simplified_elements.monomials {
                if simplified_element.power == element.power {
                    *simplified_element = simplified_element.add_monomial_of_same_power(element.clone());
                    found_match = true;
                    break;
                }
            }
            if !found_match {
                simplified_elements.monomials.push(element.clone());
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
        assert_eq!(Monomial{coefficient: 1, power: 1}, Monomial{coefficient: 1, power: 1});
        assert_ne!(Monomial{coefficient: 1, power: 1}, Monomial{coefficient: 0, power: 1});
        assert_ne!(Monomial{coefficient: 1, power: 1}, Monomial{coefficient: 1, power: 0});
    }


    #[test]
    fn test_add_monomial_of_same_power() {
        let m1 = Monomial{coefficient: 1, power: 1};
        let m2 = Monomial{coefficient: 2, power: 1};
        let m3 = Monomial{coefficient: 3, power: 1};
        assert_eq!(m3, m1.add_monomial_of_same_power(m2));

        let m4 = Monomial{coefficient: 45, power: 250};
        let m5 = Monomial{coefficient: 57, power: 250};
        let m6 = Monomial{coefficient: 102, power: 250};
        assert_eq!(m6, m4.add_monomial_of_same_power(m5));

        let m7 = Monomial{coefficient: 1, power: -11};
        let m8 = Monomial{coefficient: 2, power: -11};
        let m9 = Monomial{coefficient: 3, power: -11};
        assert_eq!(m9, m7.add_monomial_of_same_power(m8));

        let m10 = Monomial{coefficient: 1, power: 0};
        let m11 = Monomial{coefficient: 2, power: 0};
        let m12 = Monomial{coefficient: 3, power: 0};
        assert_eq!(m12, m10.add_monomial_of_same_power(m11));
    }

    #[test]
    #[should_panic(expected = "Cannot add monomials with different powers of x.")]
    fn test_panic_on_adding_monomial_of_different_power0() {
        let m1 = Monomial{coefficient: 1, power: 1};
        let m2 = Monomial{coefficient: 2, power: 2};
        m1.add_monomial_of_same_power(m2);
    }

    #[test]
    #[should_panic(expected = "Cannot add monomials with different powers of x.")]
    fn test_panic_on_adding_monomial_of_different_power1() {
        let m3 = Monomial{coefficient: 1, power: 0};
        let m4 = Monomial{coefficient: 2, power: 1};
        m3.add_monomial_of_same_power(m4);
    }

    #[test]
    #[should_panic(expected = "Cannot add monomials with different powers of x.")]
    fn test_panic_on_adding_monomial_of_different_power2() {
        let m5 = Monomial{coefficient: 1, power: -1};
        let m6 = Monomial{coefficient: 2, power: 0};
        m5.add_monomial_of_same_power(m6);
    }

    #[test]
    #[should_panic(expected = "Cannot add monomials with different powers of x.")]
    fn test_panic_on_adding_monomial_of_different_power3() {
        let m7 = Monomial{coefficient: 1, power: -1};
        let m8 = Monomial{coefficient: 2, power: 1};
        m7.add_monomial_of_same_power(m8);
    }

    #[test]
    fn test_multiply_monomial() {
        let m1 = Monomial{coefficient: 2, power: 1};
        let m2 = Monomial{coefficient: 3, power: 2};
        let m3 = Monomial{coefficient: 6, power: 3};
        assert_eq!(m3, m1.multiply_monomial(m2));

        let m4 = Monomial{coefficient: 45, power: 250};
        let m5 = Monomial{coefficient: 57, power: 250};
        let m6 = Monomial{coefficient: 2565, power: 500};
        assert_eq!(m6, m4.multiply_monomial(m5));

        let m7 = Monomial{coefficient: 1, power: -11};
        let m8 = Monomial{coefficient: 2, power: -11};
        let m9 = Monomial{coefficient: 2, power: -22};
        assert_eq!(m9, m7.multiply_monomial(m8));

        let m10 = Monomial{coefficient: 1, power: 0};
        let m11 = Monomial{coefficient: 2, power: 0};
        let m12 = Monomial{coefficient: 2, power: 0};
        assert_eq!(m12, m10.multiply_monomial(m11));

        let m13 = Monomial{coefficient: 1, power: 0};
        let m14 = Monomial{coefficient: 0, power: 500};
        let m15 = Monomial{coefficient: 0, power: 500};
        assert_eq!(m15, m13.multiply_monomial(m14));

        let m16 = Monomial{coefficient: 0, power: 0};
        let m17 = Monomial{coefficient: 0, power: 0};
        let m18 = Monomial{coefficient: 0, power: 0};
        assert_eq!(m18, m16.multiply_monomial(m17));
    }

    // Tests for the Polynomial struct.

    #[test]
    fn test_polynomial_identity() {
        let p1 = Polynomial{monomials: vec![Monomial{coefficient: 1, power: 1}]};
        let p2 = Polynomial{monomials: vec![Monomial{coefficient: 1, power: 1}]};
        assert_eq!(p1, p2);

        let p3 = Polynomial{monomials: vec![Monomial{coefficient: 1, power: 1}]};
        let p4 = Polynomial{monomials: vec![Monomial{coefficient: 1, power: 0}]};
        assert_ne!(p3, p4);

        let p5 = Polynomial{monomials: vec![Monomial{coefficient: 1, power: 1}]};
        let p6 = Polynomial{monomials: vec![Monomial{coefficient: 0, power: 1}]};
        assert_ne!(p5, p6);

        let p7 = Polynomial{monomials: vec![Monomial{coefficient: 1, power: 1}]};
        let p8 = Polynomial{monomials: vec![Monomial{coefficient: 0, power: 0}]};
        assert_ne!(p7, p8);

        let p9 = Polynomial{monomials: vec![Monomial{coefficient: 1, power: 1}]};
        let p10 = Polynomial{monomials: vec![Monomial{coefficient: 1, power: 1}, Monomial{coefficient: 1, power: 1}]};
        assert_ne!(p9, p10);
    }

    #[test]
    fn test_new_polynomial() {
        let p1 = Polynomial::new();
        let p2 = Polynomial{monomials: vec![]};
        assert_eq!(p1, p2);

        assert_eq!(p1.monomials.len(), 0);
        assert_eq!(p2.monomials.len(), 0);
        assert_eq!(p1.monomials, p2.monomials);

        assert_eq!(24, std::mem::size_of_val(&p1));
        assert_eq!(24, std::mem::size_of_val(&p2));
    }

    #[test]
    fn test_simplify_by_combining_alike_powers() {
        let p1 = Polynomial{monomials: vec![Monomial{coefficient: 1, power: 1}, Monomial{coefficient: 2, power: 1}]};
        let p2 = Polynomial{monomials: vec![Monomial{coefficient: 3, power: 1}]};
        assert_eq!(p2, p1.simplify_by_combining_alike_powers());

        let p3 = Polynomial{monomials: vec![Monomial{coefficient: 1, power: 1}, Monomial{coefficient: 2, power: 1}, Monomial{coefficient: 3, power: 1}]};
        let p4 = Polynomial{monomials: vec![Monomial{coefficient: 6, power: 1}]};
        assert_eq!(p4, p3.simplify_by_combining_alike_powers());

        let p5 = Polynomial{monomials: vec![Monomial{coefficient: -1, power: 1}, Monomial{coefficient: -2, power: 1}, Monomial{coefficient: -3, power: 1}, Monomial{coefficient: 4, power: 1}]};
        let p6 = Polynomial{monomials: vec![Monomial{coefficient: -2, power: 1}]};
        assert_eq!(p6, p5.simplify_by_combining_alike_powers());

        let p7 = Polynomial{monomials: vec![Monomial{coefficient: 1, power: 1}, Monomial{coefficient: 2, power: 1}, Monomial{coefficient: 3, power: 1}, Monomial{coefficient: 4, power: 1}, Monomial{coefficient: 5, power: 1}]};
        let p8 = Polynomial{monomials: vec![Monomial{coefficient: 15, power: 1}]};
        assert_eq!(p8, p7.simplify_by_combining_alike_powers());

        let p9 = Polynomial{monomials: vec![Monomial{coefficient: 1, power: 1}, Monomial{coefficient: 2, power: 1}, Monomial{coefficient: 3, power: 1}, Monomial{coefficient: 4, power: 1}, Monomial{coefficient: 5, power: 1}, Monomial{coefficient: 6, power: 1}]};
        let p10 = Polynomial{monomials: vec![Monomial{coefficient: 21, power: 1}]};
        assert_eq!(p10, p9.simplify_by_combining_alike_powers());

        let p11 = Polynomial{monomials: vec![Monomial{coefficient: 1, power: 1}, Monomial{coefficient: 2, power: 1}, Monomial{coefficient: 3, power: 1}, Monomial{coefficient: 4, power: 1}, Monomial{coefficient: 5, power: 1}, Monomial{coefficient: 6, power: 1}, Monomial{coefficient: 7, power: 1}]};
        let p12 = Polynomial{monomials: vec![Monomial{coefficient: 28, power: 1}]};
        assert_eq!(p12, p11.simplify_by_combining_alike_powers());

        // ADD MORE CASES HERE...



    }
    // RESUME HERE BY IMPLEMENTING TESTS FOR THE POLYNOMIAL STRUCT.


}
