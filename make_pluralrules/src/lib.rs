#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate quote;
extern crate clap;
extern crate cldr_pluralrules_parser;
extern crate proc_macro2;

mod parser;

use parser::plural_category::PluralCategory;
use parser::resource::*;
use proc_macro2::TokenStream;

pub fn generate_rs(cldr_json: &str) -> String {
    let resources = parse_plurals_resource_from_string(cldr_json);

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

    // Call gen_rs to get Rust code. Convert TokenStream to string for file out.
    parser::gen_rs::gen_fn(rule_tokens).to_string()
}
