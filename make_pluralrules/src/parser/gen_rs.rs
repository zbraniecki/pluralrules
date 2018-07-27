//! gen_rs is a Rust code generator for expression representations of CLDR plural rules.
use super::plural_category::PluralCategory;
use proc_macro2::{Literal, TokenStream};
use std::collections::BTreeMap;

fn convert_litstr(s: &str) -> Literal {
    Literal::string(s)
}

fn create_match_state(lang: &str, filling: TokenStream) -> TokenStream {
    let match_name = convert_litstr(lang);
    quote! { #match_name => Ok(|po| { #filling }) }
}

/// Generates the complete TokenStream for the generated Rust code. This wraps the head and tail of the .rs file around the generated CLDR expressions.
pub fn gen_fn(
    streams: BTreeMap<String, Vec<TokenStream>>,
    locales: BTreeMap<String, Vec<String>>,
    vr: String,
) -> TokenStream {
    let ignore_noncritical_errors = quote! { #![allow(unused_variables, unused_parens)] };
    let extern_crates = quote! { extern crate matches; };
    let use_statements = quote! { use super::operands::PluralOperands; use super::PluralCategory; };
    let plural_function = quote! { pub type PluralRule = fn(PluralOperands) -> PluralCategory; };
    let pr_type = quote! { pub enum PluralRuleType { ORDINAL, CARDINAL } };
    let num: isize = vr.parse().unwrap();
    let ver = Literal::u64_unsuffixed(num as u64);
    let version = quote! { pub static CLDR_VERSION: usize = #ver; };
    let get_locales = gen_get_locales(locales);
    let head = quote! { #ignore_noncritical_errors #extern_crates #use_statements #pr_type #plural_function #version #get_locales };
    let mut tokens = Vec::<TokenStream>::new();
    for (pr_type, stream) in streams {
        tokens.push(create_gen_pr_type_fn(&pr_type, stream));
    }
    let filling = quote!{ #(#tokens),* };
    let get_pr_function =
        quote! { pub fn get_pr(lang_code: &str, pr_type: PluralRuleType) -> Result<PluralRule, ()> {match pr_type { #filling }} };
    quote! { #head #get_pr_function }
}

fn gen_get_locales(locales: BTreeMap<String, Vec<String>>) -> TokenStream {
    let mut tokens = Vec::<TokenStream>::new();

    for (pr_type, locales) in locales {
        let match_name = match pr_type.as_str() {
            "cardinal" => quote! { PluralRuleType::CARDINAL },
            "ordinal" => quote! { PluralRuleType::ORDINAL },
            _ => panic!("Unknown plural rule type"),
        };
        let locales_tokens = quote! { &[ #(#locales),* ] };
        tokens.push(quote! { #match_name => #locales_tokens });
    }
    quote! { pub fn get_locales(pr_type: PluralRuleType) -> &'static [&'static str] { match pr_type { #(#tokens),* } } }
}

fn create_gen_pr_type_fn(pr_type: &str, mut streams: Vec<TokenStream>) -> TokenStream {
    // Add an unknown local result to locale match
    streams.push(quote! { _ => Err(()) });
    // Unpack the vector of tokenstreams. Each tokenstream is a pluralrule match result
    let unpacked_tokens = quote!{ #(#streams),* };

    let match_name = match pr_type {
        "cardinal" => quote! { PluralRuleType::CARDINAL },
        "ordinal" => quote! { PluralRuleType::ORDINAL },
        _ => panic!("Unknown plural rule type"),
    };
    quote! { #match_name => match lang_code { #unpacked_tokens } }
}

fn create_return(cat: PluralCategory, exp: &TokenStream) -> TokenStream {
    match cat {
        PluralCategory::ZERO => quote! {if #exp { PluralCategory::ZERO } },
        PluralCategory::ONE => quote! {if #exp { PluralCategory::ONE } },
        PluralCategory::TWO => quote! {if #exp { PluralCategory::TWO } },
        PluralCategory::FEW => quote! {if #exp { PluralCategory::FEW } },
        PluralCategory::MANY => quote! {if #exp { PluralCategory::MANY } },
        PluralCategory::OTHER => quote! { { PluralCategory::OTHER } },
    }
}

/// Generates the match statements that comprise the majority of the generated rust code.
///
/// These statements are the expression representations of the CLDR plural rules.
pub fn gen_mid(lang: &str, pluralrule_set: Vec<(PluralCategory, TokenStream)>) -> TokenStream {
    // make pluralrule_set iterable
    let mut iter = pluralrule_set.iter();

    let queued = iter.next();
    let rule_tokens = match queued {
        Some(pair) => {
            // instantiate tokenstream for folded match rules
            let mut tokens = create_return(pair.0, &pair.1);

            // add all tokens to token stream, separated by commas
            for pair in iter {
                let condition = create_return(pair.0, &pair.1);
                tokens = quote! { #tokens else #condition };
            }
            tokens = quote! { #tokens else { PluralCategory::OTHER } };
            tokens
        }
        None => quote! { { PluralCategory::OTHER }  },
    };

    // return the world's best tokenstreamn
    create_match_state(lang, rule_tokens)
}
