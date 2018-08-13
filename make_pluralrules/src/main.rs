extern crate clap;
extern crate make_pluralrules;

use clap::App;
use make_pluralrules::generate_rs;

use std::fs::File;
use std::io::prelude::*;
use std::process::Command;

fn main() -> std::io::Result<()> {
    let matches = App::new("CLDR Plural Rules Rust Generator")
        .version("0.1.0")
        .about("Generates Rust code for CLDR plural rules.")
        .args_from_usage(
            "<input-files> -i, --input=<PATH>... 'Input CLDR JSON plural rules files'
             <output-file> -o, --output=<PATH> 'Output RS file'",
        )
        .get_matches();

    let input_paths: Vec<&str> = matches.values_of("input-files").unwrap().collect();

    let input_jsons = input_paths
        .iter()
        .map(|path| {
            let mut f = File::open(path).expect("file not found");
            let mut contents = String::new();
            f.read_to_string(&mut contents)
                .expect("something went wrong reading the file");
            contents
        })
        .collect::<Vec<_>>();
    let complete_rs_code = generate_rs(&input_jsons);

    let output_path = matches.value_of("output-file").unwrap();
    let mut file = File::create(output_path)?;
    file.write_all(complete_rs_code.as_bytes())?;

    // Command::new("rustfmt")
    //     .args(&[output_path])
    //     .output()
    //     .expect("Failed to format the output using `rustfmt`");

    Ok(())
}
