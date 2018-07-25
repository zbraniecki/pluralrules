//! make_pluralrules generates a Rust code representation of CLDR plural rules in compliance with [Unicode](http://unicode.org/reports/tr35/tr35-numbers.html#Language_Plural_Rules).
//!
//! Representations of plural rules are generated from [Unicode's plural rules](https://github.com/unicode-cldr/cldr-core/blob/master/supplemental/plurals.json) and uses the intl_pluralrules_parser AST to build the representation.
//!
//! The ouput is a Rust file, specified by the user in the comand
//! ```text
//! cargo run <./path/to/cldr.json> <./path/to/output.rs>
//! ```
//! where `<output_file>` is the location of the desired Rust file.
extern crate clap;
extern crate make_pluralrules;

use clap::App;
use make_pluralrules::generate_rs;

use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let matches = App::new("CLDR Plura Rules Rust Generator")
        .version("0.1.0")
        .about("Generates Rust code for CLDR plural rules.")
        .args_from_usage(
            "<CLDR_JSON> 'Sets the input file to use'
            <OUTPUT_RS> 'Sets the output file to use'",
        )
        .get_matches();

    let cldr_path = matches.value_of("CLDR_JSON").unwrap();
    let output_path = matches.value_of("OUTPUT_RS").unwrap();

    let mut f = File::open(cldr_path).expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let complete_rs_code = generate_rs(&contents);

    let mut file = File::create(output_path)?;
    file.write(complete_rs_code.as_bytes())?;
    Ok(())
}
