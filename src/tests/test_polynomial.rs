#[cfg(test)]
mod test_polynomial {
    use crate::{Monomial, Polynomial};

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

        let p3 = Polynomial(vec![Monomial {
            c: 4_f64,
            e: -1.5_f64,
        }]);
        assert_eq!(
            Polynomial(vec![Monomial {
                c: -6_f64,
                e: -2.5_f64
            }]),
            p3.derivative()
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
