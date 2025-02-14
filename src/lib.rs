

/// A monomial is a polynomial with only one term. It is a product of a coefficient and a power of x.
#[derive(Debug, PartialEq)]
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

// RESUME HERE BY IMPLEMENTING METHODS FOR THE POLYNOMIAL STRUCT.  


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


    // RESUME HERE BY IMPLEMENTING TESTS FOR THE POLYNOMIAL STRUCT.


}
