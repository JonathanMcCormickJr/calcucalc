#![forbid(unsafe_code)]

//! # 🔢 calcucalc 📈
//!
//! A Rust library for doing general-purpose **calculus**.
//!
//! ## 🌱 Getting Started
//!
//! The goal of this project is to provide a simple and easy-to-use library for doing calculus operations.
//!
//! ### 🦀 Prerequisites
//!
//! You will need to have Rust installed on your machine. You can install Rust by following the instructions on the [official Rust lang site](https://www.rust-lang.org/tools/install).
//!
//! ### 🖥️ Installing
//!
//! To install this library, add the following to your `Cargo.toml` file:
//!
//! ```toml
//! [dependencies]
//! calcucalc = "VERSION_GOES_HERE"
//! ```
//!
//! It is recommended to use the latest version. You can find the latest version on [this library's crates.io page](https://crates.io/crates/calcucalc).
//!
//! ## 🛠️ Usage
//!
//! This documentation provides examples of how to use the library. Just navigate to the item you want to learn more about, and the description and examples will be there.
//!
//!

/// A monomial is a product of a coefficient and an exponent of x.
/// For example, in the monomial `3x^2`, the coefficient is `3` and the exponent of x is `2`.
/// The monomial `3x^2` can be represented as a struct with the coefficient `3` and the exponent `2`, which this library represents as `Monomial { c: 3.0, e: 2.0 }`.
///
/// This library is intended to be as general-purpose as possible, which is why the coefficient and exponent are represented as floating-point numbers (as opposed to integers). This allows for more flexibility in the types of functions that can be represented.
///
/// Here is a table showing example monomials and their corresponding struct representations:
/// | Monomial | Struct Representation |
/// | --- | --- |
/// | `3x^2` | `Monomial { c: 3.0, e: 2.0 }` |
/// | `2x` | `Monomial { c: 2.0, e: 1.0 }` |
/// | `1` | `Monomial { c: 1.0, e: 0.0 }` |
/// | `0` | `Monomial { c: 0.0, e: 0.0 }` |
/// | `5x^-1` aka `5/x` | `Monomial { c: 5.0, e: -1.0 }` |
/// | `3x^-2` aka `3/(x^2)` | `Monomial { c: 3.0, e: -2.0 }` |
/// | `2x^0.5` aka `2 * √x` | `Monomial { c: 2.0, e: 0.5 }` |
/// | `1.5x^0.25` aka `1.5 * ∜x` | `Monomial { c: 1.5, e: 0.25 }` |
/// | `2x^π` | `Monomial { c: 2.0, e: 3.141592653589793 }` |
/// | `3x^e` | `Monomial { c: 3.0, e: 2.718281828459045 }` |
///
#[derive(Clone, Debug, PartialEq)]
pub struct Monomial {
    /// <u>c</u>oefficient
    pub c: f64, // Coefficient
    /// <u>e</u>xponent
    pub e: f64, // Exponent
}

impl Monomial {
    /// Creates a new monomial
    ///
    /// #### Example
    /// ```rust
    /// use calcucalc::Monomial;
    ///
    /// let m = Monomial::new(1.0, 2.0);
    /// assert_eq!(m, Monomial { c: 1.0, e: 2.0 });
    /// ```
    pub fn new(c: f64, e: f64) -> Monomial {
        Monomial { c, e }
    }

    /// Calculates the value of the monomial for a given value of x.
    ///
    /// #### Example
    /// ```rust
    /// use calcucalc::Monomial;
    ///
    /// let m = Monomial { c: 2.0, e: 3.0 };
    /// assert_eq!(m.value(2.0), 16.0);
    /// assert_eq!(m.value(3.0), 54.0);
    /// assert_eq!(m.value(4.0), 128.0);
    /// assert_eq!(m.value(5.0), 250.0);
    /// ```
    pub fn value(&self, x: f64) -> f64 {
        self.c * (x.powf(self.e))
    }

    /// Adds one monomial to another, if they have the same exponent of x.
    ///
    /// #### Example
    /// ```rust
    /// use calcucalc::Monomial;
    ///
    /// let m1 = Monomial { c: 1.0, e: 3.141592653589793 };
    /// let m2 = Monomial { c: 2.0, e: 3.141592653589793 };
    /// let m3 = Monomial { c: 3.0, e: 3.141592653589793 };
    /// assert_eq!(m3, m1.add_monomial_of_same_power(m2));
    /// ```
    pub fn add_monomial_of_same_power(&self, other: Monomial) -> Monomial {
        if self.e != other.e {
            panic!("Cannot add monomials with different powers of x.");
        };
        Monomial {
            c: self.c + other.c,
            e: self.e,
        }
    }

    /// Multiplies one monomial by another.
    ///
    /// #### Example
    /// ```rust
    /// use calcucalc::Monomial;
    ///
    /// let m1 = Monomial { c: 1.0, e: 3.141592653589793 };
    /// let m2 = Monomial { c: 2.0, e: 3.141592653589793 };
    /// let m3 = Monomial { c: 2.0, e: 6.283185307179586 };
    /// assert_eq!(m3, m1.multiply_monomial(m2));
    /// ```
    pub fn multiply_monomial(&self, other: Monomial) -> Monomial {
        Monomial {
            c: self.c * other.c,
            e: self.e + other.e,
        }
    }

    /// Calculates the derivative of the monomial.
    /// The derivative of a monomial is the product of the exponent and the coefficient, times x raised to the power of the exponent minus one.
    ///
    /// #### Example
    /// ```rust
    /// use calcucalc::Monomial;
    ///
    /// let m = Monomial { c: 1.0, e: 3.141592653589793 };
    /// let m_derivative = Monomial { c: 3.141592653589793, e: 2.141592653589793 };
    /// assert_eq!(m_derivative, m.derivative());
    /// ```
    pub fn derivative(&self) -> Monomial {
        Monomial {
            c: self.c * self.e,
            e: self.e - 1_f64,
        }
    }

    /// Calculates the nth derivative of the monomial.
    ///
    /// The nth derivative of a monomial is the result of taking the derivative of the monomial `n` times.
    ///
    /// #### Example
    /// ```rust
    /// use calcucalc::Monomial;
    ///
    /// let m = Monomial { c: 1.0, e: 3.141592653589793 };
    /// let m_nth_derivative = m.nth_derivative(2);
    /// assert_eq!(m_nth_derivative, Monomial { c: 6.728011747499565, e: 1.1415926535897931 });
    /// ```
    pub fn nth_derivative(&self, n: u32) -> Monomial {
        let mut new_monomial = self.clone();
        for _ in 0..n {
            new_monomial = new_monomial.derivative();
        }
        new_monomial
    }
}

impl Default for Monomial {
    /// Defaults to a monomial with a coefficient of `0` and an exponent of `0`.
    ///
    /// This is equivalent to the monomial `0`.
    ///
    /// #### Example
    /// ```rust
    /// use calcucalc::Monomial;
    ///
    /// let m = Monomial::default();
    /// assert_eq!(m, Monomial { c: 0.0, e: 0.0 });
    /// ```
    fn default() -> Self {
        Self::new(0_f64, 0_f64)
    }
}

/// A polynomial is a sum of monomials.
/// For example, the polynomial `3x^2 + 2x + 1` can be represented as a vector of monomials, which is how this library represents it.
///
/// #### Example
/// ```rust
/// use calcucalc::{Monomial, Polynomial};
///
/// let my_polynomial = Polynomial(vec![
///    Monomial { c: 3.0, e: 2.0 },
///    Monomial { c: 2.0, e: 1.0 },
///    Monomial { c: 1.0, e: 0.0 },
/// ]);
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct Polynomial(pub Vec<Monomial>);

impl Polynomial {
    /// Creates a new polynomial with no monomials.
    ///
    /// #### Example
    /// ```rust
    /// use calcucalc::Polynomial;
    ///
    /// let my_polynomial = Polynomial::new();
    /// assert_eq!(my_polynomial.0.len(), 0)
    /// ```
    pub fn new() -> Polynomial {
        Polynomial(vec![])
    }

    /// Calculates the value of a polynomial for a given value of x.
    ///
    /// #### Example
    /// ```rust
    /// use calcucalc::{Monomial, Polynomial};
    ///
    /// let my_polynomial = Polynomial(vec![
    ///    Monomial { c: 1.0, e: 2.0 },
    ///    Monomial { c: 3.0, e: 1.0 },
    ///    Monomial { c: 2.0, e: 0.0 },
    /// ]);
    /// assert_eq!(my_polynomial.value(-1.0), 0.0);
    /// assert_eq!(my_polynomial.value(0.0), 2.0);
    /// assert_eq!(my_polynomial.value(1.0), 6.0);
    /// assert_eq!(my_polynomial.value(2.0), 12.0);
    /// assert_eq!(my_polynomial.value(3.0), 20.0);
    ///
    pub fn value(&self, x: f64) -> f64 {
        let elements = &self.0;
        let mut value = 0_f64;
        for element in elements {
            value += element.value(x);
        }
        value
    }

    /// Simplifies the polynomial by combining elements which have the same exponent of x, and then sorting the elements by the exponent of x (in descending order).
    ///
    /// This function is equivalent to calling `simplify_by_combining_alike_powers()`, `eliminate_zero_coefficients()`, and `sort_by_exponent()` in sequence.
    ///
    /// In most cases, if you want to simplify a polynomial, you should use this function.
    ///
    /// #### Example
    /// ```rust
    /// use calcucalc::{Monomial, Polynomial};
    ///
    /// let my_polynomial = Polynomial(vec![
    ///     Monomial { c: 1.0, e: 2.0 },
    ///     Monomial { c: -5.0, e: 0.0 },
    ///     Monomial { c: 2.0, e: 2.0 },
    ///     Monomial { c: 2.0, e: 1.0 },
    ///     Monomial { c: 0.0, e: 1.0 },
    ///     Monomial { c: 3.0, e: 0.0 },
    ///     Monomial { c: 500.0, e: 0.453 },
    ///     Monomial { c: -500.0, e: 0.453 },
    /// ]);
    ///
    /// let simplified_polynomial = my_polynomial.simplified();
    ///
    /// assert_eq!(simplified_polynomial, Polynomial(vec![
    ///     Monomial { c: 3.0, e: 2.0 },
    ///     Monomial { c: 2.0, e: 1.0 },
    ///     Monomial { c: -2.0, e: 0.0 },
    /// ]));
    /// ```
    pub fn simplified(&self) -> Polynomial {
        self.simplify_by_combining_alike_powers()
            .eliminate_zero_coefficients()
            .sort_by_exponent()
    }

    /// Combines elements which have the same exponent of x.
    ///
    /// #### Example
    /// ```rust
    /// use calcucalc::{Monomial, Polynomial};
    ///
    /// let my_polynomial = Polynomial(vec![
    ///     Monomial { c: 1.0, e: 2.0 },
    ///     Monomial { c: 2.0, e: 2.0 },
    ///     Monomial { c: 3.0, e: 2.0 },
    /// ]);
    /// let simplified_polynomial = my_polynomial.simplify_by_combining_alike_powers();
    /// assert_eq!(simplified_polynomial, Polynomial(vec![
    ///     Monomial { c: 6.0, e: 2.0 },
    /// ]));
    /// ```
    pub fn simplify_by_combining_alike_powers(&self) -> Polynomial {
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

    /// Eliminates elements with coefficients of `0`.
    ///
    /// As `0` multiplied by any number is `0`, elements with a coefficient of `0` do not affect the value of the polynomial, and can be safely ignored.
    ///
    /// #### Example
    /// ```rust
    /// use calcucalc::{Monomial, Polynomial};
    ///
    /// let my_polynomial = Polynomial(vec![
    ///     Monomial { c: 1.0, e: 2.0 },
    ///     Monomial { c: 0.0, e: 1.0 },
    /// ]);
    /// let simplified_polynomial = my_polynomial.eliminate_zero_coefficients();
    /// assert_eq!(simplified_polynomial, Polynomial(vec![
    ///    Monomial { c: 1.0, e: 2.0 },
    /// ]));
    /// ```
    pub fn eliminate_zero_coefficients(&self) -> Polynomial {
        let elements = &self.0;
        let mut new_elements = vec![];
        for element in elements {
            if element.c != 0_f64 {
                new_elements.push(element.clone());
            }
        }
        Polynomial(new_elements)
    }

    /// Sorts the elements of the polynomial by the exponent of x (in descending order).
    ///
    /// #### Example
    /// ```rust
    /// use calcucalc::{Monomial, Polynomial};
    ///
    /// let my_polynomial = Polynomial(vec![
    ///     Monomial { c: 3.0, e: -2.0 },
    ///     Monomial { c: 3.0, e: 0.0 },
    ///     Monomial { c: 3.0, e: 0.5 },
    ///     Monomial { c: 3.0, e: 25.4 },
    ///     Monomial { c: 3.0, e: 0.0 },
    ///     Monomial { c: 3.0, e: 12.0 },
    /// ]);
    /// let sorted_polynomial = my_polynomial.sort_by_exponent();
    /// assert_eq!(sorted_polynomial, Polynomial(vec![
    ///     Monomial { c: 3.0, e: 25.4 },
    ///     Monomial { c: 3.0, e: 12.0 },
    ///     Monomial { c: 3.0, e: 0.5 },
    ///     Monomial { c: 3.0, e: 0.0 },
    ///     Monomial { c: 3.0, e: 0.0 },  // Does not perform any combination of elements with the same exponent. Only sorts.
    ///     Monomial { c: 3.0, e: -2.0 },
    /// ]));
    /// ```
    pub fn sort_by_exponent(&self) -> Polynomial {
        let mut elements = self.0.clone();
        elements.sort_by(|a, b| b.e.partial_cmp(&a.e).unwrap());
        Polynomial(elements)
    }

    /// Adds one polynomial to another.
    ///
    /// #### Example
    /// ```rust
    /// use calcucalc::{Monomial, Polynomial};
    ///
    /// let my_polynomial1 = Polynomial(vec![
    ///     Monomial { c: 1.0, e: 2.0 },
    ///     Monomial { c: 2.0, e: 1.0 },
    /// ]);
    /// let my_polynomial2 = Polynomial(vec![
    ///     Monomial { c: 3.0, e: 2.0 },
    ///     Monomial { c: 4.0, e: 1.0 },
    /// ]);
    /// let my_sum = my_polynomial1.add_polynomial(my_polynomial2);
    /// assert_eq!(my_sum, Polynomial(vec![
    ///     Monomial { c: 4.0, e: 2.0 },
    ///     Monomial { c: 6.0, e: 1.0 },
    /// ]));
    /// ```
    ///
    /// `add_polynomial()` itself calls `simplified()` before returning the result.
    pub fn add_polynomial(&self, other: Polynomial) -> Polynomial {
        let mut elements = self.0.clone();
        elements.extend(other.0.clone());
        let new_polynomial = Polynomial(elements);
        new_polynomial.simplified()
    }

    /// Multiplies one polynomial by another.
    ///
    /// #### Example
    /// ```rust
    /// use calcucalc::{Monomial, Polynomial};
    ///
    /// let my_polynomial1 = Polynomial(vec![
    ///    Monomial { c: 1.0, e: 2.0 },
    ///    Monomial { c: 2.0, e: 1.0 },
    /// ]);
    /// let my_polynomial2 = Polynomial(vec![
    ///    Monomial { c: 4.0, e: 1.0 },
    ///    Monomial { c: 3.0, e: 2.0 },
    /// ]);
    /// let my_product = my_polynomial1.multiply_polynomial(my_polynomial2);
    /// assert_eq!(my_product, Polynomial(vec![
    ///    Monomial { c: 3.0, e: 4.0 },
    ///    Monomial { c: 10.0, e: 3.0 },
    ///    Monomial { c: 8.0, e: 2.0 },
    /// ]));
    /// ```
    ///
    /// `multiply_polynomial()` itself calls `simplified()` before returning the result.
    pub fn multiply_polynomial(&self, other: Polynomial) -> Polynomial {
        let mut elements = vec![];
        for element1 in &self.0 {
            for element2 in &other.0 {
                elements.push(element1.multiply_monomial(element2.clone()));
            }
        }
        let new_polynomial = Polynomial(elements);
        new_polynomial.simplified()
    }

    /// Calculates the derivative of the polynomial.
    ///
    /// The derivative of a polynomial is the sum of the derivatives of each monomial in the polynomial.
    ///
    /// #### Example
    /// ```rust
    /// use calcucalc::{Monomial, Polynomial};
    ///
    /// let mut my_polynomial = Polynomial(vec![
    ///     Monomial { c: 1.0, e: 2.0 },
    ///     Monomial { c: 2.0, e: 1.0 },
    ///     Monomial { c: 3.0, e: 0.0 },
    /// ]);
    /// let my_derivative = my_polynomial.derivative();
    /// assert_eq!(my_derivative, Polynomial(vec![Monomial { c: 2.0, e: 1.0 }, Monomial { c: 2.0, e: 0.0 }]));
    /// ```
    ///
    /// The above code does the same as the following mathematical expression:
    /// ```math
    /// f(x) = x^2 + 2x + 3
    /// f'(x) = 2x + 2
    /// ```
    ///
    /// `derivative()` itself calls `simplified()` before returning the result.
    pub fn derivative(&self) -> Polynomial {
        let mut elements = vec![];
        for element in &self.0 {
            elements.push(element.derivative());
        }
        Polynomial(elements).simplified()
    }

    /// Calculates the nth derivative of the polynomial.
    ///
    /// The nth derivative of a polynomial is the result of taking the derivative of the polynomial `n` times.
    ///
    /// #### Example
    /// ```rust
    /// use calcucalc::{Monomial, Polynomial};
    ///
    /// let my_polynomial = Polynomial(vec![
    ///     Monomial { c: 1.0, e: 2.0 },
    ///     Monomial { c: 2.0, e: 1.0 },
    ///     Monomial { c: 3.0, e: 0.0 },
    /// ]);
    /// let my_nth_derivative = my_polynomial.nth_derivative(2);
    /// assert_eq!(my_nth_derivative, Polynomial(vec![Monomial { c: 2.0, e: 0.0 }]));
    /// ```
    ///
    /// The above code does the same as the following mathematical expression:
    /// ```math
    /// f(x) = x^2 + 2x + 3
    /// f''(x) = 2
    /// ```
    ///
    /// Let's level up the complexity a bit:
    /// ```rust
    /// use calcucalc::{Monomial, Polynomial};
    ///
    /// let my_polynomial = Polynomial(vec![
    ///     Monomial { c: 1.0, e: 3.0 },
    ///     Monomial { c: 2.0, e: 2.0 },
    ///     Monomial { c: 3.0, e: 1.0 },
    ///     Monomial { c: 4.0, e: 0.0 },
    /// ]);
    /// let my_nth_derivative = my_polynomial.nth_derivative(3);
    /// assert_eq!(my_nth_derivative, Polynomial(vec![Monomial { c: 6.0, e: 0.0 }]));  // The third derivative of a constant is always 0.
    /// ```
    pub fn nth_derivative(&self, n: u32) -> Polynomial {
        let mut new_polynomial = self.clone();
        for _ in 0..n {
            new_polynomial = new_polynomial.derivative();
        }
        new_polynomial
    }

    /// Checks if the polynomial is equal to another polynomial within a certain tolerance.
    ///
    /// This function is to overcome floating point arithmetic errors.
    ///
    /// #### Example
    /// ```rust
    /// use calcucalc::{Monomial, Polynomial};
    ///
    /// let my_polynomial1 = Polynomial(vec![
    ///     Monomial { c: 1.0, e: 2.000000000001 },
    ///     Monomial { c: 1.999999999999, e: 1.0 },
    /// ]);
    /// let my_polynomial2 = Polynomial(vec![
    ///     Monomial { c: 1.0, e: 2.0 },
    ///     Monomial { c: 2.0, e: 1.0 },
    /// ]);
    /// assert!(my_polynomial1.is_equal_within_tolerance_to(my_polynomial2));
    /// ```
    ///
    /// The above code will return `true` because the two polynomials are equal within a tolerance of `1e-10`.
    pub fn is_equal_within_tolerance_to(&self, other: Polynomial) -> bool {
        let tolerance = 1e-10;
        let simplified_self = self.simplified();
        let simplified_other = other.simplified();
        if simplified_self.0.len() != simplified_other.0.len() {
            return false;
        }
        for (element1, element2) in simplified_self.0.iter().zip(simplified_other.0.iter()) {
            if (element1.c - element2.c).abs() > tolerance
                || (element1.e - element2.e).abs() > tolerance
            {
                return false;
            }
        }
        true
    }
}

impl Default for Polynomial {
    /// Defaults to an empty polynomial.
    ///
    /// This is equivalent to the polynomial `0`.
    ///
    /// #### Example
    /// ```rust
    /// use calcucalc::Polynomial;
    ///
    /// let p = Polynomial::default();
    /// assert_eq!(p, Polynomial(vec![]));
    /// assert_eq!(p.0.len(), 0);
    /// ```
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Tests for the Monomial struct.

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

    // Tests for the Polynomial struct.

    #[test]
    fn test_polynomial_identity() {
        let p1 = Polynomial(vec![Monomial { c: 1_f64, e: 1_f64 }]);
        let p2 = Polynomial(vec![Monomial { c: 1_f64, e: 1_f64 }]);
        assert_eq!(p1, p2);

        let p3 = Polynomial(vec![Monomial { c: 1_f64, e: 1_f64 }]);
        let p4 = Polynomial(vec![Monomial { c: 1_f64, e: 0_f64 }]);
        assert_ne!(p3, p4);

        let p5 = Polynomial(vec![Monomial { c: 1_f64, e: 1_f64 }]);
        let p6 = Polynomial(vec![Monomial { c: 0_f64, e: 1_f64 }]);
        assert_ne!(p5, p6);

        let p7 = Polynomial(vec![Monomial { c: 1_f64, e: 1_f64 }]);
        let p8 = Polynomial(vec![Monomial { c: 0_f64, e: 0_f64 }]);
        assert_ne!(p7, p8);

        let p9 = Polynomial(vec![Monomial { c: 1_f64, e: 1_f64 }]);
        let p10 = Polynomial(vec![
            Monomial { c: 1_f64, e: 1_f64 },
            Monomial { c: 1_f64, e: 1_f64 },
        ]);
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
    fn test_value_polynomial() {
        let p1 = Polynomial(vec![
            Monomial { c: 1_f64, e: 2_f64 },
            Monomial { c: 2_f64, e: 1_f64 },
        ]);
        assert_eq!(p1.value(2.0), 8.0);
        assert_eq!(p1.value(3.0), 15.0);
    }

    #[test]
    fn test_simplified() {
        let p1 = Polynomial(vec![
            Monomial { c: 1_f64, e: 1_f64 },
            Monomial { c: 2_f64, e: 1_f64 },
        ]);
        let p2 = Polynomial(vec![Monomial { c: 3_f64, e: 1_f64 }]);
        assert_eq!(p2, p1.simplified());

        let p3 = Polynomial(vec![
            Monomial { c: 1_f64, e: 1_f64 },
            Monomial { c: 2_f64, e: 1_f64 },
            Monomial { c: 3_f64, e: 1_f64 },
        ]);
        let p4 = Polynomial(vec![Monomial { c: 6_f64, e: 1_f64 }]);
        assert_eq!(p4, p3.simplified());

        let p5 = Polynomial(vec![
            Monomial {
                c: -1_f64,
                e: 1_f64,
            },
            Monomial {
                c: -2_f64,
                e: 1_f64,
            },
            Monomial {
                c: -3_f64,
                e: 1_f64,
            },
            Monomial { c: 4_f64, e: 1_f64 },
        ]);
        let p6 = Polynomial(vec![Monomial {
            c: -2_f64,
            e: 1_f64,
        }]);
        assert_eq!(p6, p5.simplified());

        let p7 = Polynomial(vec![
            Monomial { c: 1_f64, e: 1_f64 },
            Monomial { c: 2_f64, e: 1_f64 },
            Monomial { c: 3_f64, e: 1_f64 },
            Monomial { c: 4_f64, e: 1_f64 },
            Monomial { c: 5_f64, e: 1_f64 },
        ]);
        let p8 = Polynomial(vec![Monomial {
            c: 15_f64,
            e: 1_f64,
        }]);
        assert_eq!(p8, p7.simplified());

        let p9 = Polynomial(vec![
            Monomial { c: 1_f64, e: 1_f64 },
            Monomial { c: 2_f64, e: 1_f64 },
            Monomial { c: 3_f64, e: 1_f64 },
            Monomial { c: 4_f64, e: 1_f64 },
            Monomial { c: 5_f64, e: 1_f64 },
            Monomial { c: 6_f64, e: 1_f64 },
        ]);
        let p10 = Polynomial(vec![Monomial {
            c: 21_f64,
            e: 1_f64,
        }]);
        assert_eq!(p10, p9.simplified());

        let p11 = Polynomial(vec![
            Monomial { c: 1_f64, e: 1_f64 },
            Monomial { c: 2_f64, e: 1_f64 },
            Monomial { c: 3_f64, e: 1_f64 },
            Monomial { c: 4_f64, e: 1_f64 },
            Monomial { c: 5_f64, e: 1_f64 },
            Monomial { c: 6_f64, e: 1_f64 },
            Monomial { c: 7_f64, e: 1_f64 },
        ]);
        let p12 = Polynomial(vec![Monomial {
            c: 28_f64,
            e: 1_f64,
        }]);
        assert_eq!(p12, p11.simplified());

        let p13 = Polynomial(vec![
            Monomial {
                c: 1_f64,
                e: -16_f64,
            },
            Monomial {
                c: -2_f64,
                e: -1_f64,
            },
            Monomial { c: 3_f64, e: 1_f64 },
            Monomial {
                c: -344_f64,
                e: -50_f64,
            },
            Monomial {
                c: 5_f64,
                e: 30_f64,
            },
            Monomial {
                c: -5_f64,
                e: 30_f64,
            },
            Monomial { c: 7_f64, e: 1_f64 },
            Monomial { c: 8_f64, e: 1_f64 },
        ]);
        let p14 = Polynomial(vec![
            Monomial {
                c: 0_f64,
                e: 30_f64,
            },
            Monomial {
                c: 18_f64,
                e: 1_f64,
            },
            Monomial {
                c: -2_f64,
                e: -1_f64,
            },
            Monomial {
                c: 1_f64,
                e: -16_f64,
            },
            Monomial {
                c: -344_f64,
                e: -50_f64,
            },
        ]);
        assert_eq!(p14.simplified(), p13.simplified());

        let p15 = Polynomial(vec![
            Monomial {
                c: 1_f64,
                e: -10_f64,
            },
            Monomial {
                c: 2_f64,
                e: 31_f64,
            },
            Monomial {
                c: 3_f64,
                e: 14_f64,
            },
            Monomial {
                c: 4_f64,
                e: 94_f64,
            },
            Monomial {
                c: 5_f64,
                e: 15_f64,
            },
            Monomial { c: 6_f64, e: 0_f64 },
            Monomial { c: 7_f64, e: 0_f64 },
            Monomial {
                c: 8_f64,
                e: 31_f64,
            },
            Monomial { c: 9_f64, e: 0_f64 },
        ]);
        let p16 = Polynomial(vec![
            Monomial {
                c: 4_f64,
                e: 94_f64,
            },
            Monomial {
                c: 10_f64,
                e: 31_f64,
            },
            Monomial {
                c: 5_f64,
                e: 15_f64,
            },
            Monomial {
                c: 3_f64,
                e: 14_f64,
            },
            Monomial {
                c: 22_f64,
                e: 0_f64,
            },
            Monomial {
                c: 1_f64,
                e: -10_f64,
            },
        ]);
        assert_eq!(p15.simplified(), p16.simplify_by_combining_alike_powers());

        let p17 = Polynomial(vec![
            Monomial {
                c: 1.5_f64,
                e: -10_f64,
            },
            Monomial {
                c: 2_f64,
                e: 0.5_f64,
            },
            Monomial {
                c: 3_f64,
                e: 0.5_f64,
            },
            Monomial {
                c: 4_f64,
                e: 94_f64,
            },
            Monomial {
                c: 5_f64,
                e: -0.5_f64,
            },
            Monomial { c: 6_f64, e: 0_f64 },
            Monomial { c: 7_f64, e: 0_f64 },
            Monomial {
                c: 8_f64,
                e: -0.5_f64,
            },
            Monomial { c: 9_f64, e: 0_f64 },
        ]);
        let p18 = Polynomial(vec![
            Monomial {
                c: 4_f64,
                e: 94_f64,
            },
            Monomial {
                c: 5_f64,
                e: 0.5_f64,
            },
            Monomial {
                c: 22_f64,
                e: 0_f64,
            },
            Monomial {
                c: 13_f64,
                e: -0.5_f64,
            },
            Monomial {
                c: 1.5_f64,
                e: -10_f64,
            },
        ]);
        assert_eq!(p17.simplified(), p18);

        let p19 = Polynomial(vec![
            Monomial {
                c: -1.5_f64,
                e: -10_f64,
            },
            Monomial {
                c: 2_f64,
                e: 0.5_f64,
            },
            Monomial {
                c: -3_f64,
                e: 0.5_f64,
            },
            Monomial {
                c: -4_f64,
                e: 94_f64,
            },
            Monomial {
                c: -5_f64,
                e: -0.5_f64,
            },
            Monomial { c: 6_f64, e: 0_f64 },
            Monomial {
                c: 7_f64,
                e: -0_f64,
            },
            Monomial {
                c: 8_f64,
                e: -0.5_f64,
            },
            Monomial { c: 9_f64, e: 0_f64 },
        ]);
        let p20 = Polynomial(vec![
            Monomial {
                c: -4_f64,
                e: 94_f64,
            },
            Monomial {
                c: -1_f64,
                e: 0.5_f64,
            },
            Monomial {
                c: 22_f64,
                e: 0_f64,
            },
            Monomial {
                c: 3_f64,
                e: -0.5_f64,
            },
            Monomial {
                c: -1.5_f64,
                e: -10_f64,
            },
        ]);
        assert_eq!(p19.simplified(), p20);
    }

    #[test]
    fn test_add_polynomial() {
        let p1 = Polynomial(vec![Monomial { c: 1_f64, e: 1_f64 }]);
        let p2 = Polynomial(vec![Monomial { c: 2_f64, e: 1_f64 }]);
        let p3 = Polynomial(vec![Monomial { c: 3_f64, e: 1_f64 }]);
        assert_eq!(p3, p1.add_polynomial(p2));

        let p4 = Polynomial(vec![
            Monomial {
                c: -1.5_f64,
                e: -1.8_f64,
            },
            Monomial { c: 2_f64, e: 1_f64 },
        ]);
        let p5 = Polynomial(vec![
            Monomial { c: 3_f64, e: 1_f64 },
            Monomial {
                c: 1_f64,
                e: -1.8_f64,
            },
        ]);
        let p6 = Polynomial(vec![
            Monomial { c: 5_f64, e: 1_f64 },
            Monomial {
                c: -0.5,
                e: -1.8_f64,
            },
        ]);
        assert_eq!(p6, p4.add_polynomial(p5));

        let p7 = Polynomial(vec![
            Monomial { c: 1_f64, e: 1_f64 },
            Monomial {
                c: 2_f64,
                e: -1_f64,
            },
            Monomial {
                c: 3_f64,
                e: 0.5_f64,
            },
        ]);
        let p8 = Polynomial(vec![
            Monomial { c: 4_f64, e: 1_f64 },
            Monomial {
                c: 5_f64,
                e: -1_f64,
            },
            Monomial {
                c: 6_f64,
                e: 0.5_f64,
            },
        ]);
        let p9 = Polynomial(vec![
            Monomial { c: 5_f64, e: 1_f64 },
            Monomial {
                c: 9_f64,
                e: 0.5_f64,
            },
            Monomial {
                c: 7_f64,
                e: -1_f64,
            },
        ]);
        assert_eq!(p9, p7.add_polynomial(p8));
    }

    #[test]
    fn test_multiply_polynomial() {
        let p1 = Polynomial(vec![Monomial { c: 1_f64, e: 1_f64 }]);
        let p2 = Polynomial(vec![Monomial { c: 2_f64, e: 1_f64 }]);
        let p3 = Polynomial(vec![Monomial { c: 2_f64, e: 2_f64 }]);
        assert_eq!(p3, p1.multiply_polynomial(p2));

        let p4 = Polynomial(vec![
            Monomial {
                c: -1.5_f64,
                e: -1.8_f64,
            },
            Monomial { c: 2_f64, e: 1_f64 },
        ]);
        let p5 = Polynomial(vec![
            Monomial { c: 3_f64, e: 1_f64 },
            Monomial {
                c: 1_f64,
                e: -1.8_f64,
            },
        ]);
        let p6 = Polynomial(vec![
            Monomial { c: 6_f64, e: 2_f64 },
            Monomial {
                c: -2.5_f64,
                e: -0.8_f64,
            },
            Monomial {
                c: -1.5_f64,
                e: -3.6_f64,
            },
        ]);
        assert_eq!(p6, p4.multiply_polynomial(p5));
    }

    #[test]
    fn test_derivative() {
        let p1 = Polynomial(vec![Monomial { c: 1_f64, e: 1_f64 }]);
        assert_eq!(
            Polynomial(vec![Monomial { c: 1_f64, e: 0_f64 }]),
            p1.derivative()
        );

        let p2 = Polynomial(vec![
            Monomial {
                c: 1_f64,
                e: 1.5_f64,
            },
            Monomial { c: 3_f64, e: 1_f64 },
            Monomial { c: 2_f64, e: 0_f64 },
            Monomial {
                c: -0.7_f64,
                e: -3_f64,
            },
        ]);
        assert!(
            Polynomial(vec![
                Monomial {
                    c: 1.5_f64,
                    e: 0.5_f64
                },
                Monomial { c: 3_f64, e: 0_f64 },
                Monomial {
                    c: 2.1_f64,
                    e: -4_f64
                },
            ])
            .is_equal_within_tolerance_to(p2.derivative())
        );

        // ADD MORE TESTS HERE...
    }

    #[test]
    fn test_is_equal_within_tolerance_to() {
        // CODE GOES HERE...
        let p1 = Polynomial(vec![Monomial { c: 1_f64, e: 1_f64 }]);
        let p2 = Polynomial(vec![Monomial { c: 1_f64, e: 1_f64 }]);
        assert!(p1.is_equal_within_tolerance_to(p2));

        let p3 = Polynomial(vec![Monomial { c: 1_f64, e: 1_f64 }]);
        let p4 = Polynomial(vec![Monomial { c: 1_f64, e: 0_f64 }]);
        assert!(!p3.is_equal_within_tolerance_to(p4));

        let p5 = Polynomial(vec![Monomial {
            c: -246_f64,
            e: 0.45_f64,
        }]);
        let p6 = Polynomial(vec![Monomial {
            c: -246_f64,
            e: 0.45_f64,
        }]);
        assert!(p5.is_equal_within_tolerance_to(p6));

        let p7 = Polynomial(vec![Monomial { c: 1_f64, e: 1_f64 }]);
        let p8 = Polynomial(vec![Monomial {
            c: 1.00000000001_f64,
            e: 1.00000000001_f64,
        }]);
        assert!(p7.is_equal_within_tolerance_to(p8));

        let p9 = Polynomial(vec![Monomial { c: 1_f64, e: 1_f64 }]);
        let p10 = Polynomial(vec![Monomial {
            c: 0.99999999999_f64,
            e: 0.99999999999_f64,
        }]);
        assert!(p9.is_equal_within_tolerance_to(p10));
    }

    #[test]
    fn test_nth_derivative() {
        let p1 = Polynomial(vec![
            Monomial { c: 1_f64, e: 4_f64 },
            Monomial {
                c: -6_f64,
                e: 3_f64,
            },
            Monomial { c: 2_f64, e: 2_f64 },
            Monomial { c: 3_f64, e: 1_f64 },
        ]);
        assert_eq!(
            p1.nth_derivative(3),
            Polynomial(vec![
                Monomial { c: 24.0, e: 1.0 },
                Monomial { c: -36.0, e: 0.0 }
            ])
        );

        let p2 = Polynomial(vec![
            Monomial {
                c: 540.354,
                e: 64.3,
            },
            Monomial { c: 0.9, e: 64.0 },
            Monomial { c: 0.0, e: 63.0 },
            Monomial { c: 0.0, e: 5.0 },
        ]);
        assert_eq!(
            p2.nth_derivative(15),
            Polynomial(vec![
                Monomial {
                    c: 1.2200689110119749e29,
                    e: 49.3
                },
                Monomial {
                    c: 1.8773901659652704e26,
                    e: 49.0
                },
            ])
        );
        assert_eq!(
            p2.nth_derivative(64),
            Polynomial(vec![
                Monomial {
                    c: 2.66837419180906e92,
                    e: 0.29999999999999716
                },
                Monomial {
                    c: 1.1419823896729576e89,
                    e: 0.0
                }
            ])
        );

        let p3 = Polynomial(vec![
            Monomial {
                c: -540.354,
                e: -64.3,
            },
            Monomial { c: 0.9, e: -64.0 },
            Monomial { c: 0.0, e: 63.0 },
            Monomial { c: 0.0, e: 5.0 },
        ]);
        assert_eq!(
            p3.nth_derivative(15),
            Polynomial(vec![
                Monomial {
                    c: -5.140628626036299e27,
                    e: -79.0
                },
                Monomial {
                    c: 3.288681130950845e30,
                    e: -79.3
                },
            ])
        );
    }
}
