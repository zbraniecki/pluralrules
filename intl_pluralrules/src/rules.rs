#![allow(unused_variables, unused_parens)]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::float_cmp))]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::unreadable_literal))]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::nonminimal_bool))]
extern crate matches;
use super::operands::PluralOperands;
use super::{PluralCategory, PluralRuleType};
use phf;
pub type PluralRule = fn(&PluralOperands) -> PluralCategory;
pub static CLDR_VERSION: usize = 34;
#[cfg_attr(tarpaulin, skip)]
pub fn get_locales(pr_type: PluralRuleType) -> &'static [&'static str] {
    match pr_type {
        PluralRuleType::CARDINAL => &[
            "af", "ak", "am", "ar", "ars", "as", "asa", "ast", "az", "be", "bem", "bez", "bg",
            "bh", "bm", "bn", "bo", "br", "brx", "bs", "ca", "ce", "cgg", "chr", "ckb", "cs", "cy",
            "da", "de", "dsb", "dv", "dz", "ee", "el", "en", "eo", "es", "et", "eu", "fa", "ff",
            "fi", "fil", "fo", "fr", "fur", "fy", "ga", "gd", "gl", "gsw", "gu", "guw", "gv", "ha",
            "haw", "he", "hi", "hr", "hsb", "hu", "hy", "ia", "id", "ig", "ii", "in", "io", "is",
            "it", "iu", "iw", "ja", "jbo", "jgo", "ji", "jmc", "jv", "jw", "ka", "kab", "kaj",
            "kcg", "kde", "kea", "kk", "kkj", "kl", "km", "kn", "ko", "ks", "ksb", "ksh", "ku",
            "kw", "ky", "lag", "lb", "lg", "lkt", "ln", "lo", "lt", "lv", "mas", "mg", "mgo", "mk",
            "ml", "mn", "mo", "mr", "ms", "mt", "my", "nah", "naq", "nb", "nd", "ne", "nl", "nn",
            "nnh", "no", "nqo", "nr", "nso", "ny", "nyn", "om", "or", "os", "pa", "pap", "pl",
            "prg", "ps", "pt", "pt-PT", "rm", "ro", "rof", "root", "ru", "rwk", "sah", "saq", "sc",
            "scn", "sd", "sdh", "se", "seh", "ses", "sg", "sh", "shi", "si", "sk", "sl", "sma",
            "smi", "smj", "smn", "sms", "sn", "so", "sq", "sr", "ss", "ssy", "st", "sv", "sw",
            "syr", "ta", "te", "teo", "th", "ti", "tig", "tk", "tl", "tn", "to", "tr", "ts", "tzm",
            "ug", "uk", "ur", "uz", "ve", "vi", "vo", "vun", "wa", "wae", "wo", "xh", "xog", "yi",
            "yo", "yue", "zh", "zu",
        ],
        PluralRuleType::ORDINAL => &[
            "af", "am", "ar", "as", "az", "be", "bg", "bn", "bs", "ca", "ce", "cs", "cy", "da",
            "de", "dsb", "el", "en", "es", "et", "eu", "fa", "fi", "fil", "fr", "fy", "ga", "gd",
            "gl", "gsw", "gu", "he", "hi", "hr", "hsb", "hu", "hy", "ia", "id", "in", "is", "it",
            "iw", "ja", "ka", "kk", "km", "kn", "ko", "ky", "lo", "lt", "lv", "mk", "ml", "mn",
            "mo", "mr", "ms", "my", "nb", "ne", "nl", "or", "pa", "pl", "prg", "ps", "pt", "ro",
            "root", "ru", "sc", "scn", "sd", "sh", "si", "sk", "sl", "sq", "sr", "sv", "sw", "ta",
            "te", "th", "tk", "tl", "tr", "uk", "ur", "uz", "vi", "yue", "zh", "zu",
        ],
    }
}
#[cfg_attr(tarpaulin, skip)]
pub fn get_pr(lang_code: &str, pr_type: &PluralRuleType) -> Result<PluralRule, ()> {
    match pr_type {
        PluralRuleType::CARDINAL => {
            static LANGUAGES: phf::Map<&'static str, PluralRule> = ::phf::Map {
                key: 6246114685207409605,
                disps: ::phf::Slice::Static(&[
                    (0, 4),
                    (0, 1),
                    (0, 0),
                    (11, 153),
                    (0, 35),
                    (0, 0),
                    (1, 21),
                    (0, 62),
                    (0, 6),
                    (0, 91),
                    (0, 0),
                    (0, 87),
                    (5, 4),
                    (0, 196),
                    (2, 70),
                    (0, 44),
                    (0, 5),
                    (0, 5),
                    (1, 22),
                    (1, 152),
                    (1, 92),
                    (1, 0),
                    (0, 69),
                    (0, 22),
                    (0, 0),
                    (2, 200),
                    (1, 113),
                    (0, 1),
                    (2, 176),
                    (1, 70),
                    (6, 132),
                    (3, 170),
                    (29, 158),
                    (0, 21),
                    (0, 0),
                    (0, 154),
                    (6, 141),
                    (0, 35),
                    (39, 34),
                    (21, 134),
                    (0, 33),
                    (0, 167),
                ]),
                entries: ::phf::Slice::Static(&[
                    ("lkt", {
                        fn rule_lkt(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_lkt
                    }),
                    ("ve", {
                        fn rule_ve(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ve
                    }),
                    ("dv", {
                        fn rule_dv(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_dv
                    }),
                    ("wa", {
                        fn rule_wa(po: &PluralOperands) -> PluralCategory {
                            if (matches!(po.i, 0..=1) && po.f == 0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_wa
                    }),
                    ("sc", {
                        fn rule_sc(po: &PluralOperands) -> PluralCategory {
                            if (po.i == 1 && po.v == 0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_sc
                    }),
                    ("sah", {
                        fn rule_sah(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_sah
                    }),
                    ("sr", {
                        fn rule_sr(po: &PluralOperands) -> PluralCategory {
                            if (po.v == 0
                                && matches!(po.i % 10, 2..=4)
                                && !matches!(po.i % 100, 12..=14))
                                || (matches!(po.f % 10, 2..=4) && !matches!(po.f % 100, 12..=14))
                            {
                                PluralCategory::FEW
                            } else if (po.v == 0 && po.i % 10 == 1 && po.i % 100 != 11)
                                || (po.f % 10 == 1 && po.f % 100 != 11)
                            {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_sr
                    }),
                    ("bh", {
                        fn rule_bh(po: &PluralOperands) -> PluralCategory {
                            if (matches!(po.i, 0..=1) && po.f == 0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_bh
                    }),
                    ("it", {
                        fn rule_it(po: &PluralOperands) -> PluralCategory {
                            if (po.i == 1 && po.v == 0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_it
                    }),
                    ("ff", {
                        fn rule_ff(po: &PluralOperands) -> PluralCategory {
                            if (po.i == 0 || po.i == 1) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ff
                    }),
                    ("ml", {
                        fn rule_ml(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ml
                    }),
                    ("nah", {
                        fn rule_nah(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_nah
                    }),
                    ("nb", {
                        fn rule_nb(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_nb
                    }),
                    ("scn", {
                        fn rule_scn(po: &PluralOperands) -> PluralCategory {
                            if (po.i == 1 && po.v == 0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_scn
                    }),
                    ("mg", {
                        fn rule_mg(po: &PluralOperands) -> PluralCategory {
                            if (matches!(po.i, 0..=1) && po.f == 0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_mg
                    }),
                    ("fur", {
                        fn rule_fur(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_fur
                    }),
                    ("xh", {
                        fn rule_xh(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_xh
                    }),
                    ("wae", {
                        fn rule_wae(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_wae
                    }),
                    ("ku", {
                        fn rule_ku(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ku
                    }),
                    ("kk", {
                        fn rule_kk(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_kk
                    }),
                    ("yi", {
                        fn rule_yi(po: &PluralOperands) -> PluralCategory {
                            if (po.i == 1 && po.v == 0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_yi
                    }),
                    ("pa", {
                        fn rule_pa(po: &PluralOperands) -> PluralCategory {
                            if (matches!(po.i, 0..=1) && po.f == 0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_pa
                    }),
                    ("ia", {
                        fn rule_ia(po: &PluralOperands) -> PluralCategory {
                            if (po.i == 1 && po.v == 0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ia
                    }),
                    ("tzm", {
                        fn rule_tzm(po: &PluralOperands) -> PluralCategory {
                            if (matches!(po.i, 0..=1) && po.f == 0)
                                || (matches!(po.i, 11..=99) && po.f == 0)
                            {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_tzm
                    }),
                    ("sk", {
                        fn rule_sk(po: &PluralOperands) -> PluralCategory {
                            if (matches!(po.i, 2..=4) && po.v == 0) {
                                PluralCategory::FEW
                            } else if (po.v != 0) {
                                PluralCategory::MANY
                            } else if (po.i == 1 && po.v == 0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_sk
                    }),
                    ("saq", {
                        fn rule_saq(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_saq
                    }),
                    ("sh", {
                        fn rule_sh(po: &PluralOperands) -> PluralCategory {
                            if (po.v == 0
                                && matches!(po.i % 10, 2..=4)
                                && !matches!(po.i % 100, 12..=14))
                                || (matches!(po.f % 10, 2..=4) && !matches!(po.f % 100, 12..=14))
                            {
                                PluralCategory::FEW
                            } else if (po.v == 0 && po.i % 10 == 1 && po.i % 100 != 11)
                                || (po.f % 10 == 1 && po.f % 100 != 11)
                            {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_sh
                    }),
                    ("dsb", {
                        fn rule_dsb(po: &PluralOperands) -> PluralCategory {
                            if (po.v == 0 && matches!(po.i % 100, 3..=4))
                                || (matches!(po.f % 100, 3..=4))
                            {
                                PluralCategory::FEW
                            } else if (po.v == 0 && po.i % 100 == 1) || (po.f % 100 == 1) {
                                PluralCategory::ONE
                            } else if (po.v == 0 && po.i % 100 == 2) || (po.f % 100 == 2) {
                                PluralCategory::TWO
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_dsb
                    }),
                    ("pt", {
                        fn rule_pt(po: &PluralOperands) -> PluralCategory {
                            if (matches!(po.i, 0..=1)) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_pt
                    }),
                    ("hsb", {
                        fn rule_hsb(po: &PluralOperands) -> PluralCategory {
                            if (po.v == 0 && matches!(po.i % 100, 3..=4))
                                || (matches!(po.f % 100, 3..=4))
                            {
                                PluralCategory::FEW
                            } else if (po.v == 0 && po.i % 100 == 1) || (po.f % 100 == 1) {
                                PluralCategory::ONE
                            } else if (po.v == 0 && po.i % 100 == 2) || (po.f % 100 == 2) {
                                PluralCategory::TWO
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_hsb
                    }),
                    ("mgo", {
                        fn rule_mgo(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_mgo
                    }),
                    ("kw", {
                        fn rule_kw(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else if (po.n == 2.0) {
                                PluralCategory::TWO
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_kw
                    }),
                    ("be", {
                        fn rule_be(po: &PluralOperands) -> PluralCategory {
                            if (matches!(po.i, 2..=4) && !matches!(po.i, 12..=14)) {
                                PluralCategory::FEW
                            } else if (po.i % 10 == 0)
                                || (matches!(po.i, 5..=9))
                                || (matches!(po.i, 11..=14))
                            {
                                PluralCategory::MANY
                            } else if (po.i % 10 == 1 && po.i % 100 != 11) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_be
                    }),
                    ("st", {
                        fn rule_st(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_st
                    }),
                    ("ss", {
                        fn rule_ss(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ss
                    }),
                    ("ii", {
                        fn rule_ii(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ii
                    }),
                    ("si", {
                        fn rule_si(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 0.0 || po.n == 1.0) || (po.i == 0 && po.f == 1) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_si
                    }),
                    ("mo", {
                        fn rule_mo(po: &PluralOperands) -> PluralCategory {
                            if (po.v != 0)
                                || (po.n == 0.0)
                                || (po.n != 1.0 && matches!(po.i, 1..=19))
                            {
                                PluralCategory::FEW
                            } else if (po.i == 1 && po.v == 0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_mo
                    }),
                    ("haw", {
                        fn rule_haw(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_haw
                    }),
                    ("tk", {
                        fn rule_tk(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_tk
                    }),
                    ("sms", {
                        fn rule_sms(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else if (po.n == 2.0) {
                                PluralCategory::TWO
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_sms
                    }),
                    ("iu", {
                        fn rule_iu(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else if (po.n == 2.0) {
                                PluralCategory::TWO
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_iu
                    }),
                    ("kcg", {
                        fn rule_kcg(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_kcg
                    }),
                    ("eo", {
                        fn rule_eo(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_eo
                    }),
                    ("smn", {
                        fn rule_smn(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else if (po.n == 2.0) {
                                PluralCategory::TWO
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_smn
                    }),
                    ("el", {
                        fn rule_el(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_el
                    }),
                    ("mt", {
                        fn rule_mt(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 0.0) || (matches!(po.i, 2..=10)) {
                                PluralCategory::FEW
                            } else if (matches!(po.i, 11..=19)) {
                                PluralCategory::MANY
                            } else if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_mt
                    }),
                    ("lb", {
                        fn rule_lb(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_lb
                    }),
                    ("uk", {
                        fn rule_uk(po: &PluralOperands) -> PluralCategory {
                            if (po.v == 0
                                && matches!(po.i % 10, 2..=4)
                                && !matches!(po.i % 100, 12..=14))
                            {
                                PluralCategory::FEW
                            } else if (po.v == 0 && po.i % 10 == 0)
                                || (po.v == 0 && matches!(po.i % 10, 5..=9))
                                || (po.v == 0 && matches!(po.i % 100, 11..=14))
                            {
                                PluralCategory::MANY
                            } else if (po.v == 0 && po.i % 10 == 1 && po.i % 100 != 11) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_uk
                    }),
                    ("xog", {
                        fn rule_xog(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_xog
                    }),
                    ("ksb", {
                        fn rule_ksb(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ksb
                    }),
                    ("teo", {
                        fn rule_teo(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_teo
                    }),
                    ("hy", {
                        fn rule_hy(po: &PluralOperands) -> PluralCategory {
                            if (po.i == 0 || po.i == 1) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_hy
                    }),
                    ("rm", {
                        fn rule_rm(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_rm
                    }),
                    ("nnh", {
                        fn rule_nnh(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_nnh
                    }),
                    ("hi", {
                        fn rule_hi(po: &PluralOperands) -> PluralCategory {
                            if (po.i == 0) || (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_hi
                    }),
                    ("am", {
                        fn rule_am(po: &PluralOperands) -> PluralCategory {
                            if (po.i == 0) || (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_am
                    }),
                    ("os", {
                        fn rule_os(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_os
                    }),
                    ("pap", {
                        fn rule_pap(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_pap
                    }),
                    ("ur", {
                        fn rule_ur(po: &PluralOperands) -> PluralCategory {
                            if (po.i == 1 && po.v == 0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ur
                    }),
                    ("or", {
                        fn rule_or(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_or
                    }),
                    ("ksh", {
                        fn rule_ksh(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else if (po.n == 0.0) {
                                PluralCategory::ZERO
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ksh
                    }),
                    ("ssy", {
                        fn rule_ssy(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ssy
                    }),
                    ("ji", {
                        fn rule_ji(po: &PluralOperands) -> PluralCategory {
                            if (po.i == 1 && po.v == 0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ji
                    }),
                    ("nso", {
                        fn rule_nso(po: &PluralOperands) -> PluralCategory {
                            if (matches!(po.i, 0..=1) && po.f == 0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_nso
                    }),
                    ("lt", {
                        fn rule_lt(po: &PluralOperands) -> PluralCategory {
                            if (matches!(po.i, 2..=9) && !matches!(po.i, 11..=19)) {
                                PluralCategory::FEW
                            } else if (po.f != 0) {
                                PluralCategory::MANY
                            } else if (po.i % 10 == 1 && !matches!(po.i, 11..=19)) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_lt
                    }),
                    ("tl", {
                        fn rule_tl(po: &PluralOperands) -> PluralCategory {
                            if (po.v == 0 && (po.i == 1 || po.i == 2 || po.i == 3))
                                || (po.v == 0 && po.i % 10 != 4 && po.i % 10 != 6 && po.i % 10 != 9)
                                || (po.v != 0 && po.f % 10 != 4 && po.f % 10 != 6 && po.f % 10 != 9)
                            {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_tl
                    }),
                    ("br", {
                        fn rule_br(po: &PluralOperands) -> PluralCategory {
                            if ((po.i % 10 == 9 || matches!(po.i, 3..=4))
                                && !matches!(po.i, 10..=19)
                                && !matches!(po.i, 70..=79)
                                && !matches!(po.i, 90..=99))
                            {
                                PluralCategory::FEW
                            } else if (po.n != 0.0 && po.i % 1000000 == 0) {
                                PluralCategory::MANY
                            } else if (po.i % 10 == 1
                                && po.i % 100 != 11
                                && po.i % 100 != 71
                                && po.i % 100 != 91)
                            {
                                PluralCategory::ONE
                            } else if (po.i % 10 == 2
                                && po.i % 100 != 12
                                && po.i % 100 != 72
                                && po.i % 100 != 92)
                            {
                                PluralCategory::TWO
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_br
                    }),
                    ("lv", {
                        fn rule_lv(po: &PluralOperands) -> PluralCategory {
                            if (po.i % 10 == 1 && po.i % 100 != 11)
                                || (po.v == 2 && po.f % 10 == 1 && po.f % 100 != 11)
                                || (po.v != 2 && po.f % 10 == 1)
                            {
                                PluralCategory::ONE
                            } else if (po.i % 10 == 0)
                                || (matches!(po.i, 11..=19))
                                || (po.v == 2 && matches!(po.f % 100, 11..=19))
                            {
                                PluralCategory::ZERO
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_lv
                    }),
                    ("as", {
                        fn rule_as(po: &PluralOperands) -> PluralCategory {
                            if (po.i == 0) || (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_as
                    }),
                    ("de", {
                        fn rule_de(po: &PluralOperands) -> PluralCategory {
                            if (po.i == 1 && po.v == 0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_de
                    }),
                    ("ps", {
                        fn rule_ps(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ps
                    }),
                    ("th", {
                        fn rule_th(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_th
                    }),
                    ("ro", {
                        fn rule_ro(po: &PluralOperands) -> PluralCategory {
                            if (po.v != 0)
                                || (po.n == 0.0)
                                || (po.n != 1.0 && matches!(po.i, 1..=19))
                            {
                                PluralCategory::FEW
                            } else if (po.i == 1 && po.v == 0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ro
                    }),
                    ("kde", {
                        fn rule_kde(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_kde
                    }),
                    ("rof", {
                        fn rule_rof(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_rof
                    }),
                    ("kaj", {
                        fn rule_kaj(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_kaj
                    }),
                    ("kn", {
                        fn rule_kn(po: &PluralOperands) -> PluralCategory {
                            if (po.i == 0) || (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_kn
                    }),
                    ("cgg", {
                        fn rule_cgg(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_cgg
                    }),
                    ("sl", {
                        fn rule_sl(po: &PluralOperands) -> PluralCategory {
                            if (po.v == 0 && matches!(po.i % 100, 3..=4)) || (po.v != 0) {
                                PluralCategory::FEW
                            } else if (po.v == 0 && po.i % 100 == 1) {
                                PluralCategory::ONE
                            } else if (po.v == 0 && po.i % 100 == 2) {
                                PluralCategory::TWO
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_sl
                    }),
                    ("gu", {
                        fn rule_gu(po: &PluralOperands) -> PluralCategory {
                            if (po.i == 0) || (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_gu
                    }),
                    ("lo", {
                        fn rule_lo(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_lo
                    }),
                    ("io", {
                        fn rule_io(po: &PluralOperands) -> PluralCategory {
                            if (po.i == 1 && po.v == 0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_io
                    }),
                    ("kab", {
                        fn rule_kab(po: &PluralOperands) -> PluralCategory {
                            if (po.i == 0 || po.i == 1) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_kab
                    }),
                    ("km", {
                        fn rule_km(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_km
                    }),
                    ("brx", {
                        fn rule_brx(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_brx
                    }),
                    ("eu", {
                        fn rule_eu(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_eu
                    }),
                    ("pl", {
                        fn rule_pl(po: &PluralOperands) -> PluralCategory {
                            if (po.v == 0
                                && matches!(po.i % 10, 2..=4)
                                && !matches!(po.i % 100, 12..=14))
                            {
                                PluralCategory::FEW
                            } else if (po.v == 0 && po.i != 1 && matches!(po.i % 10, 0..=1))
                                || (po.v == 0 && matches!(po.i % 10, 5..=9))
                                || (po.v == 0 && matches!(po.i % 100, 12..=14))
                            {
                                PluralCategory::MANY
                            } else if (po.i == 1 && po.v == 0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_pl
                    }),
                    ("se", {
                        fn rule_se(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else if (po.n == 2.0) {
                                PluralCategory::TWO
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_se
                    }),
                    ("ha", {
                        fn rule_ha(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ha
                    }),
                    ("mr", {
                        fn rule_mr(po: &PluralOperands) -> PluralCategory {
                            if (po.i == 0) || (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_mr
                    }),
                    ("asa", {
                        fn rule_asa(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_asa
                    }),
                    ("mn", {
                        fn rule_mn(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_mn
                    }),
                    ("ru", {
                        fn rule_ru(po: &PluralOperands) -> PluralCategory {
                            if (po.v == 0
                                && matches!(po.i % 10, 2..=4)
                                && !matches!(po.i % 100, 12..=14))
                            {
                                PluralCategory::FEW
                            } else if (po.v == 0 && po.i % 10 == 0)
                                || (po.v == 0 && matches!(po.i % 10, 5..=9))
                                || (po.v == 0 && matches!(po.i % 100, 11..=14))
                            {
                                PluralCategory::MANY
                            } else if (po.v == 0 && po.i % 10 == 1 && po.i % 100 != 11) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ru
                    }),
                    ("hr", {
                        fn rule_hr(po: &PluralOperands) -> PluralCategory {
                            if (po.v == 0
                                && matches!(po.i % 10, 2..=4)
                                && !matches!(po.i % 100, 12..=14))
                                || (matches!(po.f % 10, 2..=4) && !matches!(po.f % 100, 12..=14))
                            {
                                PluralCategory::FEW
                            } else if (po.v == 0 && po.i % 10 == 1 && po.i % 100 != 11)
                                || (po.f % 10 == 1 && po.f % 100 != 11)
                            {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_hr
                    }),
                    ("smj", {
                        fn rule_smj(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else if (po.n == 2.0) {
                                PluralCategory::TWO
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_smj
                    }),
                    ("dz", {
                        fn rule_dz(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_dz
                    }),
                    ("seh", {
                        fn rule_seh(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_seh
                    }),
                    ("cs", {
                        fn rule_cs(po: &PluralOperands) -> PluralCategory {
                            if (matches!(po.i, 2..=4) && po.v == 0) {
                                PluralCategory::FEW
                            } else if (po.v != 0) {
                                PluralCategory::MANY
                            } else if (po.i == 1 && po.v == 0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_cs
                    }),
                    ("mk", {
                        fn rule_mk(po: &PluralOperands) -> PluralCategory {
                            if (po.v == 0 && po.i % 10 == 1 && po.i % 100 != 11)
                                || (po.f % 10 == 1 && po.f % 100 != 11)
                            {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_mk
                    }),
                    ("chr", {
                        fn rule_chr(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_chr
                    }),
                    ("ms", {
                        fn rule_ms(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ms
                    }),
                    ("ts", {
                        fn rule_ts(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ts
                    }),
                    ("iw", {
                        fn rule_iw(po: &PluralOperands) -> PluralCategory {
                            if (po.v == 0 && !matches!(po.i, 0..=10) && po.f == 0 && po.i % 10 == 0)
                            {
                                PluralCategory::MANY
                            } else if (po.i == 1 && po.v == 0) {
                                PluralCategory::ONE
                            } else if (po.i == 2 && po.v == 0) {
                                PluralCategory::TWO
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_iw
                    }),
                    ("sw", {
                        fn rule_sw(po: &PluralOperands) -> PluralCategory {
                            if (po.i == 1 && po.v == 0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_sw
                    }),
                    ("ky", {
                        fn rule_ky(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ky
                    }),
                    ("bn", {
                        fn rule_bn(po: &PluralOperands) -> PluralCategory {
                            if (po.i == 0) || (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_bn
                    }),
                    ("ca", {
                        fn rule_ca(po: &PluralOperands) -> PluralCategory {
                            if (po.i == 1 && po.v == 0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ca
                    }),
                    ("te", {
                        fn rule_te(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_te
                    }),
                    ("ses", {
                        fn rule_ses(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ses
                    }),
                    ("fa", {
                        fn rule_fa(po: &PluralOperands) -> PluralCategory {
                            if (po.i == 0) || (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_fa
                    }),
                    ("bem", {
                        fn rule_bem(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_bem
                    }),
                    ("is", {
                        fn rule_is(po: &PluralOperands) -> PluralCategory {
                            if (po.t == 0 && po.i % 10 == 1 && po.i % 100 != 11) || (po.t != 0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_is
                    }),
                    ("fi", {
                        fn rule_fi(po: &PluralOperands) -> PluralCategory {
                            if (po.i == 1 && po.v == 0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_fi
                    }),
                    ("en", {
                        fn rule_en(po: &PluralOperands) -> PluralCategory {
                            if (po.i == 1 && po.v == 0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_en
                    }),
                    ("zu", {
                        fn rule_zu(po: &PluralOperands) -> PluralCategory {
                            if (po.i == 0) || (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_zu
                    }),
                    ("uz", {
                        fn rule_uz(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_uz
                    }),
                    ("es", {
                        fn rule_es(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_es
                    }),
                    ("ak", {
                        fn rule_ak(po: &PluralOperands) -> PluralCategory {
                            if (matches!(po.i, 0..=1) && po.f == 0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ak
                    }),
                    ("ln", {
                        fn rule_ln(po: &PluralOperands) -> PluralCategory {
                            if (matches!(po.i, 0..=1) && po.f == 0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ln
                    }),
                    ("shi", {
                        fn rule_shi(po: &PluralOperands) -> PluralCategory {
                            if (matches!(po.i, 2..=10) && po.f == 0) {
                                PluralCategory::FEW
                            } else if (po.i == 0) || (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_shi
                    }),
                    ("gd", {
                        fn rule_gd(po: &PluralOperands) -> PluralCategory {
                            if (matches!(po.i, 3..=10) && po.f == 0
                                || matches!(po.i, 13..=19) && po.f == 0)
                            {
                                PluralCategory::FEW
                            } else if (po.n == 1.0 || po.n == 11.0) {
                                PluralCategory::ONE
                            } else if (po.n == 2.0 || po.n == 12.0) {
                                PluralCategory::TWO
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_gd
                    }),
                    ("bm", {
                        fn rule_bm(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_bm
                    }),
                    ("no", {
                        fn rule_no(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_no
                    }),
                    ("jmc", {
                        fn rule_jmc(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_jmc
                    }),
                    ("nyn", {
                        fn rule_nyn(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_nyn
                    }),
                    ("syr", {
                        fn rule_syr(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_syr
                    }),
                    ("nqo", {
                        fn rule_nqo(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_nqo
                    }),
                    ("bo", {
                        fn rule_bo(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_bo
                    }),
                    ("ast", {
                        fn rule_ast(po: &PluralOperands) -> PluralCategory {
                            if (po.i == 1 && po.v == 0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ast
                    }),
                    ("fil", {
                        fn rule_fil(po: &PluralOperands) -> PluralCategory {
                            if (po.v == 0 && (po.i == 1 || po.i == 2 || po.i == 3))
                                || (po.v == 0 && po.i % 10 != 4 && po.i % 10 != 6 && po.i % 10 != 9)
                                || (po.v != 0 && po.f % 10 != 4 && po.f % 10 != 6 && po.f % 10 != 9)
                            {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_fil
                    }),
                    ("ars", {
                        fn rule_ars(po: &PluralOperands) -> PluralCategory {
                            if (matches!(po.i, 3..=10)) {
                                PluralCategory::FEW
                            } else if (matches!(po.i, 11..=99)) {
                                PluralCategory::MANY
                            } else if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else if (po.n == 2.0) {
                                PluralCategory::TWO
                            } else if (po.n == 0.0) {
                                PluralCategory::ZERO
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ars
                    }),
                    ("nl", {
                        fn rule_nl(po: &PluralOperands) -> PluralCategory {
                            if (po.i == 1 && po.v == 0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_nl
                    }),
                    ("pt-PT", {
                        fn rule_pt_pt(po: &PluralOperands) -> PluralCategory {
                            if (po.i == 1 && po.v == 0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_pt_pt
                    }),
                    ("jw", {
                        fn rule_jw(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_jw
                    }),
                    ("vo", {
                        fn rule_vo(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_vo
                    }),
                    ("kl", {
                        fn rule_kl(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_kl
                    }),
                    ("smi", {
                        fn rule_smi(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else if (po.n == 2.0) {
                                PluralCategory::TWO
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_smi
                    }),
                    ("af", {
                        fn rule_af(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_af
                    }),
                    ("ig", {
                        fn rule_ig(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ig
                    }),
                    ("sdh", {
                        fn rule_sdh(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_sdh
                    }),
                    ("so", {
                        fn rule_so(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_so
                    }),
                    ("tr", {
                        fn rule_tr(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_tr
                    }),
                    ("yo", {
                        fn rule_yo(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_yo
                    }),
                    ("kea", {
                        fn rule_kea(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_kea
                    }),
                    ("tig", {
                        fn rule_tig(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_tig
                    }),
                    ("nr", {
                        fn rule_nr(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_nr
                    }),
                    ("he", {
                        fn rule_he(po: &PluralOperands) -> PluralCategory {
                            if (po.v == 0 && !matches!(po.i, 0..=10) && po.f == 0 && po.i % 10 == 0)
                            {
                                PluralCategory::MANY
                            } else if (po.i == 1 && po.v == 0) {
                                PluralCategory::ONE
                            } else if (po.i == 2 && po.v == 0) {
                                PluralCategory::TWO
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_he
                    }),
                    ("et", {
                        fn rule_et(po: &PluralOperands) -> PluralCategory {
                            if (po.i == 1 && po.v == 0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_et
                    }),
                    ("yue", {
                        fn rule_yue(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_yue
                    }),
                    ("my", {
                        fn rule_my(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_my
                    }),
                    ("hu", {
                        fn rule_hu(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_hu
                    }),
                    ("kkj", {
                        fn rule_kkj(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_kkj
                    }),
                    ("in", {
                        fn rule_in(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_in
                    }),
                    ("ne", {
                        fn rule_ne(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ne
                    }),
                    ("rwk", {
                        fn rule_rwk(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_rwk
                    }),
                    ("prg", {
                        fn rule_prg(po: &PluralOperands) -> PluralCategory {
                            if (po.i % 10 == 1 && po.i % 100 != 11)
                                || (po.v == 2 && po.f % 10 == 1 && po.f % 100 != 11)
                                || (po.v != 2 && po.f % 10 == 1)
                            {
                                PluralCategory::ONE
                            } else if (po.i % 10 == 0)
                                || (matches!(po.i, 11..=19))
                                || (po.v == 2 && matches!(po.f % 100, 11..=19))
                            {
                                PluralCategory::ZERO
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_prg
                    }),
                    ("ckb", {
                        fn rule_ckb(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ckb
                    }),
                    ("jv", {
                        fn rule_jv(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_jv
                    }),
                    ("ny", {
                        fn rule_ny(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ny
                    }),
                    ("gsw", {
                        fn rule_gsw(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_gsw
                    }),
                    ("mas", {
                        fn rule_mas(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_mas
                    }),
                    ("ta", {
                        fn rule_ta(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ta
                    }),
                    ("da", {
                        fn rule_da(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) || (po.t != 0 && (po.i == 0 || po.i == 1)) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_da
                    }),
                    ("sd", {
                        fn rule_sd(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_sd
                    }),
                    ("gv", {
                        fn rule_gv(po: &PluralOperands) -> PluralCategory {
                            if (po.v == 0
                                && (po.i % 100 == 0
                                    || po.i % 100 == 20
                                    || po.i % 100 == 40
                                    || po.i % 100 == 60
                                    || po.i % 100 == 80))
                            {
                                PluralCategory::FEW
                            } else if (po.v != 0) {
                                PluralCategory::MANY
                            } else if (po.v == 0 && po.i % 10 == 1) {
                                PluralCategory::ONE
                            } else if (po.v == 0 && po.i % 10 == 2) {
                                PluralCategory::TWO
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_gv
                    }),
                    ("bez", {
                        fn rule_bez(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_bez
                    }),
                    ("to", {
                        fn rule_to(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_to
                    }),
                    ("ee", {
                        fn rule_ee(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ee
                    }),
                    ("ks", {
                        fn rule_ks(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ks
                    }),
                    ("cy", {
                        fn rule_cy(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 3.0) {
                                PluralCategory::FEW
                            } else if (po.n == 6.0) {
                                PluralCategory::MANY
                            } else if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else if (po.n == 2.0) {
                                PluralCategory::TWO
                            } else if (po.n == 0.0) {
                                PluralCategory::ZERO
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_cy
                    }),
                    ("ga", {
                        fn rule_ga(po: &PluralOperands) -> PluralCategory {
                            if (matches!(po.i, 3..=6) && po.f == 0) {
                                PluralCategory::FEW
                            } else if (matches!(po.i, 7..=10) && po.f == 0) {
                                PluralCategory::MANY
                            } else if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else if (po.n == 2.0) {
                                PluralCategory::TWO
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ga
                    }),
                    ("lag", {
                        fn rule_lag(po: &PluralOperands) -> PluralCategory {
                            if ((po.i == 0 || po.i == 1) && po.n != 0.0) {
                                PluralCategory::ONE
                            } else if (po.n == 0.0) {
                                PluralCategory::ZERO
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_lag
                    }),
                    ("ar", {
                        fn rule_ar(po: &PluralOperands) -> PluralCategory {
                            if (matches!(po.i, 3..=10)) {
                                PluralCategory::FEW
                            } else if (matches!(po.i, 11..=99)) {
                                PluralCategory::MANY
                            } else if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else if (po.n == 2.0) {
                                PluralCategory::TWO
                            } else if (po.n == 0.0) {
                                PluralCategory::ZERO
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ar
                    }),
                    ("sma", {
                        fn rule_sma(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else if (po.n == 2.0) {
                                PluralCategory::TWO
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_sma
                    }),
                    ("om", {
                        fn rule_om(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_om
                    }),
                    ("vun", {
                        fn rule_vun(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_vun
                    }),
                    ("fr", {
                        fn rule_fr(po: &PluralOperands) -> PluralCategory {
                            if (po.i == 0 || po.i == 1) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_fr
                    }),
                    ("jbo", {
                        fn rule_jbo(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_jbo
                    }),
                    ("root", {
                        fn rule_root(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_root
                    }),
                    ("fo", {
                        fn rule_fo(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_fo
                    }),
                    ("az", {
                        fn rule_az(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_az
                    }),
                    ("wo", {
                        fn rule_wo(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_wo
                    }),
                    ("sg", {
                        fn rule_sg(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_sg
                    }),
                    ("ko", {
                        fn rule_ko(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ko
                    }),
                    ("id", {
                        fn rule_id(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_id
                    }),
                    ("ti", {
                        fn rule_ti(po: &PluralOperands) -> PluralCategory {
                            if (matches!(po.i, 0..=1) && po.f == 0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ti
                    }),
                    ("bg", {
                        fn rule_bg(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_bg
                    }),
                    ("ja", {
                        fn rule_ja(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ja
                    }),
                    ("sn", {
                        fn rule_sn(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_sn
                    }),
                    ("nd", {
                        fn rule_nd(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_nd
                    }),
                    ("vi", {
                        fn rule_vi(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_vi
                    }),
                    ("nn", {
                        fn rule_nn(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_nn
                    }),
                    ("ka", {
                        fn rule_ka(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ka
                    }),
                    ("lg", {
                        fn rule_lg(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_lg
                    }),
                    ("sq", {
                        fn rule_sq(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_sq
                    }),
                    ("jgo", {
                        fn rule_jgo(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_jgo
                    }),
                    ("tn", {
                        fn rule_tn(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_tn
                    }),
                    ("ug", {
                        fn rule_ug(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ug
                    }),
                    ("zh", {
                        fn rule_zh(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_zh
                    }),
                    ("sv", {
                        fn rule_sv(po: &PluralOperands) -> PluralCategory {
                            if (po.i == 1 && po.v == 0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_sv
                    }),
                    ("fy", {
                        fn rule_fy(po: &PluralOperands) -> PluralCategory {
                            if (po.i == 1 && po.v == 0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_fy
                    }),
                    ("bs", {
                        fn rule_bs(po: &PluralOperands) -> PluralCategory {
                            if (po.v == 0
                                && matches!(po.i % 10, 2..=4)
                                && !matches!(po.i % 100, 12..=14))
                                || (matches!(po.f % 10, 2..=4) && !matches!(po.f % 100, 12..=14))
                            {
                                PluralCategory::FEW
                            } else if (po.v == 0 && po.i % 10 == 1 && po.i % 100 != 11)
                                || (po.f % 10 == 1 && po.f % 100 != 11)
                            {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_bs
                    }),
                    ("guw", {
                        fn rule_guw(po: &PluralOperands) -> PluralCategory {
                            if (matches!(po.i, 0..=1) && po.f == 0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_guw
                    }),
                    ("ce", {
                        fn rule_ce(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ce
                    }),
                    ("naq", {
                        fn rule_naq(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else if (po.n == 2.0) {
                                PluralCategory::TWO
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_naq
                    }),
                    ("gl", {
                        fn rule_gl(po: &PluralOperands) -> PluralCategory {
                            if (po.i == 1 && po.v == 0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_gl
                    }),
                ]),
            };
            LANGUAGES.get(lang_code).cloned().ok_or(())
        }
        PluralRuleType::ORDINAL => {
            static LANGUAGES: phf::Map<&'static str, PluralRule> = ::phf::Map {
                key: 2662727287650995926,
                disps: ::phf::Slice::Static(&[
                    (5, 79),
                    (0, 8),
                    (0, 0),
                    (0, 6),
                    (1, 2),
                    (15, 25),
                    (0, 8),
                    (1, 91),
                    (7, 4),
                    (0, 2),
                    (1, 23),
                    (0, 47),
                    (0, 4),
                    (11, 95),
                    (9, 83),
                    (0, 66),
                    (0, 13),
                    (1, 10),
                    (4, 24),
                    (7, 5),
                ]),
                entries: ::phf::Slice::Static(&[
                    ("si", {
                        fn rule_si(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_si
                    }),
                    ("scn", {
                        fn rule_scn(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 11.0 || po.n == 8.0 || po.n == 80.0 || po.n == 800.0) {
                                PluralCategory::MANY
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_scn
                    }),
                    ("ro", {
                        fn rule_ro(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ro
                    }),
                    ("ne", {
                        fn rule_ne(po: &PluralOperands) -> PluralCategory {
                            if (matches!(po.i, 1..=4) && po.f == 0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ne
                    }),
                    ("ms", {
                        fn rule_ms(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ms
                    }),
                    ("kn", {
                        fn rule_kn(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_kn
                    }),
                    ("es", {
                        fn rule_es(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_es
                    }),
                    ("ml", {
                        fn rule_ml(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ml
                    }),
                    ("da", {
                        fn rule_da(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_da
                    }),
                    ("uk", {
                        fn rule_uk(po: &PluralOperands) -> PluralCategory {
                            if (po.i % 10 == 3 && po.i % 100 != 13) {
                                PluralCategory::FEW
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_uk
                    }),
                    ("cy", {
                        fn rule_cy(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 3.0 || po.n == 4.0) {
                                PluralCategory::FEW
                            } else if (po.n == 5.0 || po.n == 6.0) {
                                PluralCategory::MANY
                            } else if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else if (po.n == 2.0) {
                                PluralCategory::TWO
                            } else if (po.n == 0.0 || po.n == 7.0 || po.n == 8.0 || po.n == 9.0) {
                                PluralCategory::ZERO
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_cy
                    }),
                    ("lo", {
                        fn rule_lo(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_lo
                    }),
                    ("hu", {
                        fn rule_hu(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0 || po.n == 5.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_hu
                    }),
                    ("gsw", {
                        fn rule_gsw(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_gsw
                    }),
                    ("fy", {
                        fn rule_fy(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_fy
                    }),
                    ("hsb", {
                        fn rule_hsb(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_hsb
                    }),
                    ("uz", {
                        fn rule_uz(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_uz
                    }),
                    ("ca", {
                        fn rule_ca(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 4.0) {
                                PluralCategory::FEW
                            } else if (po.n == 1.0 || po.n == 3.0) {
                                PluralCategory::ONE
                            } else if (po.n == 2.0) {
                                PluralCategory::TWO
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ca
                    }),
                    ("dsb", {
                        fn rule_dsb(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_dsb
                    }),
                    ("sh", {
                        fn rule_sh(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_sh
                    }),
                    ("nb", {
                        fn rule_nb(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_nb
                    }),
                    ("hr", {
                        fn rule_hr(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_hr
                    }),
                    ("ga", {
                        fn rule_ga(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ga
                    }),
                    ("hy", {
                        fn rule_hy(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_hy
                    }),
                    ("am", {
                        fn rule_am(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_am
                    }),
                    ("sw", {
                        fn rule_sw(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_sw
                    }),
                    ("lv", {
                        fn rule_lv(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_lv
                    }),
                    ("sd", {
                        fn rule_sd(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_sd
                    }),
                    ("be", {
                        fn rule_be(po: &PluralOperands) -> PluralCategory {
                            if ((po.i % 10 == 2 || po.i % 10 == 3)
                                && po.i % 100 != 12
                                && po.i % 100 != 13)
                            {
                                PluralCategory::FEW
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_be
                    }),
                    ("gu", {
                        fn rule_gu(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 4.0) {
                                PluralCategory::FEW
                            } else if (po.n == 6.0) {
                                PluralCategory::MANY
                            } else if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else if (po.n == 2.0 || po.n == 3.0) {
                                PluralCategory::TWO
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_gu
                    }),
                    ("mo", {
                        fn rule_mo(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_mo
                    }),
                    ("ps", {
                        fn rule_ps(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ps
                    }),
                    ("mn", {
                        fn rule_mn(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_mn
                    }),
                    ("ar", {
                        fn rule_ar(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ar
                    }),
                    ("bn", {
                        fn rule_bn(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 4.0) {
                                PluralCategory::FEW
                            } else if (po.n == 6.0) {
                                PluralCategory::MANY
                            } else if (po.n == 1.0
                                || po.n == 5.0
                                || po.n == 7.0
                                || po.n == 8.0
                                || po.n == 9.0
                                || po.n == 10.0)
                            {
                                PluralCategory::ONE
                            } else if (po.n == 2.0 || po.n == 3.0) {
                                PluralCategory::TWO
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_bn
                    }),
                    ("ur", {
                        fn rule_ur(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ur
                    }),
                    ("ky", {
                        fn rule_ky(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ky
                    }),
                    ("pl", {
                        fn rule_pl(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_pl
                    }),
                    ("is", {
                        fn rule_is(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_is
                    }),
                    ("et", {
                        fn rule_et(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_et
                    }),
                    ("cs", {
                        fn rule_cs(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_cs
                    }),
                    ("ce", {
                        fn rule_ce(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ce
                    }),
                    ("kk", {
                        fn rule_kk(po: &PluralOperands) -> PluralCategory {
                            if (po.i % 10 == 6)
                                || (po.i % 10 == 9)
                                || (po.i % 10 == 0 && po.n != 0.0)
                            {
                                PluralCategory::MANY
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_kk
                    }),
                    ("az", {
                        fn rule_az(po: &PluralOperands) -> PluralCategory {
                            if (po.i % 10 == 3 || po.i % 10 == 4)
                                || (po.i % 1000 == 100
                                    || po.i % 1000 == 200
                                    || po.i % 1000 == 300
                                    || po.i % 1000 == 400
                                    || po.i % 1000 == 500
                                    || po.i % 1000 == 600
                                    || po.i % 1000 == 700
                                    || po.i % 1000 == 800
                                    || po.i % 1000 == 900)
                            {
                                PluralCategory::FEW
                            } else if (po.i == 0)
                                || (po.i % 10 == 6)
                                || (po.i % 100 == 40 || po.i % 100 == 60 || po.i % 100 == 90)
                            {
                                PluralCategory::MANY
                            } else if (po.i % 10 == 1
                                || po.i % 10 == 2
                                || po.i % 10 == 5
                                || po.i % 10 == 7
                                || po.i % 10 == 8)
                                || (po.i % 100 == 20
                                    || po.i % 100 == 50
                                    || po.i % 100 == 70
                                    || po.i % 100 == 80)
                            {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_az
                    }),
                    ("ta", {
                        fn rule_ta(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ta
                    }),
                    ("nl", {
                        fn rule_nl(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_nl
                    }),
                    ("sl", {
                        fn rule_sl(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_sl
                    }),
                    ("hi", {
                        fn rule_hi(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 4.0) {
                                PluralCategory::FEW
                            } else if (po.n == 6.0) {
                                PluralCategory::MANY
                            } else if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else if (po.n == 2.0 || po.n == 3.0) {
                                PluralCategory::TWO
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_hi
                    }),
                    ("prg", {
                        fn rule_prg(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_prg
                    }),
                    ("af", {
                        fn rule_af(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_af
                    }),
                    ("ja", {
                        fn rule_ja(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ja
                    }),
                    ("pt", {
                        fn rule_pt(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_pt
                    }),
                    ("lt", {
                        fn rule_lt(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_lt
                    }),
                    ("ru", {
                        fn rule_ru(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ru
                    }),
                    ("iw", {
                        fn rule_iw(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_iw
                    }),
                    ("mr", {
                        fn rule_mr(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 4.0) {
                                PluralCategory::FEW
                            } else if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else if (po.n == 2.0 || po.n == 3.0) {
                                PluralCategory::TWO
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_mr
                    }),
                    ("gl", {
                        fn rule_gl(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_gl
                    }),
                    ("km", {
                        fn rule_km(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_km
                    }),
                    ("root", {
                        fn rule_root(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_root
                    }),
                    ("sk", {
                        fn rule_sk(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_sk
                    }),
                    ("zu", {
                        fn rule_zu(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_zu
                    }),
                    ("th", {
                        fn rule_th(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_th
                    }),
                    ("id", {
                        fn rule_id(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_id
                    }),
                    ("ko", {
                        fn rule_ko(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ko
                    }),
                    ("sc", {
                        fn rule_sc(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 11.0 || po.n == 8.0 || po.n == 80.0 || po.n == 800.0) {
                                PluralCategory::MANY
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_sc
                    }),
                    ("sq", {
                        fn rule_sq(po: &PluralOperands) -> PluralCategory {
                            if (po.i % 10 == 4 && po.i % 100 != 14) {
                                PluralCategory::MANY
                            } else if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_sq
                    }),
                    ("in", {
                        fn rule_in(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_in
                    }),
                    ("bg", {
                        fn rule_bg(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_bg
                    }),
                    ("it", {
                        fn rule_it(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 11.0 || po.n == 8.0 || po.n == 80.0 || po.n == 800.0) {
                                PluralCategory::MANY
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_it
                    }),
                    ("bs", {
                        fn rule_bs(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_bs
                    }),
                    ("ka", {
                        fn rule_ka(po: &PluralOperands) -> PluralCategory {
                            if (po.i == 0)
                                || (po.i % 100 == 40
                                    || po.i % 100 == 60
                                    || po.i % 100 == 80
                                    || matches!(po.i % 100, 2..=20))
                            {
                                PluralCategory::MANY
                            } else if (po.i == 1) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ka
                    }),
                    ("tl", {
                        fn rule_tl(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_tl
                    }),
                    ("vi", {
                        fn rule_vi(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_vi
                    }),
                    ("my", {
                        fn rule_my(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_my
                    }),
                    ("as", {
                        fn rule_as(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 4.0) {
                                PluralCategory::FEW
                            } else if (po.n == 6.0) {
                                PluralCategory::MANY
                            } else if (po.n == 1.0
                                || po.n == 5.0
                                || po.n == 7.0
                                || po.n == 8.0
                                || po.n == 9.0
                                || po.n == 10.0)
                            {
                                PluralCategory::ONE
                            } else if (po.n == 2.0 || po.n == 3.0) {
                                PluralCategory::TWO
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_as
                    }),
                    ("en", {
                        fn rule_en(po: &PluralOperands) -> PluralCategory {
                            if (po.i % 10 == 3 && po.i % 100 != 13) {
                                PluralCategory::FEW
                            } else if (po.i % 10 == 1 && po.i % 100 != 11) {
                                PluralCategory::ONE
                            } else if (po.i % 10 == 2 && po.i % 100 != 12) {
                                PluralCategory::TWO
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_en
                    }),
                    ("sv", {
                        fn rule_sv(po: &PluralOperands) -> PluralCategory {
                            if ((po.i % 10 == 1 || po.i % 10 == 2)
                                && po.i % 100 != 11
                                && po.i % 100 != 12)
                            {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_sv
                    }),
                    ("eu", {
                        fn rule_eu(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_eu
                    }),
                    ("he", {
                        fn rule_he(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_he
                    }),
                    ("fi", {
                        fn rule_fi(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_fi
                    }),
                    ("zh", {
                        fn rule_zh(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_zh
                    }),
                    ("de", {
                        fn rule_de(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_de
                    }),
                    ("el", {
                        fn rule_el(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_el
                    }),
                    ("tk", {
                        fn rule_tk(po: &PluralOperands) -> PluralCategory {
                            if (po.i % 10 == 6 || po.i % 10 == 9) || (po.n == 10.0) {
                                PluralCategory::FEW
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_tk
                    }),
                    ("sr", {
                        fn rule_sr(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_sr
                    }),
                    ("yue", {
                        fn rule_yue(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_yue
                    }),
                    ("te", {
                        fn rule_te(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_te
                    }),
                    ("fil", {
                        fn rule_fil(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_fil
                    }),
                    ("mk", {
                        fn rule_mk(po: &PluralOperands) -> PluralCategory {
                            if ((po.i % 10 == 7 || po.i % 10 == 8)
                                && po.i % 100 != 17
                                && po.i % 100 != 18)
                            {
                                PluralCategory::MANY
                            } else if (po.i % 10 == 1 && po.i % 100 != 11) {
                                PluralCategory::ONE
                            } else if (po.i % 10 == 2 && po.i % 100 != 12) {
                                PluralCategory::TWO
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_mk
                    }),
                    ("fr", {
                        fn rule_fr(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_fr
                    }),
                    ("or", {
                        fn rule_or(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 4.0) {
                                PluralCategory::FEW
                            } else if (po.n == 6.0) {
                                PluralCategory::MANY
                            } else if (po.n == 1.0
                                || po.n == 5.0
                                || matches!(po.i, 7..=9) && po.f == 0)
                            {
                                PluralCategory::ONE
                            } else if (po.n == 2.0 || po.n == 3.0) {
                                PluralCategory::TWO
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_or
                    }),
                    ("fa", {
                        fn rule_fa(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_fa
                    }),
                    ("ia", {
                        fn rule_ia(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ia
                    }),
                    ("gd", {
                        fn rule_gd(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 3.0 || po.n == 13.0) {
                                PluralCategory::FEW
                            } else if (po.n == 1.0 || po.n == 11.0) {
                                PluralCategory::ONE
                            } else if (po.n == 2.0 || po.n == 12.0) {
                                PluralCategory::TWO
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_gd
                    }),
                    ("pa", {
                        fn rule_pa(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_pa
                    }),
                    ("tr", {
                        fn rule_tr(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_tr
                    }),
                ]),
            };
            LANGUAGES.get(lang_code).cloned().ok_or(())
        }
    }
}
