extern crate syn;
extern crate proc_macro2;
extern crate quote;

use ast::*;

use self::proc_macro2::{Span};

use self::syn::token;

use quote::ToTokens;
// use proc_macro2::{Ident, Span};

fn create_boilerplate_from_quote() -> syn::File {
    let locale = "en";
    let plural_type = "cardinal";
    // let fn_name = format!("pr_{}_{}", locale, plural_type);
    let fnname = "pr_en";
    let boilerplate_tokens = quote! {
        fn pr_en (po: PluralOperands) -> PluralCategory {}
    };
    syn::parse2(boilerplate_tokens).expect("Unable to parse boilerplate")
}

fn create_return(cat : PluralCategory, exp : syn::Expr ) -> syn::Expr {
    let one_clause_tokens = 
        match cat {
            PluralCategory::ZERO => quote! {if #exp { return PluralCategory::ZERO } },
            PluralCategory::ONE => quote! {if #exp { return PluralCategory::ONE } },
            PluralCategory::TWO  => quote! {if #exp { return PluralCategory::TWO } },
            PluralCategory::FEW => quote! {if #exp { return PluralCategory::FEW } },
            PluralCategory::MANY => quote! {if #exp { return PluralCategory::MANY } },
            PluralCategory::OTHER => quote! {if #exp { return PluralCategory::OTHER } }
        };
    syn::parse2(one_clause_tokens).expect("Unable to parse tokens")
}

fn add_block(f: &mut syn::File, b: syn::Expr) {
    let func = &mut f.items[0];
    match func {
        syn::Item::Fn(ref mut f) => {
            let block = &mut f.block;
            let stmts = &mut block.stmts;
            stmts.push(syn::Stmt::Expr(b));
        },
        _ => panic!("Unknown boilerplate")
    };
}

pub fn gen_fn(cat : PluralCategory, ex : syn::Expr) -> syn::Expr {


    // 1. Get the boilerplate AST
    let mut result = create_boilerplate_from_quote();

    // 2. Get the `one` block
    let one = create_return(cat, ex.clone());
    add_block(&mut result, one);

    println!("{}", result.into_token_stream().to_string());

    ex
}

pub fn convert_to_rust(representation : Relation) -> syn::Expr {
    let cat = 
            if x[0] == "zero" {
                println!("Zero");
                PluralCategory::ZERO
            } else if x[0] == "one" {
                println!("One");
                PluralCategory::ONE
            } else if x[0] == "two" {
                PluralCategory::TWO
            } else if x[0] == "few" {
                PluralCategory::FEW
            } else if x[0] == "many" {
                PluralCategory::MANY
            } else {
                PluralCategory::OTHER
            };

    gen_rs_funct::gen_fn(cat, representation)
}