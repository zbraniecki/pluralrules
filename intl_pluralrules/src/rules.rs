#![allow(unused_variables, unused_parens)]
extern crate matches;
use super::operands::PluralOperands;
use super::PluralCategory;
type PluralRule = fn(PluralOperands) -> PluralCategory;
pub fn get_pr(lang: &str) -> PluralRule {
    match lang {
        "ky" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "gu" => |po| {
            if (po.i == 0) || (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "sg" => |po| PluralCategory::OTHER,
        "br" => |po| {
            if (po.i % 10 == 1 && po.i % 100 != 11 && po.i % 100 != 71 && po.i % 100 != 91) {
                PluralCategory::ONE
            } else if ((po.i % 10 == 9 || matches!(po.i % 10, 3..=4) && po.f == 0)
                && matches!(po.i % 100, 10..=19) && po.f == 0
                && matches!(po.i % 100, 70..=79) && po.f == 0
                && matches!(po.i % 100, 90..=99) && po.f == 0)
            {
                PluralCategory::FEW
            } else if (po.n != 0.0 && po.i % 1000000 == 0) {
                PluralCategory::MANY
            } else if (po.i % 10 == 2 && po.i % 100 != 12 && po.i % 100 != 72 && po.i % 100 != 92) {
                PluralCategory::TWO
            } else {
                PluralCategory::OTHER
            }
        },
        "de" => |po| {
            if (po.i == 1 && po.v == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "uk" => |po| {
            if (po.v == 0 && po.i % 10 == 1 && po.i % 100 != 11) {
                PluralCategory::ONE
            } else if (po.v == 0 && matches!(po.i % 10, 2..=4) && matches!(po.i % 100, 12..=14)) {
                PluralCategory::FEW
            } else if (po.v == 0 && po.i % 10 == 0) || (po.v == 0 && matches!(po.i % 10, 5..=9))
                || (po.v == 0 && matches!(po.i % 100, 11..=14))
            {
                PluralCategory::MANY
            } else {
                PluralCategory::OTHER
            }
        },
        "root" => |po| PluralCategory::OTHER,
        "ur" => |po| {
            if (po.i == 1 && po.v == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "it" => |po| {
            if (po.i == 1 && po.v == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "sah" => |po| PluralCategory::OTHER,
        "nso" => |po| {
            if (matches!(po.i, 0..=1) && po.f == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "bo" => |po| PluralCategory::OTHER,
        "no" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "eu" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "kk" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "mas" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "nn" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "hr" => |po| {
            if (po.v == 0 && matches!(po.i % 10, 2..=4) && matches!(po.i % 100, 12..=14))
                || (matches!(po.f % 10, 2..=4) && matches!(po.f % 100, 12..=14))
            {
                PluralCategory::FEW
            } else if (po.v == 0 && po.i % 10 == 1 && po.i % 100 != 11)
                || (po.f % 10 == 1 && po.f % 100 != 11)
            {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "iu" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else if (po.n == 2.0) {
                PluralCategory::TWO
            } else {
                PluralCategory::OTHER
            }
        },
        "rm" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "uz" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "sr" => |po| {
            if (po.v == 0 && po.i % 10 == 1 && po.i % 100 != 11)
                || (po.f % 10 == 1 && po.f % 100 != 11)
            {
                PluralCategory::ONE
            } else if (po.v == 0 && matches!(po.i % 10, 2..=4) && matches!(po.i % 100, 12..=14))
                || (matches!(po.f % 10, 2..=4) && matches!(po.f % 100, 12..=14))
            {
                PluralCategory::FEW
            } else {
                PluralCategory::OTHER
            }
        },
        "kl" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "ji" => |po| {
            if (po.i == 1 && po.v == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "seh" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "lb" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "ko" => |po| PluralCategory::OTHER,
        "hsb" => |po| {
            if (po.v == 0 && po.i % 100 == 2) || (po.f % 100 == 2) {
                PluralCategory::TWO
            } else if (po.v == 0 && po.i % 100 == 1) || (po.f % 100 == 1) {
                PluralCategory::ONE
            } else if (po.v == 0 && matches!(po.i % 100, 3..=4)) || (matches!(po.f % 100, 3..=4)) {
                PluralCategory::FEW
            } else {
                PluralCategory::OTHER
            }
        },
        "ckb" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "tzm" => |po| {
            if (matches!(po.i, 0..=1) && po.f == 0) || (matches!(po.i, 11..=99) && po.f == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "af" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "id" => |po| PluralCategory::OTHER,
        "zh" => |po| PluralCategory::OTHER,
        "et" => |po| {
            if (po.i == 1 && po.v == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "ksb" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "dsb" => |po| {
            if (po.v == 0 && matches!(po.i % 100, 3..=4)) || (matches!(po.f % 100, 3..=4)) {
                PluralCategory::FEW
            } else if (po.v == 0 && po.i % 100 == 1) || (po.f % 100 == 1) {
                PluralCategory::ONE
            } else if (po.v == 0 && po.i % 100 == 2) || (po.f % 100 == 2) {
                PluralCategory::TWO
            } else {
                PluralCategory::OTHER
            }
        },
        "lg" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "mg" => |po| {
            if (matches!(po.i, 0..=1) && po.f == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "fo" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "ve" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "sv" => |po| {
            if (po.i == 1 && po.v == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "naq" => |po| {
            if (po.n == 2.0) {
                PluralCategory::TWO
            } else if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "ce" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "ee" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "bs" => |po| {
            if (po.v == 0 && matches!(po.i % 10, 2..=4) && matches!(po.i % 100, 12..=14))
                || (matches!(po.f % 10, 2..=4) && matches!(po.f % 100, 12..=14))
            {
                PluralCategory::FEW
            } else if (po.v == 0 && po.i % 10 == 1 && po.i % 100 != 11)
                || (po.f % 10 == 1 && po.f % 100 != 11)
            {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "ja" => |po| PluralCategory::OTHER,
        "is" => |po| {
            if (po.t == 0 && po.i % 10 == 1 && po.i % 100 != 11) || (po.t != 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "ga" => |po| {
            if (matches!(po.i, 3..=6) && po.f == 0) {
                PluralCategory::FEW
            } else if (matches!(po.i, 7..=10) && po.f == 0) {
                PluralCategory::MANY
            } else if (po.n == 2.0) {
                PluralCategory::TWO
            } else if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "be" => |po| {
            if (po.i % 10 == 1 && po.i % 100 != 11) {
                PluralCategory::ONE
            } else if (matches!(po.i % 10, 2..=4) && po.f == 0 && matches!(po.i % 100, 12..=14)
                && po.f == 0)
            {
                PluralCategory::FEW
            } else if (po.i % 10 == 0) || (matches!(po.i % 10, 5..=9) && po.f == 0)
                || (matches!(po.i % 100, 11..=14) && po.f == 0)
            {
                PluralCategory::MANY
            } else {
                PluralCategory::OTHER
            }
        },
        "fil" => |po| {
            if (po.v == 0 && (po.i == 1 || po.i == 2 || po.i == 3))
                || (po.v == 0 && po.i % 10 != 4 && po.i % 10 != 6 && po.i % 10 != 9)
                || (po.v != 0 && po.f % 10 != 4 && po.f % 10 != 6 && po.f % 10 != 9)
            {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "tl" => |po| {
            if (po.v == 0 && (po.i == 1 || po.i == 2 || po.i == 3))
                || (po.v == 0 && po.i % 10 != 4 && po.i % 10 != 6 && po.i % 10 != 9)
                || (po.v != 0 && po.f % 10 != 4 && po.f % 10 != 6 && po.f % 10 != 9)
            {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "to" => |po| PluralCategory::OTHER,
        "yo" => |po| PluralCategory::OTHER,
        "chr" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "bh" => |po| {
            if (matches!(po.i, 0..=1) && po.f == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "kde" => |po| PluralCategory::OTHER,
        "bg" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "hy" => |po| {
            if (po.i == 0 || po.i == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "ksh" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else if (po.n == 0.0) {
                PluralCategory::ZERO
            } else {
                PluralCategory::OTHER
            }
        },
        "sq" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "vun" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "io" => |po| {
            if (po.i == 1 && po.v == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "lag" => |po| {
            if ((po.i == 0 || po.i == 1) && po.n != 0.0) {
                PluralCategory::ONE
            } else if (po.n == 0.0) {
                PluralCategory::ZERO
            } else {
                PluralCategory::OTHER
            }
        },
        "nr" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "lo" => |po| PluralCategory::OTHER,
        "shi" => |po| {
            if (matches!(po.i, 2..=10) && po.f == 0) {
                PluralCategory::FEW
            } else if (po.i == 0) || (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "cs" => |po| {
            if (matches!(po.i, 2..=4) && po.v == 0) {
                PluralCategory::FEW
            } else if (po.i == 1 && po.v == 0) {
                PluralCategory::ONE
            } else if (po.v != 0) {
                PluralCategory::MANY
            } else {
                PluralCategory::OTHER
            }
        },
        "ptPT" => |po| {
            if (po.i == 1 && po.v == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "ts" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "hi" => |po| {
            if (po.i == 0) || (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "ms" => |po| PluralCategory::OTHER,
        "smi" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else if (po.n == 2.0) {
                PluralCategory::TWO
            } else {
                PluralCategory::OTHER
            }
        },
        "mt" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else if (po.n == 0.0) || (matches!(po.i % 100, 2..=10) && po.f == 0) {
                PluralCategory::FEW
            } else if (matches!(po.i % 100, 11..=19) && po.f == 0) {
                PluralCategory::MANY
            } else {
                PluralCategory::OTHER
            }
        },
        "ast" => |po| {
            if (po.i == 1 && po.v == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "gd" => |po| {
            if (po.n == 2.0 || po.n == 12.0) {
                PluralCategory::TWO
            } else if (po.n == 1.0 || po.n == 11.0) {
                PluralCategory::ONE
            } else if (matches!(po.i, 3..=10) && po.f == 0 || matches!(po.i, 13..=19) && po.f == 0)
            {
                PluralCategory::FEW
            } else {
                PluralCategory::OTHER
            }
        },
        "bm" => |po| PluralCategory::OTHER,
        "mr" => |po| {
            if (po.i == 0) || (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "xh" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "km" => |po| PluralCategory::OTHER,
        "en" => |po| {
            if (po.i == 1 && po.v == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "lt" => |po| {
            if (po.f != 0) {
                PluralCategory::MANY
            } else if (matches!(po.i % 10, 2..=9) && po.f == 0 && matches!(po.i % 100, 11..=19)
                && po.f == 0)
            {
                PluralCategory::FEW
            } else if (po.i % 10 == 1 && matches!(po.i % 100, 11..=19) && po.f == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "my" => |po| PluralCategory::OTHER,
        "xog" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "jv" => |po| PluralCategory::OTHER,
        "vo" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "lkt" => |po| PluralCategory::OTHER,
        "prg" => |po| {
            if (po.i % 10 == 1 && po.i % 100 != 11)
                || (po.v == 2 && po.f % 10 == 1 && po.f % 100 != 11)
                || (po.v != 2 && po.f % 10 == 1)
            {
                PluralCategory::ONE
            } else if (po.i % 10 == 0) || (matches!(po.i % 100, 11..=19) && po.f == 0)
                || (po.v == 2 && matches!(po.f % 100, 11..=19))
            {
                PluralCategory::ZERO
            } else {
                PluralCategory::OTHER
            }
        },
        "nyn" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "fi" => |po| {
            if (po.i == 1 && po.v == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "sdh" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "tk" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "th" => |po| PluralCategory::OTHER,
        "bn" => |po| {
            if (po.i == 0) || (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "ps" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "kab" => |po| {
            if (po.i == 0 || po.i == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "st" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "si" => |po| {
            if (po.n == 0.0 || po.n == 1.0) || (po.i == 0 && po.f == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "ka" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "ks" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "he" => |po| {
            if (po.i == 1 && po.v == 0) {
                PluralCategory::ONE
            } else if (po.i == 2 && po.v == 0) {
                PluralCategory::TWO
            } else if (po.v == 0 && matches!(po.i, 0..=10) && po.f == 0 && po.i % 10 == 0) {
                PluralCategory::MANY
            } else {
                PluralCategory::OTHER
            }
        },
        "sd" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "vi" => |po| PluralCategory::OTHER,
        "in" => |po| PluralCategory::OTHER,
        "pap" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "gl" => |po| {
            if (po.i == 1 && po.v == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "fur" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "eo" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "ig" => |po| PluralCategory::OTHER,
        "ln" => |po| {
            if (matches!(po.i, 0..=1) && po.f == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "kw" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else if (po.n == 2.0) {
                PluralCategory::TWO
            } else {
                PluralCategory::OTHER
            }
        },
        "gsw" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "sn" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "ro" => |po| {
            if (po.v != 0) || (po.n == 0.0)
                || (po.n != 1.0 && matches!(po.i % 100, 1..=19) && po.f == 0)
            {
                PluralCategory::FEW
            } else if (po.i == 1 && po.v == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "smn" => |po| {
            if (po.n == 2.0) {
                PluralCategory::TWO
            } else if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "nnh" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "smj" => |po| {
            if (po.n == 2.0) {
                PluralCategory::TWO
            } else if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "dv" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "ak" => |po| {
            if (matches!(po.i, 0..=1) && po.f == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "ny" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "mn" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "sms" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else if (po.n == 2.0) {
                PluralCategory::TWO
            } else {
                PluralCategory::OTHER
            }
        },
        "jgo" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "lv" => |po| {
            if (po.i % 10 == 0) || (matches!(po.i % 100, 11..=19) && po.f == 0)
                || (po.v == 2 && matches!(po.f % 100, 11..=19))
            {
                PluralCategory::ZERO
            } else if (po.i % 10 == 1 && po.i % 100 != 11)
                || (po.v == 2 && po.f % 10 == 1 && po.f % 100 != 11)
                || (po.v != 2 && po.f % 10 == 1)
            {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "am" => |po| {
            if (po.i == 0) || (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "yi" => |po| {
            if (po.i == 1 && po.v == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "da" => |po| {
            if (po.n == 1.0) || (po.t != 0 && (po.i == 0 || po.i == 1)) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "hu" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "kcg" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "bez" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "nl" => |po| {
            if (po.i == 1 && po.v == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "rof" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "nqo" => |po| PluralCategory::OTHER,
        "nd" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "pa" => |po| {
            if (matches!(po.i, 0..=1) && po.f == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "mo" => |po| {
            if (po.i == 1 && po.v == 0) {
                PluralCategory::ONE
            } else if (po.v != 0) || (po.n == 0.0)
                || (po.n != 1.0 && matches!(po.i % 100, 1..=19) && po.f == 0)
            {
                PluralCategory::FEW
            } else {
                PluralCategory::OTHER
            }
        },
        "ne" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "dz" => |po| PluralCategory::OTHER,
        "ses" => |po| PluralCategory::OTHER,
        "asa" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "pt" => |po| {
            if (matches!(po.i, 0..=1)) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "syr" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "fa" => |po| {
            if (po.i == 0) || (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "kn" => |po| {
            if (po.i == 0) || (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "cy" => |po| {
            if (po.n == 2.0) {
                PluralCategory::TWO
            } else if (po.n == 1.0) {
                PluralCategory::ONE
            } else if (po.n == 6.0) {
                PluralCategory::MANY
            } else if (po.n == 0.0) {
                PluralCategory::ZERO
            } else if (po.n == 3.0) {
                PluralCategory::FEW
            } else {
                PluralCategory::OTHER
            }
        },
        "kea" => |po| PluralCategory::OTHER,
        "pl" => |po| {
            if (po.v == 0 && po.i != 1 && matches!(po.i % 10, 0..=1))
                || (po.v == 0 && matches!(po.i % 10, 5..=9))
                || (po.v == 0 && matches!(po.i % 100, 12..=14))
            {
                PluralCategory::MANY
            } else if (po.v == 0 && matches!(po.i % 10, 2..=4) && matches!(po.i % 100, 12..=14)) {
                PluralCategory::FEW
            } else if (po.i == 1 && po.v == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "ss" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "as" => |po| {
            if (po.i == 0) || (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "cgg" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "ars" => |po| {
            if (matches!(po.i % 100, 11..=99) && po.f == 0) {
                PluralCategory::MANY
            } else if (matches!(po.i % 100, 3..=10) && po.f == 0) {
                PluralCategory::FEW
            } else if (po.n == 2.0) {
                PluralCategory::TWO
            } else if (po.n == 0.0) {
                PluralCategory::ZERO
            } else if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "gv" => |po| {
            if (po.v == 0 && po.i % 10 == 2) {
                PluralCategory::TWO
            } else if (po.v == 0 && po.i % 10 == 1) {
                PluralCategory::ONE
            } else if (po.v == 0
                && (po.i % 100 == 0 || po.i % 100 == 20 || po.i % 100 == 40 || po.i % 100 == 60
                    || po.i % 100 == 80))
            {
                PluralCategory::FEW
            } else if (po.v != 0) {
                PluralCategory::MANY
            } else {
                PluralCategory::OTHER
            }
        },
        "ug" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "ru" => |po| {
            if (po.v == 0 && matches!(po.i % 10, 2..=4) && matches!(po.i % 100, 12..=14)) {
                PluralCategory::FEW
            } else if (po.v == 0 && po.i % 10 == 0) || (po.v == 0 && matches!(po.i % 10, 5..=9))
                || (po.v == 0 && matches!(po.i % 100, 11..=14))
            {
                PluralCategory::MANY
            } else if (po.v == 0 && po.i % 10 == 1 && po.i % 100 != 11) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "os" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "sk" => |po| {
            if (matches!(po.i, 2..=4) && po.v == 0) {
                PluralCategory::FEW
            } else if (po.i == 1 && po.v == 0) {
                PluralCategory::ONE
            } else if (po.v != 0) {
                PluralCategory::MANY
            } else {
                PluralCategory::OTHER
            }
        },
        "rwk" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "ar" => |po| {
            if (po.n == 0.0) {
                PluralCategory::ZERO
            } else if (po.n == 1.0) {
                PluralCategory::ONE
            } else if (matches!(po.i % 100, 11..=99) && po.f == 0) {
                PluralCategory::MANY
            } else if (po.n == 2.0) {
                PluralCategory::TWO
            } else if (matches!(po.i % 100, 3..=10) && po.f == 0) {
                PluralCategory::FEW
            } else {
                PluralCategory::OTHER
            }
        },
        "tn" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "ml" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "bem" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "jbo" => |po| PluralCategory::OTHER,
        "nb" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "haw" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "tig" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "guw" => |po| {
            if (matches!(po.i, 0..=1) && po.f == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "az" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "wa" => |po| {
            if (matches!(po.i, 0..=1) && po.f == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "se" => |po| {
            if (po.n == 2.0) {
                PluralCategory::TWO
            } else if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "fr" => |po| {
            if (po.i == 0 || po.i == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "or" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "mgo" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "scn" => |po| {
            if (po.i == 1 && po.v == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "wo" => |po| PluralCategory::OTHER,
        "teo" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "mk" => |po| {
            if (po.v == 0 && po.i % 10 == 1 && po.i % 100 != 11)
                || (po.f % 10 == 1 && po.f % 100 != 11)
            {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "ca" => |po| {
            if (po.i == 1 && po.v == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "ku" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "el" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "saq" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "nah" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "sma" => |po| {
            if (po.n == 2.0) {
                PluralCategory::TWO
            } else if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "kaj" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "om" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "ssy" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "sw" => |po| {
            if (po.i == 1 && po.v == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "so" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "tr" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "kkj" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "es" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "jmc" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "jw" => |po| PluralCategory::OTHER,
        "sh" => |po| {
            if (po.v == 0 && po.i % 10 == 1 && po.i % 100 != 11)
                || (po.f % 10 == 1 && po.f % 100 != 11)
            {
                PluralCategory::ONE
            } else if (po.v == 0 && matches!(po.i % 10, 2..=4) && matches!(po.i % 100, 12..=14))
                || (matches!(po.f % 10, 2..=4) && matches!(po.f % 100, 12..=14))
            {
                PluralCategory::FEW
            } else {
                PluralCategory::OTHER
            }
        },
        "te" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "fy" => |po| {
            if (po.i == 1 && po.v == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "sl" => |po| {
            if (po.v == 0 && po.i % 100 == 1) {
                PluralCategory::ONE
            } else if (po.v == 0 && po.i % 100 == 2) {
                PluralCategory::TWO
            } else if (po.v == 0 && matches!(po.i % 100, 3..=4)) || (po.v != 0) {
                PluralCategory::FEW
            } else {
                PluralCategory::OTHER
            }
        },
        "ff" => |po| {
            if (po.i == 0 || po.i == 1) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "ta" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "zu" => |po| {
            if (po.i == 0) || (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "wae" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "brx" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "iw" => |po| {
            if (po.i == 1 && po.v == 0) {
                PluralCategory::ONE
            } else if (po.i == 2 && po.v == 0) {
                PluralCategory::TWO
            } else if (po.v == 0 && matches!(po.i, 0..=10) && po.f == 0 && po.i % 10 == 0) {
                PluralCategory::MANY
            } else {
                PluralCategory::OTHER
            }
        },
        "ti" => |po| {
            if (matches!(po.i, 0..=1) && po.f == 0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        "yue" => |po| PluralCategory::OTHER,
        "ii" => |po| PluralCategory::OTHER,
        "ha" => |po| {
            if (po.n == 1.0) {
                PluralCategory::ONE
            } else {
                PluralCategory::OTHER
            }
        },
        _ => panic!("Unknown locale!"),
    }
}
