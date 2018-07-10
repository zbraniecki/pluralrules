#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate quote;
extern crate syn;
extern crate cldr_pluralrules_parser;

mod parser;

use std::path::Path;
use parser::resource::*;
use parser::plural_category::PluralCategory;
use cldr_pluralrules_parser::*;
use cldr_pluralrules_parser::ast::*;

// use quote::ToTokens;

fn main() {
    let file_path = Path::new("resources/plurals.json");


    // How to use the resource getter:
    let resources = parse_plurals_resource(file_path);

    // println!("{:#?}", resources);

    let r = resources.unwrap().supplemental.plurals_type_cardinal;

    if let Some(rules) = r {
    	for (lang1, r) in rules {
    		// println!("Rules for lang {:#?}: {:#?}", lang, r);
            println!("\n\nRules for lang {:#?}\n", &lang1);

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

                if representation != Condition(vec![]) {

                    let synxpr = parser::gen_pr::gen_pr(representation);

                    these_rules.push((cat, synxpr.clone()));

                    // println!("{:?}", synxpr.into_token_stream().to_string());
                }



                // println!("{:#?}: {:#?}", &short_name_arr[2], rule_line);

                // println!("{:#?}", parse_plural_rule(&rule_line));
            }
            let five = parser::gen_rs_funct::gen_fn(&lang, these_rules);
    	}
    }    

    //println!("{:#?}", r);

    // for r2 in r {
    // 	for r3 in r2 {
    // 		println!("Look {:#?}", &r3);
    // 		 for r4 in r3 {
    // 		 	println!("Inside {:#?}", r4);
    // 		 }
    // 	}
    // } 

    // How to use the parser:
    // let parsed = parse_plural_rule("i = 5");
    // println!("{:#?}", parsed);
}
