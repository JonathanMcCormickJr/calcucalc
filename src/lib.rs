

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

    /// Subtract one monomial from another, if they have the same power of x.
    fn subtract_monomial_of_same_power(&self, other: Monomial) -> Monomial {
        if self.power != other.power {
            panic!("Cannot subtract monomials with different powers of x.");
        };
        Monomial{coefficient: self.coefficient - other.coefficient, power: self.power}
    }

    /// Multiply one monomial by another.
    fn multiply_monomial(&self, other: Monomial) -> Monomial {
        Monomial{coefficient: self.coefficient * other.coefficient, power: self.power + other.power}
    }

    /// Divide one monomial by another.
    fn divide_monomial(&self, other: Monomial) -> Monomial {
        Monomial{coefficient: self.coefficient / other.coefficient, power: self.power - other.power}
    }


}


// RESUME HERE...


#[cfg(test)]
mod tests {
    use super::*;

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
    }

    #[test]
    #[should_panic]
    fn test_panic_on_adding_monomial_of_different_power() {
        let m1 = Monomial{coefficient: 1, power: 1};
        let m2 = Monomial{coefficient: 2, power: 2};
        m1.add_monomial_of_same_power(m2);
    }

    #[test]
    fn test_subtract_monomial_of_same_power() {
        let m1 = Monomial{coefficient: 1, power: 1};
        let m2 = Monomial{coefficient: 2, power: 1};
        let m3 = Monomial{coefficient: -1, power: 1};
        assert_eq!(m3, m1.subtract_monomial_of_same_power(m2));
    }

    #[test]
    #[should_panic]
    fn test_panic_on_subtracting_monomial_of_different_power() {
        let m1 = Monomial{coefficient: 1, power: 1};
        let m2 = Monomial{coefficient: 2, power: 2};
        m1.subtract_monomial_of_same_power(m2);
    }

    #[test]
    fn test_multiply_monomial() {
        let m1 = Monomial{coefficient: 2, power: 1};
        let m2 = Monomial{coefficient: 3, power: 2};
        let m3 = Monomial{coefficient: 6, power: 3};
        assert_eq!(m3, m1.multiply_monomial(m2));
    }

}
