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
//! Overall, this library relies heavily on the use of the `f64` type for the sake of flexibility and generality.
//!

/// A monomial is a product of a coefficient and an exponent of x.
/// For example, in the monomial `3x^2`, the coefficient is `3` and the exponent of x is `2`.
/// The monomial `3x^2` can be represented as a struct with the coefficient `3` and the exponent `2`. Using the calcucalc library, this monomial would be represented in this way
/// ```rust
/// use calcucalc::Monomial;
///
/// let three_x_squared = Monomial { c: 3.0, e: 2.0 }; // 3x^2
/// ```
///
/// One of the beautiful things about the monomial is its flexibility. For example, we can represent a **fraction** like `5/x` as a monomial. This is because `5/x` is equivalent to `5 * x^-1`. In this case, the coefficient is `5` and the exponent of x is `-1`.
///
/// ```rust
/// use calcucalc::Monomial;
///
/// let five_divided_by_x = Monomial { c: 5.0, e: -1.0 }; // 5/x
/// ```
///
/// We can also represent a **constant** as a monomial. For example, the number `1` can be represented as `1 * x^0`. In this case, the coefficient is `1` and the exponent of x is `0`.
///
/// ```rust
/// use calcucalc::Monomial;
///
/// let one = Monomial { c: 1.0, e: 0.0 }; // 1
/// ```
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
/// | `2x^π` | `Monomial { c: 2.0, e: std::f64::consts::PI }` |
/// | `3x^e` | `Monomial { c: 3.0, e: std::f64::consts::E }` |
///
/// #### Accessing monomial fields
///
/// The coefficient and exponent of a monomial can be accessed using the `c` and `e` fields, respectively.
///
/// ```rust
/// use calcucalc::Monomial;
///
/// let m = Monomial { c: 1.0, e: 2.0 };
/// assert_eq!(m.c, 1.0);
/// assert_eq!(m.e, 2.0);
/// ```
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

    /// Checks whether the values of a given interval of a polynomial overall is increasing, decreasing, staying constant, or is undefined.
    /// The interval is defined by the start and end values.
    ///
    /// #### Example
    /// ```rust
    /// use calcucalc::{Monomial, Polynomial};
    ///
    /// let my_polynomial = Polynomial(vec![
    ///     Monomial { c: 1.0, e: 2.0 },
    ///     Monomial { c: -2.0, e: 1.0 },
    ///     Monomial { c: 1.0, e: 0.0 },
    /// ]);
    /// assert_eq!(my_polynomial.trend_over_interval(-1.0, 1.0), "decreasing");
    /// assert_eq!(my_polynomial.trend_over_interval(1.0, 2.0), "increasing");
    /// assert_eq!(my_polynomial.trend_over_interval(-1.0, 0.0), "decreasing");
    /// assert_eq!(my_polynomial.trend_over_interval(0.0, 2.0), "constant");
    /// ```
    ///
    /// While it is recommended to order the start and end x-values in ascending order, this function will automatically swap them if they are not.
    pub fn trend_over_interval(&self, start: f64, end: f64) -> String {
        // Validate the start and end x-values are in the correct order,
        // and swap them if they are not.
        let mut start_x = start;
        let mut end_x = end;
        if start_x > end_x {
            std::mem::swap(&mut start_x, &mut end_x);
        }

        let start_value = self.value(start_x);
        let end_value = self.value(end_x);
        if start_value < end_value {
            "increasing".to_string()
        } else if start_value > end_value {
            "decreasing".to_string()
        } else if start_value == end_value {
            "constant".to_string()
        } else {
            "undefined".to_string()
        }
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
mod tests;
