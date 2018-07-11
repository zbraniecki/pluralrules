use std::isize;
use std::str::FromStr;

#[derive(Debug, PartialEq)]
pub struct PluralOperands {
    pub n: f64,
    pub i: isize,
    pub v: isize,
    pub w: isize,
    pub f: isize,
    pub t: isize,
}

impl PluralOperands {
    pub fn new<S: ToString>(num: S) -> Self {
        let mut str_num: String = num.to_string();

        if str_num.starts_with("-") {
            str_num.remove(0);
        }

        let absolute_value = f64::from_str(&str_num).expect("Incorrect number passed!");

        let v: Vec<&str> = str_num.split('.').collect();

        let int_str = v.get(0).unwrap();
        let dec_str = v.get(1).unwrap_or(&"");

        let integer_digits =
            isize::from_str(int_str).expect("Failed to parse the integer to isize");

        let (num_fraction_digits0, num_fraction_digits, fraction_digits0, fraction_digits) =
            if dec_str.is_empty() {
                (0, 0, 0, 0)
            } else {
                let backtrace = dec_str.trim_right_matches('0');
                (
                    dec_str.chars().count() as isize,
                    backtrace.chars().count() as isize,
                    isize::from_str(dec_str).unwrap(),
                    isize::from_str(&backtrace).unwrap_or(0),
                )
            };

        PluralOperands {
            n: absolute_value,
            i: integer_digits,
            v: num_fraction_digits0,
            w: num_fraction_digits,
            f: fraction_digits0,
            t: fraction_digits,
        }
    }
}
