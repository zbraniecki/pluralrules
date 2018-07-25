extern crate proc_macro2;
extern crate quote;
extern crate syn;

extern crate cldr_pluralrules_parser;

use self::proc_macro2::Span;
use self::syn::BinOp;

use self::proc_macro2::TokenStream;
use cldr_pluralrules_parser::ast::*;
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
fn convert_rangl(
    rangl: RangeList,
) -> (
    Vec<syn::LitInt>,
    Vec<(syn::LitInt, syn::token::DotDotEq, syn::LitInt)>,
) {
    let mut litints = Vec::<syn::LitInt>::new();
    let mut litrange = Vec::<(syn::LitInt, syn::token::DotDotEq, syn::LitInt)>::new();

    for x in rangl.0 {
        match &x {
            RangeListItem::Value(x) => litints.push(convert_literal(x.0)),
            RangeListItem::Range(x) => litrange.push(convert_range(x.lower_val.0, x.upper_val.0)),
        }
    }

    (litints, litrange)
}

// match the needed operator symbol
fn get_operator_symbol(op: &Operator) -> BinOp {
    match op {
        Operator::In => BinOp::Eq(syn::token::EqEq::new(Span::call_site())),
        Operator::NotIn => BinOp::Ne(syn::token::Ne::new(Span::call_site())),
        Operator::Within => BinOp::Le(syn::token::Le::new(Span::call_site())),
        Operator::NotWithin => BinOp::Gt(syn::token::Gt::new(Span::call_site())),
        Operator::Is => BinOp::Eq(syn::token::EqEq::new(Span::call_site())),
        Operator::IsNot => BinOp::Ne(syn::token::Ne::new(Span::call_site())),
        Operator::EQ => BinOp::Eq(syn::token::EqEq::new(Span::call_site())),
        Operator::NotEQ => BinOp::Ne(syn::token::Ne::new(Span::call_site())),
    }
}

fn create_relation(rel: Relation) -> TokenStream {
    let left = rel.expression;
    let operator = rel.operator;
    let right = rel.range_list;

    let mut relations = Vec::<TokenStream>::new();

    let l = convert_ident(&left.operand.0.to_string());
    let o = get_operator_symbol(&operator);
    let r1 = convert_rangl(right);

    let mut m = convert_literal(0);
    let mod_check = if left.modulus != None {
        m = convert_literal((left.modulus.unwrap().0).0);
        true
    } else {
        false
    };



    if operator == Operator::Within || operator == Operator::NotWithin {
        let rfront_lit = &(r1.1)[0].0;
        let rback_lit = &(r1.1)[0].2;

        let (rfront, rback, whole_symbol) =
            if left.operand.0 == 'n' {
                if !mod_check {
                    (quote!(#rfront_lit.0),
                    quote!(#rback_lit.0),
                    quote!(po.#l))
                } else {
                    (quote!(#rfront_lit),
                    quote!(#rback_lit),
                    quote!(po.i % #m))
                }
            } else {
                (quote!(#rfront_lit),
                quote!(#rback_lit),
                if !mod_check {
                    quote!(po.#l)
                } else {
                    quote!(po.i % #m)
                })
            };

        let rel_tokens = quote! { #rfront #o #whole_symbol && #whole_symbol #o #rback};

        relations.push(rel_tokens);
    } else {
        for r in r1.0 {

            let (symbol, rval) = 
                if left.operand.0 == 'n' {
                    if !mod_check {
                        (quote!(po.#l), 
                        quote!(#r.0))
                    } else {
                        (quote!(po.i % #m), 
                        quote!(#r))
                    }
                } else {
                    (
                    if !mod_check {
                        quote!(po.#l)
                    } else {
                        quote!(po.#l % #m)
                    },
                    quote!(#r))
                };


            // let rel_tokens = if left.operand.0 == 'n' {
            //     if mod_check == false {
            //         quote! {po.#l #o #r.0}
            //     } else {
            //         quote! {po.i % #m #o #r}
            //     }
            // } else {
            //     if mod_check == false {
            //         quote! {po.#l #o #r}
            //     } else {
            //         quote! {po.#l % #m #o #r}
            //     }
            // };

            // symbol = #l || i 
            // mod_symbol = #symbol || #symbol % m
            // rval = #r || r.0

            // let whole_symbol = quote!{ po.#symbol };

            let rel_tokens = quote!{ #symbol #o #rval };

            relations.push(rel_tokens);
        }
        for r in r1.1 {
            let rfront = r.0;
            let rdot = r.1;
            let rback = r.2;

            let rel_tokens = if left.operand.0 == 'n' {
                if mod_check == false {
                    quote! {matches!(po.i, #rfront #rdot #rback) && po.f == 0 }
                } else {
                    quote! {matches!(po.i % #m, #rfront #rdot #rback) && po.f == 0}
                }
            } else {
                if mod_check == false {
                    quote! {matches!(po.#l, #rfront #rdot #rback)}
                } else {
                    quote! {matches!(po.#l % #m, #rfront #rdot #rback)}
                }
            };

            relations.push(rel_tokens);
        }
    }

    let relationexpr = match operator {
        Operator::In => if relations.len() > 1 {
            quote!{ ( #(#relations)||* ) }
        } else {
            quote!{ #(#relations)||* }
        },
        Operator::NotIn => quote!{ #(#relations)&&* },
        Operator::Within => quote!{ #(#relations)||* },
        Operator::NotWithin => quote!{ #(#relations)||* },
        Operator::Is => if relations.len() > 1 {
            quote!{ ( #(#relations)||* ) }
        } else {
            quote!{ #(#relations)||* }
        },
        Operator::IsNot => quote!{ #(#relations)&&* },
        Operator::EQ => if relations.len() > 1 {
            quote!{ ( #(#relations)||* ) }
        } else {
            quote!{ #(#relations)||* }
        },
        Operator::NotEQ => quote!{ #(#relations)&&* },
    };
    relationexpr
}

// Unfold AndConditions and tokenize together with &&
fn create_and_condition(acond: AndCondition) -> TokenStream {
    let mut andcondvec = Vec::<TokenStream>::new();

    // unpack the AndCondition and get all relations from within it
    for a in acond.0 {
        andcondvec.push(create_relation(a));
    }

    // Unfold AndConditions and tokenize together with &&
    quote!{ ( #(#andcondvec)&&* ) }
}

// unfold OrConditions and tokenize together with ||
fn create_condition(cond: Condition) -> TokenStream {
    let mut condvec = Vec::<TokenStream>::new();

    // unpack the OrCondition and get all AndConditions from within it
    for c in cond.0 {
        condvec.push(create_and_condition(c));
    }

    // unfold OrConditions and tokenize together with ||
    quote!{ #(#condvec)||* }
}

// Function takes a full condition as input
// Returns a TokenStream of the expression of the plural rule in Rust
pub fn gen_pr(cond: Condition) -> TokenStream {
    // create_condition(cond).into_token_stream()
    create_condition(cond)
}
