#![allow(unused_variables, unused_parens)]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::float_cmp))]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::unreadable_literal))]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::nonminimal_bool))]
use super::operands::PluralOperands;
use super::{PluralCategory, PluralRuleType};
use matches::matches;
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
pub fn get_pr(lang_code: &str, pr_type: PluralRuleType) -> Result<PluralRule, ()> {
    match pr_type {
        PluralRuleType::CARDINAL => {
            static LANGUAGES: phf::Map<&'static str, PluralRule> = ::phf::Map {
                key: 3213172566270843353,
                disps: ::phf::Slice::Static(&[
                    (0, 167),
                    (0, 5),
                    (0, 27),
                    (0, 2),
                    (3, 164),
                    (1, 16),
                    (0, 150),
                    (0, 0),
                    (2, 126),
                    (0, 4),
                    (2, 33),
                    (1, 91),
                    (0, 10),
                    (1, 0),
                    (17, 13),
                    (6, 61),
                    (0, 8),
                    (1, 62),
                    (0, 5),
                    (0, 13),
                    (0, 81),
                    (0, 2),
                    (0, 7),
                    (0, 148),
                    (0, 170),
                    (1, 17),
                    (0, 71),
                    (18, 44),
                    (1, 125),
                    (0, 122),
                    (0, 1),
                    (5, 19),
                    (3, 151),
                    (5, 43),
                    (6, 68),
                    (0, 97),
                    (4, 135),
                    (11, 111),
                    (40, 90),
                    (2, 9),
                    (0, 52),
                    (10, 125),
                ]),
                entries: ::phf::Slice::Static(&[
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
                    ("to", {
                        fn rule_to(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_to
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
                    ("sg", {
                        fn rule_sg(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_sg
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
                    ("ii", {
                        fn rule_ii(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ii
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
                    ("kea", {
                        fn rule_kea(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_kea
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
                    ("my", {
                        fn rule_my(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_my
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
                    ("id", {
                        fn rule_id(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_id
                    }),
                    ("nqo", {
                        fn rule_nqo(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_nqo
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
                    ("ms", {
                        fn rule_ms(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ms
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
                    ("ses", {
                        fn rule_ses(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ses
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
                    ("zh", {
                        fn rule_zh(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_zh
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
                    ("yue", {
                        fn rule_yue(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_yue
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
                    ("ja", {
                        fn rule_ja(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ja
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
                    ("jw", {
                        fn rule_jw(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_jw
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
                    ("bm", {
                        fn rule_bm(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_bm
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
                    ("sah", {
                        fn rule_sah(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_sah
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
                    ("jv", {
                        fn rule_jv(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_jv
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
                    ("bo", {
                        fn rule_bo(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_bo
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
                    ("ig", {
                        fn rule_ig(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ig
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
                    ("lo", {
                        fn rule_lo(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_lo
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
                    ("wo", {
                        fn rule_wo(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_wo
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
                    ("vi", {
                        fn rule_vi(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_vi
                    }),
                    ("dz", {
                        fn rule_dz(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_dz
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
                    ("jbo", {
                        fn rule_jbo(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_jbo
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
                    ("ko", {
                        fn rule_ko(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ko
                    }),
                    ("lkt", {
                        fn rule_lkt(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_lkt
                    }),
                    ("km", {
                        fn rule_km(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_km
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
                    ("root", {
                        fn rule_root(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_root
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
                    ("yo", {
                        fn rule_yo(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_yo
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
                    ("kde", {
                        fn rule_kde(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_kde
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
                    ("th", {
                        fn rule_th(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_th
                    }),
                    ("in", {
                        fn rule_in(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_in
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
                ]),
            };
            LANGUAGES.get(lang_code).cloned().ok_or(())
        }
        PluralRuleType::ORDINAL => {
            static LANGUAGES: phf::Map<&'static str, PluralRule> = ::phf::Map {
                key: 6925680744564340301,
                disps: ::phf::Slice::Static(&[
                    (0, 8),
                    (1, 77),
                    (1, 78),
                    (11, 42),
                    (5, 59),
                    (0, 17),
                    (3, 73),
                    (1, 83),
                    (5, 92),
                    (0, 13),
                    (1, 43),
                    (0, 10),
                    (28, 46),
                    (0, 0),
                    (0, 7),
                    (0, 43),
                    (0, 0),
                    (0, 37),
                    (28, 89),
                    (2, 19),
                ]),
                entries: ::phf::Slice::Static(&[
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
                    ("pt", {
                        fn rule_pt(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_pt
                    }),
                    ("el", {
                        fn rule_el(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_el
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
                    ("sd", {
                        fn rule_sd(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_sd
                    }),
                    ("kn", {
                        fn rule_kn(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_kn
                    }),
                    ("ky", {
                        fn rule_ky(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ky
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
                    ("gsw", {
                        fn rule_gsw(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_gsw
                    }),
                    ("uz", {
                        fn rule_uz(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_uz
                    }),
                    ("ar", {
                        fn rule_ar(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ar
                    }),
                    ("prg", {
                        fn rule_prg(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_prg
                    }),
                    ("ko", {
                        fn rule_ko(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ko
                    }),
                    ("nb", {
                        fn rule_nb(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_nb
                    }),
                    ("sk", {
                        fn rule_sk(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_sk
                    }),
                    ("bg", {
                        fn rule_bg(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_bg
                    }),
                    ("fi", {
                        fn rule_fi(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_fi
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
                    ("nl", {
                        fn rule_nl(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_nl
                    }),
                    ("ur", {
                        fn rule_ur(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ur
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
                    ("lv", {
                        fn rule_lv(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_lv
                    }),
                    ("fa", {
                        fn rule_fa(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_fa
                    }),
                    ("de", {
                        fn rule_de(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_de
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
                    ("fy", {
                        fn rule_fy(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_fy
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
                    ("ru", {
                        fn rule_ru(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ru
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
                    ("am", {
                        fn rule_am(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_am
                    }),
                    ("da", {
                        fn rule_da(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_da
                    }),
                    ("km", {
                        fn rule_km(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_km
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
                            if (matches!(po.i, 1..=4) && po.f == 0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ne
                    }),
                    ("ps", {
                        fn rule_ps(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ps
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
                    ("he", {
                        fn rule_he(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_he
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
                    ("zh", {
                        fn rule_zh(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_zh
                    }),
                    ("es", {
                        fn rule_es(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_es
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
                    ("root", {
                        fn rule_root(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_root
                    }),
                    ("sh", {
                        fn rule_sh(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_sh
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
                    ("hsb", {
                        fn rule_hsb(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_hsb
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
                    ("yue", {
                        fn rule_yue(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_yue
                    }),
                    ("id", {
                        fn rule_id(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_id
                    }),
                    ("iw", {
                        fn rule_iw(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_iw
                    }),
                    ("th", {
                        fn rule_th(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_th
                    }),
                    ("ta", {
                        fn rule_ta(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ta
                    }),
                    ("sl", {
                        fn rule_sl(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_sl
                    }),
                    ("mn", {
                        fn rule_mn(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_mn
                    }),
                    ("ce", {
                        fn rule_ce(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ce
                    }),
                    ("pl", {
                        fn rule_pl(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_pl
                    }),
                    ("te", {
                        fn rule_te(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_te
                    }),
                    ("eu", {
                        fn rule_eu(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_eu
                    }),
                    ("my", {
                        fn rule_my(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_my
                    }),
                    ("et", {
                        fn rule_et(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_et
                    }),
                    ("ia", {
                        fn rule_ia(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ia
                    }),
                    ("si", {
                        fn rule_si(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_si
                    }),
                    ("lt", {
                        fn rule_lt(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_lt
                    }),
                    ("dsb", {
                        fn rule_dsb(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_dsb
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
                    ("ja", {
                        fn rule_ja(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ja
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
                    ("is", {
                        fn rule_is(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_is
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
                    ("pa", {
                        fn rule_pa(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_pa
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
                    ("tr", {
                        fn rule_tr(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_tr
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
                    ("ml", {
                        fn rule_ml(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ml
                    }),
                    ("sr", {
                        fn rule_sr(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_sr
                    }),
                    ("gl", {
                        fn rule_gl(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_gl
                    }),
                    ("hr", {
                        fn rule_hr(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_hr
                    }),
                    ("sw", {
                        fn rule_sw(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_sw
                    }),
                    ("af", {
                        fn rule_af(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_af
                    }),
                    ("bs", {
                        fn rule_bs(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_bs
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
                    ("zu", {
                        fn rule_zu(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_zu
                    }),
                    ("cs", {
                        fn rule_cs(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_cs
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
                ]),
            };
            LANGUAGES.get(lang_code).cloned().ok_or(())
        }
    }
}
