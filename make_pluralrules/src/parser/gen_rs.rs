extern crate syn;
extern crate proc_macro2;
extern crate quote;

use super::plural_category::PluralCategory;

use self::proc_macro2::{Span};

use self::syn::token;

// use quote::ToTokens;
use self::proc_macro2::TokenStream;
// use proc_macro2::{Ident, Span};

fn convert_ident(ch: &str) -> syn::Ident {
    syn::Ident::new(ch, Span::call_site())
}

fn create_boilerplate_from_quote() -> syn::File {
    let boilerplate_tokens = quote! {
        fn get_pr(lang: &str) -> PluralRule { match lang {} }
    };
    syn::parse2(boilerplate_tokens).expect("Unable to parse boilerplate")
}

fn convert_litstr(s: &str) -> syn::LitStr {
    syn::LitStr::new(s, Span::call_site())
}

fn create_match_state(lang: &str, filling : TokenStream) -> TokenStream {
    let fnname = "pr_".to_owned() + lang;
    let match_name = convert_litstr(&lang);
    // let match_tokens = quote! {
    //     #match_name => |po| {},
    // }
    quote! { #match_name => |po| { #filling }}
    // syn::parse2(boilerplate_tokens).expect("Unable to parse boilerplate")
}

fn create_fun(filling : TokenStream) -> TokenStream {
    let head = quote! { #![allow(unused_variables, unused_parens)] extern crate matches; use super::operands::PluralOperands; use super::PluralCategory; type PluralRule = fn(PluralOperands) -> PluralCategory; };

    quote! { #head pub fn get_pr(lang: &str) -> PluralRule {match lang { #filling }}}
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

// fn add_block(f: &mut syn::File, b: syn::Expr) {
//     let func = &mut f.items[0];
//     match func {
//         syn::Item::Fn(ref mut f) => {
//             let block = &mut f.block;
//             let stmts = &mut block.stmts;
//             stmts.push(syn::Stmt::Expr(b));
//         },
//         _ => panic!("Unknown boilerplate")
//     };
// }

pub fn gen_mid(lang: &str, ex : Vec<(PluralCategory, syn::Expr)> ) -> TokenStream {

    let size = ex.len();

    let mut result = create_boilerplate_from_quote();

    // let mut center_block = create_match_state(lang);

    let mut iter = ex.iter();
    let pair = iter.next().unwrap();
    let mut tokens = create_return(pair.0, &pair.1);
    
    if size > 1 {
        for pair in iter {
            let condition = create_return(pair.0, &pair.1);
            tokens = quote! { #tokens else #condition };
        }
    }

    // let  central_expression = syn::parse2(tokens).expect("Unable to parse tokens");
    
    let mid = create_match_state(&lang,tokens);

    // println!("{:?}", mid.clone().to_string());

    // let mid_give = syn::parse2(mid.clone()).expect("MID GIVE PROBLEMS WITH MY CODE");

    // add_block(&mut result, mid_give);

	// result

    let what_I_want = mid.clone();

    what_I_want
}

pub fn gen_fn(mut streams: Vec<TokenStream> ) -> TokenStream {

    streams.push(quote! { _ => panic!("Unknown locale!") });

    // for x in streams.clone() {
    //     println!("{:?}", x.to_string());
    // }

    let record_label = streams[0].clone();

    let mbfgw = quote!{ #(#streams),* };

    // println!("{}", mbfgw.to_string());

    let junk = create_fun(mbfgw);

    println!("{}", junk.to_string());

    // let size = ex.len();

    // let mut result = create_boilerplate_from_quote();

    // // let mut center_block = create_match_state(lang);

    // let mut iter = ex.iter();
    // let pair = iter.next().unwrap();
    // let mut tokens = create_return(pair.0, &pair.1);
    
    // if size > 1 {
    //     for pair in iter {
    //         let condition = create_return(pair.0, &pair.1);
    //         tokens = quote! { #tokens else #condition };
    //     }
    // }

    // // let  central_expression = syn::parse2(tokens).expect("Unable to parse tokens");
    
    // let mid = create_match_state(&lang,tokens);

    // println!("{:?}", mid.clone().to_string());

    // let mid_give = syn::parse2(mid.clone()).expect("MID GIVE PROBLEMS WITH MY CODE");

    // add_block(&mut result, mid_give);

    // // result

    // let what_I_want = mid.clone();

    // what_I_want

    record_label
}