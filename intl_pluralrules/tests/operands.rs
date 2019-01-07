// Still want to test
// Non-numeric input
// Empty Input

// use intl_pluralrules::operands::*;
use intl_pluralrules::{operands::*, IntlPluralRules, PluralRuleType};

#[test]
fn test_operands_from_str() {
    let tests = vec![
        ((0_f64, 0, 0, 0, 0, 0), "0"),
        ((2_f64, 2, 0, 0, 0, 0), "2"),
        ((57_f64, 57, 0, 0, 0, 0), "57"),
        ((987_f64, 987, 0, 0, 0, 0), "987"),
        ((1234567_f64, 1234567, 0, 0, 0, 0), "1234567"),
        ((10_f64, 10, 0, 0, 0, 0), "-10"),
        ((1000000_f64, 1000000, 0, 0, 0, 0), "-1000000"),
        ((0.23_f64, 0, 2, 2, 23, 23), "0.23"),
        ((0.230_f64, 0, 3, 2, 230, 23), "0.230"),
        ((23.00_f64, 23, 2, 0, 00, 0), "23.00"),
        ((0.0203000_f64, 0, 7, 4, 203000, 203), "0.0203000"),
        ((123.45_f64, 123, 2, 2, 45, 45), "123.45"),
        ((1234.567_f64, 1234, 3, 3, 567, 567), "-1234.567"),
    ];

    for test in tests {
        assert_eq!(
            Ok(PluralOperands {
                n: (test.0).0,
                i: (test.0).1,
                v: (test.0).2,
                w: (test.0).3,
                f: (test.0).4,
                t: (test.0).5,
            }),
            PluralOperands::from(test.1)
        );
    }
}

#[test]
fn test_operands_from_int() {
    let tests = vec![
        ((0_f64, 0, 0, 0, 0, 0), 0),
        ((2_f64, 2, 0, 0, 0, 0), 2),
        ((57_f64, 57, 0, 0, 0, 0), 57),
        ((987_f64, 987, 0, 0, 0, 0), 987),
        ((1234567_f64, 1234567, 0, 0, 0, 0), 1234567),
        ((10_f64, 10, 0, 0, 0, 0), -10),
        ((1000000_f64, 1000000, 0, 0, 0, 0), -1000000),
    ];

    for test in tests {
        assert_eq!(
            Ok(PluralOperands {
                n: (test.0).0,
                i: (test.0).1,
                v: (test.0).2,
                w: (test.0).3,
                f: (test.0).4,
                t: (test.0).5,
            }),
            PluralOperands::from(test.1)
        );
    }
}

#[test]
fn test_operands_from_float() {
    let tests = vec![
        ((0.23_f64, 0, 2, 2, 23, 23), 0.23),
        ((0.230_f64, 0, 2, 2, 23, 23), 0.230),
        ((0.0203000_f64, 0, 4, 4, 203, 203), 0.0203000),
        ((123.45_f64, 123, 2, 2, 45, 45), 123.45),
        ((1234.567_f64, 1234, 3, 3, 567, 567), -1234.567),
    ];

    for test in tests {
        assert_eq!(
            Ok(PluralOperands {
                n: (test.0).0,
                i: (test.0).1,
                v: (test.0).2,
                w: (test.0).3,
                f: (test.0).4,
                t: (test.0).5,
            }),
            PluralOperands::from(test.1)
        );
    }
}

#[test]
fn test_incorrect_operand() {
    assert_eq!(PluralOperands::from("foo").is_err(), true);
}

#[test]
fn test_get_locale() {
    let pr_naq = IntlPluralRules::create("naq", PluralRuleType::CARDINAL).unwrap();
    assert_eq!(pr_naq.get_locale(), "naq");
}

#[test]
fn custom_type() {
    use intl_pluralrules::{IntlPluralRules, PluralCategory, PluralRuleType};
    struct MyType {
        value: isize,
    }

    impl IntoPluralOperands for MyType {
        fn into_plural(self) -> Result<PluralOperands, &'static str> {
            Ok(PluralOperands {
                n: self.value as f64,
                i: self.value as isize,
                v: 0,
                w: 0,
                f: 0,
                t: 0,
            })
        }
    }

    let pr = IntlPluralRules::create("en", PluralRuleType::CARDINAL).unwrap();
    let v = MyType { value: 5 };

    assert_eq!(pr.select(v), Ok(PluralCategory::OTHER));
}
