extern crate make_pluralrules;

use make_pluralrules::generate_rs;

use std::fs::File;
use std::io;
use std::io::Read;

fn read_file(path: &str) -> Result<String, io::Error> {
    let mut f = try!(File::open(path));
    let mut s = String::new();
    try!(f.read_to_string(&mut s));
    Ok(s)
}

#[test]
fn full_cldr_test() {
    let cardinal_json = read_file("./tests/fixtures/cldr_pluralrules_cardinals_33.json")
        .expect("Could not read input json");
    let ordinal_json = read_file("./tests/fixtures/cldr_pluralrules_ordinals_33.json")
        .expect("Could not read input json");
    let output_rs =
        read_file("./tests/fixtures/cldr_pluralrules_33.rs").expect("Could not read output rs");

    let output = generate_rs(&[cardinal_json, ordinal_json]);

    assert_eq!(output_rs, output);
}

#[test]
fn within_test() {
    let text = String::from(
r#"{
  "supplemental": {
    "version": {
      "_number": "$Revision: 13898 $",
      "_unicodeVersion": "10.0.0",
      "_cldrVersion": "0"
    },
    "plurals-type-cardinal": {
      "test": {
        "pluralRule-count-one": "n = 1 @integer 1 @decimal 1.0, 1.00, 1.000, 1.0000",
        "pluralRule-count-two": "n % 10 within 1..2 @ 11, 12, 21, 22...",
        "pluralRule-count-few": "n within 2..10 and i not within 8..9 @ 1.1, 1.2, 2, 3, 4, 5, 6, 7, 10",
        "pluralRule-count-other": " @integer 0, 2~16, 100, 1000, 10000, 100000, 1000000, … @decimal 0.0~0.9, 1.1~1.6, 10.0, 100.0, 1000.0, 10000.0, 100000.0, 1000000.0, …"
      }
    }
  }
}"#);
    let output_rs = read_file("./tests/fixtures/cldr_pluralrules_within_test.rs")
        .expect("Could not read output rs");

    let output = generate_rs(&[text]);

    assert_eq!(output_rs, output);
}

#[test]
#[should_panic]
fn bad_type_test() {
    let text = String::from(
r#"{
  "supplemental": {
    "version": {
      "_number": "$Revision: 13898 $",
      "_unicodeVersion": "10.0.0",
      "_cldrVersion": "0"
    },
    "plurals-type-cardinals": {
      "test": {
        "pluralRule-count-one": "n = 1 @integer 1 @decimal 1.0, 1.00, 1.000, 1.0000",
        "pluralRule-count-other": " @integer 0, 2~16, 100, 1000, 10000, 100000, 1000000, … @decimal 0.0~0.9, 1.1~1.6, 10.0, 100.0, 1000.0, 10000.0, 100000.0, 1000000.0, …"
      }
    }
  }
}"#);

    generate_rs(&[text]);
}
