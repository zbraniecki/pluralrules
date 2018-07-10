extern crate syn;
extern crate proc_macro2;
extern crate quote;

use super::plural_category::PluralCategory;

use self::proc_macro2::{Span};

use self::syn::token;

use quote::ToTokens;
use self::proc_macro2::TokenStream;
// use proc_macro2::{Ident, Span};

fn convert_ident(ch: &str) -> syn::Ident {
    syn::Ident::new(ch, Span::call_site())
}

fn create_boilerplate_from_quote(lang: &str) -> syn::File {
    let plural_type = "cardinal";
    let fnname = "pr_".to_owned() + lang;
    let function_name = convert_ident(&fnname);
    let boilerplate_tokens = quote! {
        fn #function_name (po: PluralOperands) -> PluralCategory {}
    };
    syn::parse2(boilerplate_tokens).expect("Unable to parse boilerplate")
}

fn create_return(cat: PluralCategory, exp: &syn::Expr ) -> TokenStream {
	match cat {
		PluralCategory::ZERO => quote! {if #exp { PluralCategory::ZERO } },
	    PluralCategory::ONE => quote! {if #exp { PluralCategory::ONE } },
	    PluralCategory::TWO  => quote! {if #exp { PluralCategory::TWO } },
	    PluralCategory::FEW => quote! {if #exp { PluralCategory::FEW } },
	    PluralCategory::MANY => quote! {if #exp { PluralCategory::MANY } },
	    PluralCategory::OTHER => quote! { { PluralCategory::OTHER } }
	}
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

    let size = ex.len();

    let mut result = create_boilerplate_from_quote(lang);
    let mut iter = ex.iter();
    let pair = iter.next().unwrap();
    let mut tokens = create_return(pair.0, &pair.1);
    
    if size > 1 {
        for pair in iter {
            let condition = create_return(pair.0, &pair.1);
            tokens = quote! { #tokens else #condition };
        }
    }
    let  my_end_expr = syn::parse2(tokens).expect("WITH MY TOKES Unable to parse tokens");
    add_block(&mut result, my_end_expr);

	println!("{:#?}", result.clone().into_token_stream().to_string());

	result
}