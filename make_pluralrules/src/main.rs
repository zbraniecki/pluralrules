//! make_pluralrules generates a Rust code representation of CLDR plural rules in compliance with [Unicode](http://unicode.org/reports/tr35/tr35-numbers.html#Language_Plural_Rules).
//! 
//! Representations of plural rules are generated from [Unicode's plural rules](https://github.com/unicode-cldr/cldr-core/blob/master/supplemental/plurals.json) and uses the intl_pluralrules_parser AST to build the representation.
//!
//! The ouput is a Rust file, specified by the user in the comand
//! ```text
//! cargo run <output_file>
//! ```
//! where `<output_file> is the location of the desired Rust file. 

#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate quote;
extern crate cldr_pluralrules_parser;
extern crate proc_macro2;
extern crate reqwest;

mod parser;

// use std::path::Path;
use parser::plural_category::PluralCategory;
use parser::resource::*;
use proc_macro2::TokenStream;

use std::env;
use std::fs::File;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let mut f = File::open("resources/plurals.json").expect("file not found");

    let mut contents = String::new();
    f.read_to_string(&mut contents)
        .expect("something went wrong reading the file");

    let resources = if args.len() == 2 {
        parse_plurals_resource_from_string(&contents)
    } else {
        panic!("Specify an output file path")
    };

    // resource_items is a struct representation of the raw CLDR rules.
    let resource_items = resources.unwrap().supplemental.plurals_type_cardinal;

    // rule_tokens is a vector of TokenStreams that represent the CLDR plural rules as Rust expressions.
    let mut rule_tokens = Vec::<TokenStream>::new();

    if let Some(rules) = resource_items {
        for (lang_code, r) in rules {
            // `-` cannot appear in a function name. This removes a Rust-breaking character.
            let lang = str::replace(&lang_code, "-", "");

            // this_lang_rules is a vector of plural rules saved as a PluralCategory and a TokenStream
            let mut this_lang_rules = Vec::<(PluralCategory, TokenStream)>::new();

            for (rule_name, rule_line) in r {
                // cat_name is the simplified category name from the CLDR source file
                let cat_name = rule_name.split("-").collect::<Vec<_>>()[2];

                // representation is the
                let representation = cldr_pluralrules_parser::parse_plural_rule(&rule_line);

                let cat = if cat_name == "zero" {
                    PluralCategory::ZERO
                } else if cat_name == "one" {
                    PluralCategory::ONE
                } else if cat_name == "two" {
                    PluralCategory::TWO
                } else if cat_name == "few" {
                    PluralCategory::FEW
                } else if cat_name == "many" {
                    PluralCategory::MANY
                } else {
                    PluralCategory::OTHER
                };

                // Only allow rules that are not `OTHER` to be added. `OTHER` can have no rules and is added outside of the loop.
                if cat != PluralCategory::OTHER {
                    let tokens = parser::gen_pr::gen_pr(representation);
                    this_lang_rules.push((cat, tokens));
                }
            }
            // convert language rules to TokenStream and add them to all the rules
            rule_tokens.push(parser::gen_rs::gen_mid(&lang, this_lang_rules));
        }
    }

    // I want this as a String rather than some weird struct
    let complete_rs_code = parser::gen_rs::gen_fn(rule_tokens);

    let mut file = File::create(&args[1])?;
    file.write(complete_rs_code.to_string().as_bytes())?;
    Ok(())
}
