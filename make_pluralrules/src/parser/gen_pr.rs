extern crate syn;
extern crate proc_macro2;
extern crate quote;

extern crate cldr_pluralrules_parser;

use self::syn::BinOp;
use self::proc_macro2::{Span};

use cldr_pluralrules_parser::ast::*;
use self::proc_macro2::TokenStream;
// use quote::ToTokens;

// use syn to convert a usize to a literal int in the generated rust code
fn convert_literal(num: usize) -> syn::LitInt {
    syn::LitInt::new(num as u64, syn::IntSuffix::None, Span::call_site())
}

// use syn to convert a string to a literal variable in the generated rust code
fn convert_ident(id: &str) -> syn::Ident {
    syn::Ident::new(id, Span::call_site())
}

// use syn to convert two usize into a tuple with literal ints and a dotdot for range
fn convert_range(low: usize, up: usize) -> (syn::LitInt, syn::token::DotDotEq, syn::LitInt) {
    let u = convert_literal(up);
    let d = convert_literal(low);
    let p = syn::token::DotDotEq::new(Span::call_site());

    (d, p, u)
}

// convert a range list into a tuple of lists: one for lists of values and one for lists of ranges
fn convert_rangl (rangl: RangeList) -> (Vec<syn::LitInt>, Vec<(syn::LitInt, syn::token::DotDotEq, syn::LitInt)>) {
    let mut litints = Vec::<syn::LitInt>::new();
    let mut litrange = Vec::<(syn::LitInt, syn::token::DotDotEq, syn::LitInt)>::new();

    for x in rangl.0 {
        match &x {
            RangeListItem::Value(x) => litints.push(convert_literal(x.0)),
            RangeListItem::Range(x) => litrange.push(convert_range(x.lower_val.0, x.upper_val.0))
        }
    };

    (litints, litrange)
}

// match the needed operator symbol
fn get_operator_symbol(op: Operator) -> BinOp {
	match op {
		Operator::In => BinOp::Eq(syn::token::EqEq::new(Span::call_site())),
	    Operator::NotIn => BinOp::Ne(syn::token::Ne::new(Span::call_site())),
	    Operator::Within => BinOp::Le(syn::token::Le::new(Span::call_site())),
	    Operator::NotWithin => BinOp::Gt(syn::token::Gt::new(Span::call_site())),
	    Operator::Is => BinOp::Eq(syn::token::EqEq::new(Span::call_site())),
	    Operator::IsNot => BinOp::Ne(syn::token::Ne::new(Span::call_site())),
	    Operator::EQ => BinOp::Eq(syn::token::EqEq::new(Span::call_site())),
	    Operator::NotEQ => BinOp::Ne(syn::token::Ne::new(Span::call_site()))
	}
}

fn create_relation(rel : Relation) -> TokenStream {
    let left = rel.expression;
    let operator = rel.operator;
    let right = rel.range_list;

    let mut relations = Vec::<TokenStream>::new();

    let l = convert_ident(&left.operand.0.to_string());
    let o = get_operator_symbol(operator.clone());
    let r1 = convert_rangl(right);

    if operator == Operator::Within || operator == Operator::NotWithin{
        let rfront = (r1.1)[0].0.clone();
        let rback = (r1.1)[0].2.clone();

        let rel_tokens =
            if left.operand.0.to_string() == "n" {
                if left.modulus == None {
                    quote! {#rfront.0 #o po.#l && po.#l #o #rback.0}
                } else {
                    let m = convert_literal((left.modulus.clone().unwrap().0).0);
                    quote! {#rfront #o po.i % #m && po.i % #m #o #rback}
                }
            } else {
                if left.modulus == None {
                    quote! {#rfront #o po.#l && po.#l #o #rback}
                } else {
                    let m = convert_literal((left.modulus.clone().unwrap().0).0);
                    quote! {#rfront #o po.#l % #m && po.#l % #m #o #rback}
                }
            };

        relations.push(rel_tokens);
    } else {
        for r in r1.0 {

            let rel_tokens =
                if &left.operand.0.to_string() == "n" {
                    if &left.modulus == &None {
                        quote! {po.#l #o #r.0}
                    } else {
                        let m = convert_literal((left.modulus.clone().unwrap().0).0);
                        quote! {po.i % #m #o #r}
                    }
                } else {
                    if &left.modulus == &None {
                        quote! {po.#l #o #r}
                    } else {
                        let m = convert_literal((left.modulus.clone().unwrap().0).0);
                        quote! {po.#l % #m #o #r}
                    }
                };

            relations.push(rel_tokens);
        }
        for r in r1.1 {

            let rfront = r.0;
            let rdot = r.1;
            let rback = r.2;

            let rel_tokens =
                if &left.operand.0.to_string() == "n" {
                    if &left.modulus == &None {
                        quote! {matches!(po.i, #rfront #rdot #rback) && po.f == 0 }
                    } else {
                        let m = convert_literal((left.modulus.clone().unwrap().0).0);
                        quote! {matches!(po.i % #m, #rfront #rdot #rback) && po.f == 0}
                    }
                } else {
                    if &left.modulus == &None {
                        quote! {matches!(po.#l, #rfront #rdot #rback)}
                    } else {
                        let m = convert_literal((left.modulus.clone().unwrap().0).0);
                        quote! {matches!(po.#l % #m, #rfront #rdot #rback)}
                    }
                };

            relations.push(rel_tokens);
        }
    }

    let relationexpr = 
        match operator {
            Operator::In => if relations.len() > 1 { quote!{ ( #(#relations)||* ) } } else {quote!{ #(#relations)||* } },
            Operator::NotIn => quote!{ #(#relations)&&* },
            Operator::Within => quote!{ #(#relations)||* },
            Operator::NotWithin => quote!{ #(#relations)||* },
            Operator::Is => if relations.len() > 1 { quote!{ ( #(#relations)||* ) } } else {quote!{ #(#relations)||* } },
            Operator::IsNot => quote!{ #(#relations)&&* },
            Operator::EQ => if relations.len() > 1 { quote!{ ( #(#relations)||* ) } } else {quote!{ #(#relations)||* } },
            Operator::NotEQ => quote!{ #(#relations)&&* }
        };
    relationexpr

}

// Unfold AndConditions and tokenize together with &&
fn create_and_condition(acond: AndCondition) -> TokenStream {
    let mut andcondvec = Vec::<TokenStream>::new();
 
    // unpack the AndCondition and get all relations from within it
    for a in acond.0 {
        andcondvec.push(create_relation(a.clone()));
    };
    
    // Unfold AndConditions and tokenize together with &&
    quote!{ ( #(#andcondvec)&&* ) }
}

// unfold OrConditions and tokenize together with ||
fn create_condition(cond: Condition) -> TokenStream {
    let mut condvec = Vec::<TokenStream>::new();
 
    // unpack the OrCondition and get all AndConditions from within it
    for c in cond.0 {
        condvec.push(create_and_condition(c.clone()));
    };

    // unfold OrConditions and tokenize together with ||
    quote!{ #(#condvec)||* }
}

// Function takes a full condition as input
// Returns a TokenStream of the expression of the plural rule in Rust
pub fn gen_pr(cond: Condition) -> TokenStream {
	// create_condition(cond).into_token_stream()
    create_condition(cond)
} 

// Place holder for the other rule. Fills the required right side of the match expression but it is not used in gen_rs
pub fn other() -> TokenStream {
    let temp = convert_literal(1);
    let condexpr = quote!{ #temp };

    condexpr
} 