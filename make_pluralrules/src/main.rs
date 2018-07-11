#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate quote;
extern crate proc_macro2;
extern crate syn;
extern crate cldr_pluralrules_parser;

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

use quote::ToTokens;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        panic!("Specify a file path");
    } 
    // } else if args.len() < 3 {
    //     panic!("Specify a source and output file path");
    // }

    let path_string = &args[1];
    println!("{:?}", path_string);

    let file_path = Path::new(&path_string);
    let resources = parse_plurals_resource(file_path);

    let r = resources.unwrap().supplemental.plurals_type_cardinal;

    // let function = || { println!("test") };

    // function();

    let mut five = Vec::<TokenStream>::new();

    if let Some(rules) = r {
    	for (lang1, r) in rules {
            // println!("\n\nRules for lang {:#?}\n", &lang1);

            //? ANY LANGUAE WITH A `-` IN THE NAME HAS THIS SYMBOL REMOVED
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


            // println!("{:?}", five.into_token_stream().to_string());
            // println!("{:?}", five.to_string())
 
    	}
    } 

    let six = gen_fn(five);

}
