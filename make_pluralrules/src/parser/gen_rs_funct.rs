extern crate syn;
extern crate proc_macro2;
extern crate quote;

use super::plural_category::PluralCategory;

use self::proc_macro2::{Span};

use self::syn::token;

use quote::ToTokens;
// use proc_macro2::{Ident, Span};

fn convert_ident(ch: &str) -> syn::Ident {
    syn::Ident::new(ch, Span::call_site())
}

fn create_boilerplate_from_quote(lang: &str) -> syn::File {
    // let locale = "en";
    let plural_type = "cardinal";
    // let fn_name = format!("pr_{}_{}", locale, plural_type);
    let fnname = "pr_".to_owned() + lang;
    let function_name = convert_ident(&fnname);
    let boilerplate_tokens = quote! {
        fn #function_name (po: PluralOperands) -> PluralCategory {}
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

pub fn gen_fn(lang: &str, ex : Vec<(PluralCategory, syn::Expr)> ) -> syn::File {


	// 1. Get the boilerplate AST
    let mut result = create_boilerplate_from_quote(lang);

    // 2. Get the `one` block
    for pair in ex {
        let block = create_return(pair.0, pair.1.clone());
        add_block(&mut result, block);
    }

	println!("{:#?}", result.clone().into_token_stream().to_string());

	result
}