

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


// RESUME HERE...


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_monomial() {
        assert_eq!(Monomial{coefficient: 1, power: 1}, Monomial{coefficient: 1, power: 1});
        assert_ne!(Monomial{coefficient: 1, power: 1}, Monomial{coefficient: 0, power: 1});
        assert_ne!(Monomial{coefficient: 1, power: 1}, Monomial{coefficient: 1, power: 0});

        // ...AND HERE
        
    }
}
