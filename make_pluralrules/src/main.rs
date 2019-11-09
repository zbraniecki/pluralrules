use clap::App;
use make_pluralrules::generate_rs;
use std::process::Command;

use std::fs;
use std::io::Write;

fn main() -> std::io::Result<()> {
    let matches = App::new("CLDR Plural Rules Rust Generator")
        .version("0.1.0")
        .about("Generates Rust code for CLDR plural rules.")
        .args_from_usage(
            "<input-files> -i, --input=<PATH>... 'Input CLDR JSON plural rules files'
             <output-file> -o, --output=<PATH> 'Output RS file'
             [ugly] -u, --ugly",
        )
        .get_matches();

    let input_paths: Vec<&str> = matches.values_of("input-files").unwrap().collect();

    let input_jsons = input_paths
        .iter()
        .map(|path| fs::read_to_string(path).expect("file not found"))
        .collect::<Vec<_>>();
    let complete_rs_code = generate_rs(&input_jsons);

    let output_path = matches.value_of("output-file").unwrap();
    let mut file = fs::File::create(output_path)?;
    file.write_all(complete_rs_code.as_bytes())?;

    if !matches.is_present("ugly") {
        Command::new("rustfmt")
            .args(&[output_path])
            .output()
            .expect("Failed to format the output using `rustfmt`");
    }

    Ok(())
}
