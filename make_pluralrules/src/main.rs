#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate quote;
extern crate cldr_pluralrules_parser;
extern crate proc_macro2;
extern crate reqwest;

mod parser;

// use std::path::Path; // for reading local file
use parser::plural_category::PluralCategory;
use parser::resource::*;
use proc_macro2::TokenStream;

use std::env;
use std::fs::File;
use std::io::prelude::*;

// Use Command: `cargo run <output_file>`
fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let body = reqwest::get(
        "https://raw.githubusercontent.com/unicode-cldr/cldr-core/master/supplemental/plurals.json",
    ).unwrap()
        .text()
        .unwrap()
        .to_string();

    let resources = if args.len() == 2 {
        parse_plurals_resource_from_string(&body)
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
