#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate quote;
extern crate proc_macro2;
extern crate syn;
extern crate cldr_pluralrules_parser;
extern crate reqwest;

mod parser;

use std::env;
use std::path::Path;
use parser::resource::*;
use parser::gen_pr::*;
use parser::gen_rs::*;
use parser::plural_category::PluralCategory;
use cldr_pluralrules_parser::*;
use cldr_pluralrules_parser::ast::*;
use proc_macro2::TokenStream;

use std::fs::File;
use std::io::prelude::*;

use quote::ToTokens;


/// Use Command: `cargo run <output_file>`
fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();

    let body = reqwest::get("https://raw.githubusercontent.com/unicode-cldr/cldr-core/master/supplemental/plurals.json").unwrap().text().unwrap().to_string();

    let resources = if args.len() == 2 {
        parse_plurals_resource_from_string(&body)
    } else {
        panic!("Specify an output file path")
    };

    let r = resources.unwrap().supplemental.plurals_type_cardinal;

    let mut five = Vec::<TokenStream>::new();

    if let Some(rules) = r {
    	for (lang1, r) in rules {
            let lang = str::replace(&lang1, "-", "");

            let mut these_rules = Vec::<(PluralCategory, syn::Expr)>::new();

            for (rule_name, rule_line) in r {

                let short_name_arr = rule_name.split("-").collect::<Vec<_>>();
                let representation = parse_plural_rule(&rule_line);

                let cat = 
                    if short_name_arr[2] == "zero" {
                        PluralCategory::ZERO
                    } else if short_name_arr[2] == "one" {
                        PluralCategory::ONE
                    } else if short_name_arr[2] == "two" {
                        PluralCategory::TWO
                    } else if short_name_arr[2] == "few" {
                        PluralCategory::FEW
                    } else if short_name_arr[2] == "many" {
                        PluralCategory::MANY
                    } else {
                        PluralCategory::OTHER
                    };

                if cat != PluralCategory::OTHER {
                    let synxpr = gen_pr(representation);
                    these_rules.push((cat, synxpr));
                }
            }
            let oth = (PluralCategory::OTHER, other());
            these_rules.push(oth);
            five.push(gen_mid(&lang, these_rules)); 
    	}
    } 

    let six = gen_fn(five);

    let mut file = File::create(&args[1])?;
    file.write(six.to_string().as_bytes())?;
    Ok(())
}
