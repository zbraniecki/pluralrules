//! make_pluralrules generates a Rust code representation of CLDR plural rules in compliance with [Unicode](http://unicode.org/reports/tr35/tr35-numbers.html#Language_Plural_Rules).
//!
//! Representations of plural rules are generated from [Unicode's plural rules](https://github.com/unicode-cldr/cldr-core/blob/master/supplemental/plurals.json) and uses the intl_pluralrules_parser AST to build the representation.
//!
//! The ouput is a Rust file, specified by the user in the comand
//! ```text
//! cargo run -- -i <./path/to/cldr.json>... -o <./path/to/output.rs>
//! ```
#[macro_use]
extern crate serde_derive;
extern crate serde;
extern crate serde_json;
#[macro_use]
extern crate quote;
extern crate clap;
extern crate cldr_pluralrules_parser;
extern crate phf_codegen;
extern crate proc_macro2;

mod parser;

use parser::plural_category::PluralCategory;
use parser::resource::*;
use proc_macro2::TokenStream;
use std::collections::BTreeMap;

/// Takes a string representation of a CLDR JSON file and produces a string representation of the generated Rust code for the plural rules.
///
/// The string representation of the Rust code is written to a specified Rust file and can be used to get the plural category for numerical input.
pub fn generate_rs(cldr_jsons: &[String]) -> String {
    let mut cldr_version = None;
    let mut tokens = BTreeMap::new();
    let mut locales = BTreeMap::new();

    for cldr_json in cldr_jsons {
        // resource_items is a struct representation of the raw CLDR rules.
        let resource_items = parse_plurals_resource_from_string(cldr_json).unwrap();

        let res_cldr_version = resource_items.supplemental.version.cldr_version;

        if cldr_version.is_none() {
            cldr_version = Some(res_cldr_version);
        } else if cldr_version != Some(res_cldr_version) {
            panic!("All input resources must use the same CLDR version!");
        }

        if let Some(data) = resource_items.supplemental.plurals_type_cardinal {
            let (res_locales, rule_tokens) = gen_type_rs(data);
            if tokens.contains_key("cardinal") {
                panic!("Cannot provide two inputs with the same data!");
            }
            tokens.insert("cardinal".to_owned(), rule_tokens);
            locales.insert("cardinal".to_owned(), res_locales);
        }

        if let Some(data) = resource_items.supplemental.plurals_type_ordinal {
            let (res_locales, rule_tokens) = gen_type_rs(data);
            if tokens.contains_key("ordinal") {
                panic!("Cannot provide two inputs with the same data!");
            }
            tokens.insert("ordinal".to_owned(), rule_tokens);
            locales.insert("ordinal".to_owned(), res_locales);
        }
    }

    if cldr_version.is_none() || tokens.is_empty() {
        panic!("None of the input files provided core data!");
    }

    // Call gen_rs to get Rust code. Convert TokenStream to string for file out.
    parser::gen_rs::gen_fn(tokens, locales, &cldr_version.unwrap()).to_string()
}

fn gen_type_rs(
    rules: BTreeMap<String, BTreeMap<String, String>>,
) -> (Vec<String>, Vec<(String, TokenStream)>) {
    // rule_tokens is a vector of TokenStreams that represent the CLDR plural rules as Rust expressions.
    let mut rule_tokens = Vec::<(String, TokenStream)>::new();
    let mut langnames = Vec::<String>::new();

    for (lang, r) in rules {
        // this_lang_rules is a vector of plural rules saved as a PluralCategory and a TokenStream
        let mut this_lang_rules = Vec::<(PluralCategory, TokenStream)>::new();

        for (rule_name, rule_line) in r {
            // cat_name is the simplified category name from the CLDR source file
            let cat_name = rule_name.split('-').collect::<Vec<_>>()[2];

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
        rule_tokens.push((
            lang.clone(),
            parser::gen_rs::gen_mid(&lang, &this_lang_rules),
        ));
        langnames.push(lang);
    }
    (langnames, rule_tokens)
}
