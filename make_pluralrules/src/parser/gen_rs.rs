//! gen_rs is a Rust code generator for expression representations of CLDR plural rules.
use super::plural_category::PluralCategory;

use proc_macro2::{Literal, TokenStream};

fn convert_litstr(s: &str) -> Literal {
    Literal::string(s)
}

fn create_match_state(lang: &str, filling: TokenStream) -> TokenStream {
    let match_name = convert_litstr(lang);
    quote! { #match_name => |po| { #filling }}
}

fn create_gen_pr_fn(filling: TokenStream) -> TokenStream {
    let ignore_noncritical_errors = quote! { #![allow(unused_variables, unused_parens)] };
    let extern_crates = quote! { extern crate matches; };
    let use_statements = quote! { use super::operands::PluralOperands; use super::PluralCategory; };
    let plural_function = quote! { type PluralRule = fn(PluralOperands) -> PluralCategory; };
    let head = quote! { #ignore_noncritical_errors #extern_crates #use_statements #plural_function};
    let get_pr_function =
        quote! { pub fn get_pr(lang: &str) -> PluralRule {match lang { #filling }} };
    quote! { #head #get_pr_function }
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

/// Generates the complete TokenStream for the generated Rust code. This wraps the head and tail of the .rs file around the generated CLDR expressions.
pub fn gen_fn(mut streams: Vec<TokenStream>) -> TokenStream {
    // Add an unknown local result to locale match
    streams.push(quote! { _ => panic!("Unknown locale!") });
    // Unpack the vector of tokenstreams. Each tokenstream is a pluralrule match result
    let unpacked_tokens = quote!{ #(#streams),* };
    // wrap the match options in the outermost gen code
    create_gen_pr_fn(unpacked_tokens)
}
