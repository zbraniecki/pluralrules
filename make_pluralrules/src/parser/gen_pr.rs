use proc_macro2::Span;

use proc_macro2::{Ident, Literal, TokenStream};
use cldr_pluralrules_parser::ast::*;

fn convert_literal(num: usize) -> Literal {
    Literal::u64_unsuffixed(num as u64)
}

fn convert_ident(id: &str) -> Ident {
    Ident::new(id, Span::call_site())
}

fn convert_range(low: usize, up: usize) -> (Literal, Literal) {
    let u = convert_literal(up);
    let d = convert_literal(low);

    (d, u)
}

// convert a range list into a tuple of lists: one for lists of values and one for lists of ranges

fn convert_rangl(rangl: RangeList) 
) -> (
    Vec<Literal>,
    Vec<(Literal, Literal)>,
) {
    
    let mut litints = Vec::new();
    let mut litrange = Vec::new();

    for x in rangl.0 {
        match &x {
            RangeListItem::Value(x) => litints.push(convert_literal(x.0)),
            RangeListItem::Range(x) => litrange.push(convert_range(x.lower_val.0, x.upper_val.0)),
        }
    }

    (litints, litrange)
}

// match the needed operator symbol
fn get_operator_symbol(op: &Operator) -> TokenStream {
    match op {
        Operator::In | Operator::Is | Operator::EQ => quote!(==),
        Operator::NotIn | Operator::IsNot | Operator::NotEQ => quote!(!=),
        Operator::Within => quote!(<=),
        Operator::NotWithin => quote!(>),
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
        let rfront = &(r1.1)[0].0;
        let rback = &(r1.1)[0].1;

        let (rfront, rback, whole_symbol) = if left.operand.0 == 'n' {
            if !mod_check {
                (quote!(#rfront_lit.0), quote!(#rback_lit.0), quote!(po.#l))
            } else {
                (quote!(#rfront_lit), quote!(#rback_lit), quote!(po.i % #m))
            }
        } else {
            (
                quote!(#rfront_lit),
                quote!(#rback_lit),
                if !mod_check {
                    quote!(po.#l)
                } else {
                    quote!(po.i % #m)
                },
            )
        };

        let rel_tokens = quote! { #rfront #o #whole_symbol && #whole_symbol #o #rback};
        relations.push(rel_tokens);
    } else {
        for r in r1.0 {
            let (symbol, rval) = if left.operand.0 == 'n' {
                if !mod_check {
                    (quote!(po.#l), quote!(#r.0))
                } else {
                    (quote!(po.i % #m), quote!(#r))
                }
            } else {
                (
                    if !mod_check {
                        quote!(po.#l)
                    } else {
                        quote!(po.#l % #m)
                    },
                    quote!(#r),
                )
            };

            let rel_tokens = quote!{ #symbol #o #rval };
            relations.push(rel_tokens);
        }
        for r in r1.1 {
            let rfront = r.0;
            let rback = r.1;


            let (symbol, perim) = if left.operand.0 == 'n' {
                if !mod_check {
                    (quote!(po.i), quote!{ && po.f == 0})
                } else {
                    (quote!(po.i), quote!{})
                }
            } else {
                (
                    if !mod_check {
                        quote!(po.#l)
                    } else {
                        quote!(po.#l % #m)
                    },
                    quote!{},
                )
            };

            let filling = quote! { #symbol, #rfront ..= #rback };
            let rel_tokens = quote! { matches!( #filling ) #perim };
            relations.push(rel_tokens);
        }
    }

    let relationexpr = match operator {
        Operator::In | Operator::Is | Operator::EQ => if relations.len() > 1 {
            quote!{ ( #(#relations)||* ) }
        } else {
            quote!{ #(#relations)||* }
        },
        Operator::NotIn | Operator::NotEQ | Operator::IsNot => quote!{ #(#relations)&&* },
        Operator::Within | Operator::NotWithin => quote!{ #(#relations)||* },
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
