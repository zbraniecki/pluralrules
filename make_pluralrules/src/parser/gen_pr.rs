//! gen_pr contains functions that convert the CLDR Plural Rule AST into a token stream.
//!
//! The resulting token stream is used to generate a Rust code representation of CLDR plural rules. That Rust code is then used to determine the corresponding plural rule for a number.

/// This code utilizes the cldr_pluralrules_parser AST
use cldr_pluralrules_parser::ast::*;
/// proc_macro2 provides the TokenStream type
use proc_macro2::{Ident, Literal, Span, TokenStream};

/// Convert a usize to a Literal
fn convert_literal(num: usize) -> Literal {
    Literal::u64_unsuffixed(num as u64)
}

/// Convert a string slice into a Ident
fn convert_ident(id: &str) -> Ident {
    Ident::new(id, Span::call_site())
}

/// Convert a usize pair into a tuple of literals
fn convert_range(low: usize, up: usize) -> (Literal, Literal) {
    let u = convert_literal(up);
    let d = convert_literal(low);

    (d, u)
}

/// Convert a range list into a tuple of lists: one for lists of values and one for lists of ranges
fn convert_rangl(rangl: RangeList) -> (Vec<Literal>, Vec<(Literal, Literal)>) {
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

/// Match the needed operator symbol from Operator type in AST
fn get_operator_symbol(op: &Operator) -> TokenStream {
    match op {
        Operator::In | Operator::Is | Operator::EQ => quote!(==),
        Operator::NotIn | Operator::IsNot | Operator::NotEQ => quote!(!=),
        Operator::Within => quote!(<=),
        Operator::NotWithin => quote!(>),
    }
}

/// Create a token stream representation from the AST Relation
fn create_relation(rel: Relation) -> TokenStream {
    let left = rel.expression;
    let operator = rel.operator;
    let right = rel.range_list;

    // All relations that need to be represented in the rust code are folded here. They are unfolded into one token steam later.
    let mut relations = Vec::<TokenStream>::new();

    let l = convert_ident(&left.operand.0.to_string());
    let o = get_operator_symbol(&operator);
    let r1 = convert_rangl(right);

    // If there is a modulus, convert to literal. If not, placehold literal
    let (mod_check, m) = if left.modulus != None {
        (true, convert_literal((left.modulus.unwrap().0).0))
    } else {
        (false, convert_literal(0))
    };

    // Here, we have to manage 4 varying types of expression that we may write.

    // First, we check for a within operator because it changes the number of expressions we need.
    // Then, within that check, we look for the operand 'n'. Because 'n' can be a float, we may need to
    // change the compared value to a float type, or use the operand 'i' in 'n's place and also check for f == 0
    // We also check for a modulus on the operand.
    // Lastly, we want to make sure we allow values and ranges to be indeterminately folded together.

    // If within type operator, use format x < po && po < y
    if operator == Operator::Within || operator == Operator::NotWithin {
        let rfront = &(r1.1)[0].0;
        let rback = &(r1.1)[0].1;

        // Variants handled here
        let (rfront, rback, whole_symbol) = if left.operand.0 == 'n' {
            if !mod_check {
                (quote!(#rfront.0), quote!(#rback.0), quote!(po.#l))
            } else {
                (quote!(#rfront), quote!(#rback), quote!(po.i % #m))
            }
        } else {
            (
                quote!(#rfront),
                quote!(#rback),
                if !mod_check {
                    quote!(po.#l)
                } else {
                    quote!(po.i % #m)
                },
            )
        };

        let rel_tokens = quote! { #rfront #o #whole_symbol && #whole_symbol #o #rback};
        relations.push(rel_tokens);
    // if not within type, use format po < x
    } else {
        // Recurisvely fold all values
        for r in r1.0 {
            // Variants handled here
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
        // Recursively fold all ranges
        for r in r1.1 {
            let rfront = r.0;
            let rback = r.1;

            // Variants handled here
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

            let rel_tokens = quote! { matches!( #symbol, #rfront ..= #rback) #perim };
            relations.push(rel_tokens);
        }
    }

    // Unfold all expressions from folded expressions
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

/// This Function takes a full condition as input and returns a TokenStream of the expression of the plural rule in Rust.
pub fn gen_pr(cond: Condition) -> TokenStream {
    // create_condition(cond).into_token_stream()
    create_condition(cond)
}
