use std::io::Error;

#[derive(Debug, PartialEq)]
struct Monomial {
    coefficient: u8,
    power: u8,
}

impl Monomial {
    fn add_monomial_of_same_power(&self, other: Monomial) -> Result<Monomial, Error> {
        if self.power == other.power {
            Ok(Monomial{coefficient: self.coefficient + other.coefficient, power: self.power})
        } else {
            return Err(Error::new(std::io::ErrorKind::InvalidInput, "cannot combine monomials of different powers of x"));
        }
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
