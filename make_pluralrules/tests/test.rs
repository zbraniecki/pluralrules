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
    let cardinal_json = read_file("./tests/fixtures/cldr_pluralrules_cardinals_test.json")
        .expect("Could not read input json");
    let output_rs =
        read_file("./tests/fixtures/cldr_pluralrules_test.rs").expect("Could not read output rs");

    let output = generate_rs(&[cardinal_json]);

    assert_eq!(output_rs, output);
}
