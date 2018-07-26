// Still want to test
// Non-numeric input
// Empty Input

extern crate intl_pluralrules;

use intl_pluralrules::operands::*;

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
