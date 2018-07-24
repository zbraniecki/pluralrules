extern crate proc_macro2;
extern crate cldr_pluralrules_parser;

use self::proc_macro2::Span;

use self::proc_macro2::{Ident, Literal, TokenStream};
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
fn convert_rangl(
    rangl: RangeList,
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

        let rel_tokens = if left.operand.0 == 'n' {
            if mod_check == false {
                quote! {#rfront.0 #o po.#l && po.#l #o #rback.0}
            } else {
                quote! {#rfront #o po.i % #m && po.i % #m #o #rback}
            }
        } else {
            if mod_check == false {
                quote! {#rfront #o po.#l && po.#l #o #rback}
            } else {
                quote! {#rfront #o po.#l % #m && po.#l % #m #o #rback}
            }
        };

        relations.push(rel_tokens);
    } else {
        for r in r1.0 {
            let rel_tokens = if left.operand.0 == 'n' {
                if mod_check == false {
                    quote! {po.#l #o #r.0}
                } else {
                    quote! {po.i % #m #o #r}
                }
            } else {
                if mod_check == false {
                    quote! {po.#l #o #r}
                } else {
                    quote! {po.#l % #m #o #r}
                }
            };

            relations.push(rel_tokens);
        }
        for r in r1.1 {
            let rfront = r.0;
            let rback = r.1;

            let rel_tokens = if left.operand.0 == 'n' {
                if mod_check == false {
                    quote! {matches!(po.i, #rfront ..= #rback) && po.f == 0 }
                } else {
                    quote! {matches!(po.i % #m, #rfront ..= #rback) && po.f == 0}
                }
            } else {
                if mod_check == false {
                    quote! {matches!(po.#l, #rfront ..= #rback)}
                } else {
                    quote! {matches!(po.#l % #m, #rfront ..= #rback)}
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
