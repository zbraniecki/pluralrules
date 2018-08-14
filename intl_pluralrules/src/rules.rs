#![allow(unused_variables, unused_parens)]
#[cfg_attr(feature = "cargo-clippy", allow(float_cmp))]
extern crate matches;
use super::operands::PluralOperands;
use super::{PluralCategory, PluralRuleType};
use phf;
pub type PluralRule = fn(PluralOperands) -> PluralCategory;
pub static CLDR_VERSION: usize = 33;
#[cfg_attr(tarpaulin, skip)]
pub fn get_locales(pr_type: PluralRuleType) -> &'static [&'static str] {
    match pr_type {
        PluralRuleType::CARDINAL => &[
            "af", "ak", "am", "ar", "ars", "as", "asa", "ast", "az", "be", "bem", "bez", "bg",
            "bh", "bm", "bn", "bo", "br", "brx", "bs", "ca", "ce", "cgg", "chr", "ckb", "cs", "cy",
            "da", "de", "dsb", "dv", "dz", "ee", "el", "en", "eo", "es", "et", "eu", "fa", "ff",
            "fi", "fil", "fo", "fr", "fur", "fy", "ga", "gd", "gl", "gsw", "gu", "guw", "gv", "ha",
            "haw", "he", "hi", "hr", "hsb", "hu", "hy", "id", "ig", "ii", "in", "io", "is", "it",
            "iu", "iw", "ja", "jbo", "jgo", "ji", "jmc", "jv", "jw", "ka", "kab", "kaj", "kcg",
            "kde", "kea", "kk", "kkj", "kl", "km", "kn", "ko", "ks", "ksb", "ksh", "ku", "kw",
            "ky", "lag", "lb", "lg", "lkt", "ln", "lo", "lt", "lv", "mas", "mg", "mgo", "mk", "ml",
            "mn", "mo", "mr", "ms", "mt", "my", "nah", "naq", "nb", "nd", "ne", "nl", "nn", "nnh",
            "no", "nqo", "nr", "nso", "ny", "nyn", "om", "or", "os", "pa", "pap", "pl", "prg",
            "ps", "pt", "pt-PT", "rm", "ro", "rof", "root", "ru", "rwk", "sah", "saq", "scn", "sd",
            "sdh", "se", "seh", "ses", "sg", "sh", "shi", "si", "sk", "sl", "sma", "smi", "smj",
            "smn", "sms", "sn", "so", "sq", "sr", "ss", "ssy", "st", "sv", "sw", "syr", "ta", "te",
            "teo", "th", "ti", "tig", "tk", "tl", "tn", "to", "tr", "ts", "tzm", "ug", "uk", "ur",
            "uz", "ve", "vi", "vo", "vun", "wa", "wae", "wo", "xh", "xog", "yi", "yo", "yue", "zh",
            "zu",
        ],
        PluralRuleType::ORDINAL => &[
            "af", "am", "ar", "as", "az", "be", "bg", "bn", "bs", "ca", "ce", "cs", "cy", "da",
            "de", "dsb", "el", "en", "es", "et", "eu", "fa", "fi", "fil", "fr", "fy", "ga", "gl",
            "gsw", "gu", "he", "hi", "hr", "hsb", "hu", "hy", "id", "in", "is", "it", "iw", "ja",
            "ka", "kk", "km", "kn", "ko", "ky", "lo", "lt", "lv", "mk", "ml", "mn", "mo", "mr",
            "ms", "my", "nb", "ne", "nl", "or", "pa", "pl", "prg", "ps", "pt", "ro", "root", "ru",
            "scn", "sd", "sh", "si", "sk", "sl", "sq", "sr", "sv", "sw", "ta", "te", "th", "tk",
            "tl", "tr", "uk", "ur", "uz", "vi", "yue", "zh", "zu",
        ],
    }
}
#[cfg_attr(tarpaulin, skip)]
pub fn get_pr(lang_code: &str, pr_type: PluralRuleType) -> Result<PluralRule, ()> {
    match pr_type {
        PluralRuleType::CARDINAL => {
            static LANGUAGES: phf::Map<&'static str, PluralRule> = ::phf::Map {
                key: 1897749892740154578,
                disps: ::phf::Slice::Static(&[
                    (0, 9),
                    (0, 3),
                    (0, 113),
                    (1, 0),
                    (0, 143),
                    (0, 14),
                    (0, 57),
                    (5, 154),
                    (0, 69),
                    (0, 123),
                    (0, 157),
                    (0, 1),
                    (0, 176),
                    (0, 52),
                    (0, 21),
                    (0, 3),
                    (3, 79),
                    (3, 26),
                    (6, 23),
                    (0, 12),
                    (1, 0),
                    (0, 151),
                    (0, 168),
                    (0, 21),
                    (2, 15),
                    (0, 125),
                    (3, 145),
                    (1, 33),
                    (0, 102),
                    (0, 173),
                    (3, 102),
                    (23, 64),
                    (0, 99),
                    (0, 113),
                    (2, 29),
                    (7, 145),
                    (0, 0),
                    (0, 19),
                    (0, 3),
                    (0, 70),
                    (0, 4),
                ]),
                entries: ::phf::Slice::Static(&[
                    ("ms", {
                        fn rule_ms(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ms
                    }),
                    ("es", {
                        fn rule_es(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_es
                    }),
                    ("haw", {
                        fn rule_haw(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_haw
                    }),
                    ("kn", {
                        fn rule_kn(po: PluralOperands) -> PluralCategory {
                            if (po.i == 0) || (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_kn
                    }),
                    ("he", {
                        fn rule_he(po: PluralOperands) -> PluralCategory {
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
                    ("ja", {
                        fn rule_ja(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ja
                    }),
                    ("ce", {
                        fn rule_ce(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ce
                    }),
                    ("ksb", {
                        fn rule_ksb(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ksb
                    }),
                    ("ksh", {
                        fn rule_ksh(po: PluralOperands) -> PluralCategory {
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
                    ("my", {
                        fn rule_my(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_my
                    }),
                    ("rof", {
                        fn rule_rof(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_rof
                    }),
                    ("mg", {
                        fn rule_mg(po: PluralOperands) -> PluralCategory {
                            if (matches!(po.i, 0..=1) && po.f == 0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_mg
                    }),
                    ("et", {
                        fn rule_et(po: PluralOperands) -> PluralCategory {
                            if (po.i == 1 && po.v == 0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_et
                    }),
                    ("yo", {
                        fn rule_yo(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_yo
                    }),
                    ("ny", {
                        fn rule_ny(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ny
                    }),
                    ("kab", {
                        fn rule_kab(po: PluralOperands) -> PluralCategory {
                            if (po.i == 0 || po.i == 1) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_kab
                    }),
                    ("om", {
                        fn rule_om(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_om
                    }),
                    ("kaj", {
                        fn rule_kaj(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_kaj
                    }),
                    ("zu", {
                        fn rule_zu(po: PluralOperands) -> PluralCategory {
                            if (po.i == 0) || (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_zu
                    }),
                    ("ee", {
                        fn rule_ee(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ee
                    }),
                    ("seh", {
                        fn rule_seh(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_seh
                    }),
                    ("bs", {
                        fn rule_bs(po: PluralOperands) -> PluralCategory {
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
                    ("syr", {
                        fn rule_syr(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_syr
                    }),
                    ("sdh", {
                        fn rule_sdh(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_sdh
                    }),
                    ("sn", {
                        fn rule_sn(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_sn
                    }),
                    ("ssy", {
                        fn rule_ssy(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ssy
                    }),
                    ("kw", {
                        fn rule_kw(po: PluralOperands) -> PluralCategory {
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
                    ("ha", {
                        fn rule_ha(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ha
                    }),
                    ("ses", {
                        fn rule_ses(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ses
                    }),
                    ("shi", {
                        fn rule_shi(po: PluralOperands) -> PluralCategory {
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
                    ("fy", {
                        fn rule_fy(po: PluralOperands) -> PluralCategory {
                            if (po.i == 1 && po.v == 0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_fy
                    }),
                    ("is", {
                        fn rule_is(po: PluralOperands) -> PluralCategory {
                            if (po.t == 0 && po.i % 10 == 1 && po.i % 100 != 11) || (po.t != 0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_is
                    }),
                    ("gsw", {
                        fn rule_gsw(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_gsw
                    }),
                    ("jw", {
                        fn rule_jw(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_jw
                    }),
                    ("mo", {
                        fn rule_mo(po: PluralOperands) -> PluralCategory {
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
                    ("da", {
                        fn rule_da(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) || (po.t != 0 && (po.i == 0 || po.i == 1)) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_da
                    }),
                    ("tr", {
                        fn rule_tr(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_tr
                    }),
                    ("cs", {
                        fn rule_cs(po: PluralOperands) -> PluralCategory {
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
                    ("iw", {
                        fn rule_iw(po: PluralOperands) -> PluralCategory {
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
                    ("km", {
                        fn rule_km(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_km
                    }),
                    ("ug", {
                        fn rule_ug(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ug
                    }),
                    ("lag", {
                        fn rule_lag(po: PluralOperands) -> PluralCategory {
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
                    ("ka", {
                        fn rule_ka(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ka
                    }),
                    ("ru", {
                        fn rule_ru(po: PluralOperands) -> PluralCategory {
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
                    ("ky", {
                        fn rule_ky(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ky
                    }),
                    ("wae", {
                        fn rule_wae(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_wae
                    }),
                    ("ve", {
                        fn rule_ve(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ve
                    }),
                    ("sg", {
                        fn rule_sg(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_sg
                    }),
                    ("yue", {
                        fn rule_yue(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_yue
                    }),
                    ("ig", {
                        fn rule_ig(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ig
                    }),
                    ("ckb", {
                        fn rule_ckb(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ckb
                    }),
                    ("pap", {
                        fn rule_pap(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_pap
                    }),
                    ("fur", {
                        fn rule_fur(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_fur
                    }),
                    ("hu", {
                        fn rule_hu(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_hu
                    }),
                    ("pl", {
                        fn rule_pl(po: PluralOperands) -> PluralCategory {
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
                    ("rm", {
                        fn rule_rm(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_rm
                    }),
                    ("nb", {
                        fn rule_nb(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_nb
                    }),
                    ("pa", {
                        fn rule_pa(po: PluralOperands) -> PluralCategory {
                            if (matches!(po.i, 0..=1) && po.f == 0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_pa
                    }),
                    ("be", {
                        fn rule_be(po: PluralOperands) -> PluralCategory {
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
                    ("wa", {
                        fn rule_wa(po: PluralOperands) -> PluralCategory {
                            if (matches!(po.i, 0..=1) && po.f == 0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_wa
                    }),
                    ("bg", {
                        fn rule_bg(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_bg
                    }),
                    ("smj", {
                        fn rule_smj(po: PluralOperands) -> PluralCategory {
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
                    ("jmc", {
                        fn rule_jmc(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_jmc
                    }),
                    ("jbo", {
                        fn rule_jbo(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_jbo
                    }),
                    ("ff", {
                        fn rule_ff(po: PluralOperands) -> PluralCategory {
                            if (po.i == 0 || po.i == 1) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ff
                    }),
                    ("iu", {
                        fn rule_iu(po: PluralOperands) -> PluralCategory {
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
                    ("sr", {
                        fn rule_sr(po: PluralOperands) -> PluralCategory {
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
                    ("br", {
                        fn rule_br(po: PluralOperands) -> PluralCategory {
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
                    ("th", {
                        fn rule_th(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_th
                    }),
                    ("smi", {
                        fn rule_smi(po: PluralOperands) -> PluralCategory {
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
                    ("ks", {
                        fn rule_ks(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ks
                    }),
                    ("sq", {
                        fn rule_sq(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_sq
                    }),
                    ("af", {
                        fn rule_af(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_af
                    }),
                    ("yi", {
                        fn rule_yi(po: PluralOperands) -> PluralCategory {
                            if (po.i == 1 && po.v == 0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_yi
                    }),
                    ("prg", {
                        fn rule_prg(po: PluralOperands) -> PluralCategory {
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
                    ("vi", {
                        fn rule_vi(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_vi
                    }),
                    ("wo", {
                        fn rule_wo(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_wo
                    }),
                    ("sd", {
                        fn rule_sd(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_sd
                    }),
                    ("xog", {
                        fn rule_xog(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_xog
                    }),
                    ("naq", {
                        fn rule_naq(po: PluralOperands) -> PluralCategory {
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
                    ("root", {
                        fn rule_root(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_root
                    }),
                    ("gl", {
                        fn rule_gl(po: PluralOperands) -> PluralCategory {
                            if (po.i == 1 && po.v == 0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_gl
                    }),
                    ("fil", {
                        fn rule_fil(po: PluralOperands) -> PluralCategory {
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
                    ("id", {
                        fn rule_id(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_id
                    }),
                    ("el", {
                        fn rule_el(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_el
                    }),
                    ("zh", {
                        fn rule_zh(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_zh
                    }),
                    ("az", {
                        fn rule_az(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_az
                    }),
                    ("tl", {
                        fn rule_tl(po: PluralOperands) -> PluralCategory {
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
                    ("to", {
                        fn rule_to(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_to
                    }),
                    ("lv", {
                        fn rule_lv(po: PluralOperands) -> PluralCategory {
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
                    ("vun", {
                        fn rule_vun(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_vun
                    }),
                    ("ak", {
                        fn rule_ak(po: PluralOperands) -> PluralCategory {
                            if (matches!(po.i, 0..=1) && po.f == 0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ak
                    }),
                    ("tn", {
                        fn rule_tn(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_tn
                    }),
                    ("fo", {
                        fn rule_fo(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_fo
                    }),
                    ("pt", {
                        fn rule_pt(po: PluralOperands) -> PluralCategory {
                            if (matches!(po.i, 0..=1)) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_pt
                    }),
                    ("cgg", {
                        fn rule_cgg(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_cgg
                    }),
                    ("si", {
                        fn rule_si(po: PluralOperands) -> PluralCategory {
                            if (po.n == 0.0 || po.n == 1.0) || (po.i == 0 && po.f == 1) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_si
                    }),
                    ("tzm", {
                        fn rule_tzm(po: PluralOperands) -> PluralCategory {
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
                    ("sv", {
                        fn rule_sv(po: PluralOperands) -> PluralCategory {
                            if (po.i == 1 && po.v == 0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_sv
                    }),
                    ("mas", {
                        fn rule_mas(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_mas
                    }),
                    ("mn", {
                        fn rule_mn(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_mn
                    }),
                    ("xh", {
                        fn rule_xh(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_xh
                    }),
                    ("asa", {
                        fn rule_asa(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_asa
                    }),
                    ("ro", {
                        fn rule_ro(po: PluralOperands) -> PluralCategory {
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
                    ("kl", {
                        fn rule_kl(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_kl
                    }),
                    ("smn", {
                        fn rule_smn(po: PluralOperands) -> PluralCategory {
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
                    ("se", {
                        fn rule_se(po: PluralOperands) -> PluralCategory {
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
                    ("st", {
                        fn rule_st(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_st
                    }),
                    ("no", {
                        fn rule_no(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_no
                    }),
                    ("fa", {
                        fn rule_fa(po: PluralOperands) -> PluralCategory {
                            if (po.i == 0) || (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_fa
                    }),
                    ("gu", {
                        fn rule_gu(po: PluralOperands) -> PluralCategory {
                            if (po.i == 0) || (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_gu
                    }),
                    ("mgo", {
                        fn rule_mgo(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_mgo
                    }),
                    ("io", {
                        fn rule_io(po: PluralOperands) -> PluralCategory {
                            if (po.i == 1 && po.v == 0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_io
                    }),
                    ("saq", {
                        fn rule_saq(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_saq
                    }),
                    ("ku", {
                        fn rule_ku(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ku
                    }),
                    ("ta", {
                        fn rule_ta(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ta
                    }),
                    ("am", {
                        fn rule_am(po: PluralOperands) -> PluralCategory {
                            if (po.i == 0) || (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_am
                    }),
                    ("lb", {
                        fn rule_lb(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_lb
                    }),
                    ("ars", {
                        fn rule_ars(po: PluralOperands) -> PluralCategory {
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
                    ("sh", {
                        fn rule_sh(po: PluralOperands) -> PluralCategory {
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
                    ("mt", {
                        fn rule_mt(po: PluralOperands) -> PluralCategory {
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
                    ("ne", {
                        fn rule_ne(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ne
                    }),
                    ("ts", {
                        fn rule_ts(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ts
                    }),
                    ("bh", {
                        fn rule_bh(po: PluralOperands) -> PluralCategory {
                            if (matches!(po.i, 0..=1) && po.f == 0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_bh
                    }),
                    ("bem", {
                        fn rule_bem(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_bem
                    }),
                    ("ml", {
                        fn rule_ml(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ml
                    }),
                    ("ti", {
                        fn rule_ti(po: PluralOperands) -> PluralCategory {
                            if (matches!(po.i, 0..=1) && po.f == 0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ti
                    }),
                    ("as", {
                        fn rule_as(po: PluralOperands) -> PluralCategory {
                            if (po.i == 0) || (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_as
                    }),
                    ("en", {
                        fn rule_en(po: PluralOperands) -> PluralCategory {
                            if (po.i == 1 && po.v == 0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_en
                    }),
                    ("lkt", {
                        fn rule_lkt(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_lkt
                    }),
                    ("in", {
                        fn rule_in(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_in
                    }),
                    ("or", {
                        fn rule_or(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_or
                    }),
                    ("mr", {
                        fn rule_mr(po: PluralOperands) -> PluralCategory {
                            if (po.i == 0) || (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_mr
                    }),
                    ("chr", {
                        fn rule_chr(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_chr
                    }),
                    ("bm", {
                        fn rule_bm(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_bm
                    }),
                    ("ln", {
                        fn rule_ln(po: PluralOperands) -> PluralCategory {
                            if (matches!(po.i, 0..=1) && po.f == 0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ln
                    }),
                    ("nah", {
                        fn rule_nah(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_nah
                    }),
                    ("nr", {
                        fn rule_nr(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_nr
                    }),
                    ("nl", {
                        fn rule_nl(po: PluralOperands) -> PluralCategory {
                            if (po.i == 1 && po.v == 0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_nl
                    }),
                    ("brx", {
                        fn rule_brx(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_brx
                    }),
                    ("uk", {
                        fn rule_uk(po: PluralOperands) -> PluralCategory {
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
                    ("nyn", {
                        fn rule_nyn(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_nyn
                    }),
                    ("dz", {
                        fn rule_dz(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_dz
                    }),
                    ("ar", {
                        fn rule_ar(po: PluralOperands) -> PluralCategory {
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
                    ("ps", {
                        fn rule_ps(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ps
                    }),
                    ("jgo", {
                        fn rule_jgo(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_jgo
                    }),
                    ("sl", {
                        fn rule_sl(po: PluralOperands) -> PluralCategory {
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
                    ("pt-PT", {
                        fn rule_pt_pt(po: PluralOperands) -> PluralCategory {
                            if (po.i == 1 && po.v == 0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_pt_pt
                    }),
                    ("gd", {
                        fn rule_gd(po: PluralOperands) -> PluralCategory {
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
                    ("de", {
                        fn rule_de(po: PluralOperands) -> PluralCategory {
                            if (po.i == 1 && po.v == 0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_de
                    }),
                    ("ko", {
                        fn rule_ko(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ko
                    }),
                    ("jv", {
                        fn rule_jv(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_jv
                    }),
                    ("kk", {
                        fn rule_kk(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_kk
                    }),
                    ("scn", {
                        fn rule_scn(po: PluralOperands) -> PluralCategory {
                            if (po.i == 1 && po.v == 0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_scn
                    }),
                    ("cy", {
                        fn rule_cy(po: PluralOperands) -> PluralCategory {
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
                    ("ur", {
                        fn rule_ur(po: PluralOperands) -> PluralCategory {
                            if (po.i == 1 && po.v == 0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ur
                    }),
                    ("nqo", {
                        fn rule_nqo(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_nqo
                    }),
                    ("tk", {
                        fn rule_tk(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_tk
                    }),
                    ("nnh", {
                        fn rule_nnh(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_nnh
                    }),
                    ("hr", {
                        fn rule_hr(po: PluralOperands) -> PluralCategory {
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
                    ("it", {
                        fn rule_it(po: PluralOperands) -> PluralCategory {
                            if (po.i == 1 && po.v == 0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_it
                    }),
                    ("sms", {
                        fn rule_sms(po: PluralOperands) -> PluralCategory {
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
                    ("nd", {
                        fn rule_nd(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_nd
                    }),
                    ("dv", {
                        fn rule_dv(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_dv
                    }),
                    ("bez", {
                        fn rule_bez(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_bez
                    }),
                    ("hi", {
                        fn rule_hi(po: PluralOperands) -> PluralCategory {
                            if (po.i == 0) || (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_hi
                    }),
                    ("ca", {
                        fn rule_ca(po: PluralOperands) -> PluralCategory {
                            if (po.i == 1 && po.v == 0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ca
                    }),
                    ("guw", {
                        fn rule_guw(po: PluralOperands) -> PluralCategory {
                            if (matches!(po.i, 0..=1) && po.f == 0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_guw
                    }),
                    ("kkj", {
                        fn rule_kkj(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_kkj
                    }),
                    ("ast", {
                        fn rule_ast(po: PluralOperands) -> PluralCategory {
                            if (po.i == 1 && po.v == 0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ast
                    }),
                    ("sk", {
                        fn rule_sk(po: PluralOperands) -> PluralCategory {
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
                    ("sma", {
                        fn rule_sma(po: PluralOperands) -> PluralCategory {
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
                    ("te", {
                        fn rule_te(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_te
                    }),
                    ("gv", {
                        fn rule_gv(po: PluralOperands) -> PluralCategory {
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
                    ("lg", {
                        fn rule_lg(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_lg
                    }),
                    ("nn", {
                        fn rule_nn(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_nn
                    }),
                    ("kcg", {
                        fn rule_kcg(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_kcg
                    }),
                    ("rwk", {
                        fn rule_rwk(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_rwk
                    }),
                    ("lt", {
                        fn rule_lt(po: PluralOperands) -> PluralCategory {
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
                    ("hy", {
                        fn rule_hy(po: PluralOperands) -> PluralCategory {
                            if (po.i == 0 || po.i == 1) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_hy
                    }),
                    ("ga", {
                        fn rule_ga(po: PluralOperands) -> PluralCategory {
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
                    ("nso", {
                        fn rule_nso(po: PluralOperands) -> PluralCategory {
                            if (matches!(po.i, 0..=1) && po.f == 0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_nso
                    }),
                    ("kea", {
                        fn rule_kea(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_kea
                    }),
                    ("ji", {
                        fn rule_ji(po: PluralOperands) -> PluralCategory {
                            if (po.i == 1 && po.v == 0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ji
                    }),
                    ("os", {
                        fn rule_os(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_os
                    }),
                    ("ii", {
                        fn rule_ii(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ii
                    }),
                    ("kde", {
                        fn rule_kde(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_kde
                    }),
                    ("lo", {
                        fn rule_lo(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_lo
                    }),
                    ("hsb", {
                        fn rule_hsb(po: PluralOperands) -> PluralCategory {
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
                    ("bn", {
                        fn rule_bn(po: PluralOperands) -> PluralCategory {
                            if (po.i == 0) || (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_bn
                    }),
                    ("sw", {
                        fn rule_sw(po: PluralOperands) -> PluralCategory {
                            if (po.i == 1 && po.v == 0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_sw
                    }),
                    ("ss", {
                        fn rule_ss(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ss
                    }),
                    ("bo", {
                        fn rule_bo(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_bo
                    }),
                    ("eo", {
                        fn rule_eo(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_eo
                    }),
                    ("uz", {
                        fn rule_uz(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_uz
                    }),
                    ("fi", {
                        fn rule_fi(po: PluralOperands) -> PluralCategory {
                            if (po.i == 1 && po.v == 0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_fi
                    }),
                    ("sah", {
                        fn rule_sah(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_sah
                    }),
                    ("mk", {
                        fn rule_mk(po: PluralOperands) -> PluralCategory {
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
                    ("so", {
                        fn rule_so(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_so
                    }),
                    ("tig", {
                        fn rule_tig(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_tig
                    }),
                    ("dsb", {
                        fn rule_dsb(po: PluralOperands) -> PluralCategory {
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
                    ("fr", {
                        fn rule_fr(po: PluralOperands) -> PluralCategory {
                            if (po.i == 0 || po.i == 1) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_fr
                    }),
                    ("eu", {
                        fn rule_eu(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_eu
                    }),
                    ("teo", {
                        fn rule_teo(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_teo
                    }),
                    ("vo", {
                        fn rule_vo(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_vo
                    }),
                ]),
            };
            LANGUAGES.get(lang_code).cloned().ok_or(())
        }
        PluralRuleType::ORDINAL => {
            static LANGUAGES: phf::Map<&'static str, PluralRule> = ::phf::Map {
                key: 9603444721912725599,
                disps: ::phf::Slice::Static(&[
                    (0, 1),
                    (1, 24),
                    (7, 2),
                    (2, 13),
                    (0, 0),
                    (0, 19),
                    (1, 8),
                    (0, 2),
                    (0, 52),
                    (0, 6),
                    (1, 0),
                    (4, 57),
                    (0, 14),
                    (35, 51),
                    (0, 91),
                    (1, 1),
                    (0, 32),
                    (0, 59),
                    (33, 82),
                ]),
                entries: ::phf::Slice::Static(&[
                    ("ca", {
                        fn rule_ca(po: PluralOperands) -> PluralCategory {
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
                    ("lo", {
                        fn rule_lo(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_lo
                    }),
                    ("sq", {
                        fn rule_sq(po: PluralOperands) -> PluralCategory {
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
                    ("fr", {
                        fn rule_fr(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_fr
                    }),
                    ("fil", {
                        fn rule_fil(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_fil
                    }),
                    ("es", {
                        fn rule_es(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_es
                    }),
                    ("ko", {
                        fn rule_ko(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ko
                    }),
                    ("is", {
                        fn rule_is(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_is
                    }),
                    ("nb", {
                        fn rule_nb(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_nb
                    }),
                    ("ja", {
                        fn rule_ja(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ja
                    }),
                    ("th", {
                        fn rule_th(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_th
                    }),
                    ("pt", {
                        fn rule_pt(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_pt
                    }),
                    ("da", {
                        fn rule_da(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_da
                    }),
                    ("hi", {
                        fn rule_hi(po: PluralOperands) -> PluralCategory {
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
                    ("kk", {
                        fn rule_kk(po: PluralOperands) -> PluralCategory {
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
                    ("hu", {
                        fn rule_hu(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0 || po.n == 5.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_hu
                    }),
                    ("gu", {
                        fn rule_gu(po: PluralOperands) -> PluralCategory {
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
                    ("ce", {
                        fn rule_ce(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ce
                    }),
                    ("eu", {
                        fn rule_eu(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_eu
                    }),
                    ("sh", {
                        fn rule_sh(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_sh
                    }),
                    ("el", {
                        fn rule_el(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_el
                    }),
                    ("ar", {
                        fn rule_ar(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ar
                    }),
                    ("as", {
                        fn rule_as(po: PluralOperands) -> PluralCategory {
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
                    ("af", {
                        fn rule_af(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_af
                    }),
                    ("fa", {
                        fn rule_fa(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_fa
                    }),
                    ("sw", {
                        fn rule_sw(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_sw
                    }),
                    ("km", {
                        fn rule_km(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_km
                    }),
                    ("or", {
                        fn rule_or(po: PluralOperands) -> PluralCategory {
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
                    ("sr", {
                        fn rule_sr(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_sr
                    }),
                    ("uz", {
                        fn rule_uz(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_uz
                    }),
                    ("bn", {
                        fn rule_bn(po: PluralOperands) -> PluralCategory {
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
                    ("az", {
                        fn rule_az(po: PluralOperands) -> PluralCategory {
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
                    ("ky", {
                        fn rule_ky(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ky
                    }),
                    ("vi", {
                        fn rule_vi(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_vi
                    }),
                    ("it", {
                        fn rule_it(po: PluralOperands) -> PluralCategory {
                            if (po.n == 11.0 || po.n == 8.0 || po.n == 80.0 || po.n == 800.0) {
                                PluralCategory::MANY
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_it
                    }),
                    ("he", {
                        fn rule_he(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_he
                    }),
                    ("gl", {
                        fn rule_gl(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_gl
                    }),
                    ("prg", {
                        fn rule_prg(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_prg
                    }),
                    ("iw", {
                        fn rule_iw(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_iw
                    }),
                    ("am", {
                        fn rule_am(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_am
                    }),
                    ("ta", {
                        fn rule_ta(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ta
                    }),
                    ("dsb", {
                        fn rule_dsb(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_dsb
                    }),
                    ("ru", {
                        fn rule_ru(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ru
                    }),
                    ("ne", {
                        fn rule_ne(po: PluralOperands) -> PluralCategory {
                            if (matches!(po.i, 1..=4) && po.f == 0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ne
                    }),
                    ("pl", {
                        fn rule_pl(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_pl
                    }),
                    ("zu", {
                        fn rule_zu(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_zu
                    }),
                    ("ms", {
                        fn rule_ms(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ms
                    }),
                    ("tr", {
                        fn rule_tr(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_tr
                    }),
                    ("cy", {
                        fn rule_cy(po: PluralOperands) -> PluralCategory {
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
                    ("ro", {
                        fn rule_ro(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ro
                    }),
                    ("sd", {
                        fn rule_sd(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_sd
                    }),
                    ("mo", {
                        fn rule_mo(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_mo
                    }),
                    ("zh", {
                        fn rule_zh(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_zh
                    }),
                    ("tk", {
                        fn rule_tk(po: PluralOperands) -> PluralCategory {
                            if (po.i % 10 == 6 || po.i % 10 == 9) || (po.n == 10.0) {
                                PluralCategory::FEW
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_tk
                    }),
                    ("bs", {
                        fn rule_bs(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_bs
                    }),
                    ("ps", {
                        fn rule_ps(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ps
                    }),
                    ("id", {
                        fn rule_id(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_id
                    }),
                    ("et", {
                        fn rule_et(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_et
                    }),
                    ("hr", {
                        fn rule_hr(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_hr
                    }),
                    ("my", {
                        fn rule_my(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_my
                    }),
                    ("si", {
                        fn rule_si(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_si
                    }),
                    ("ka", {
                        fn rule_ka(po: PluralOperands) -> PluralCategory {
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
                    ("ml", {
                        fn rule_ml(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ml
                    }),
                    ("yue", {
                        fn rule_yue(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_yue
                    }),
                    ("lv", {
                        fn rule_lv(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_lv
                    }),
                    ("te", {
                        fn rule_te(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_te
                    }),
                    ("tl", {
                        fn rule_tl(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_tl
                    }),
                    ("nl", {
                        fn rule_nl(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_nl
                    }),
                    ("sl", {
                        fn rule_sl(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_sl
                    }),
                    ("scn", {
                        fn rule_scn(po: PluralOperands) -> PluralCategory {
                            if (po.n == 11.0 || po.n == 8.0 || po.n == 80.0 || po.n == 800.0) {
                                PluralCategory::MANY
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_scn
                    }),
                    ("be", {
                        fn rule_be(po: PluralOperands) -> PluralCategory {
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
                    ("hy", {
                        fn rule_hy(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_hy
                    }),
                    ("hsb", {
                        fn rule_hsb(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_hsb
                    }),
                    ("fy", {
                        fn rule_fy(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_fy
                    }),
                    ("kn", {
                        fn rule_kn(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_kn
                    }),
                    ("sk", {
                        fn rule_sk(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_sk
                    }),
                    ("mk", {
                        fn rule_mk(po: PluralOperands) -> PluralCategory {
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
                    ("sv", {
                        fn rule_sv(po: PluralOperands) -> PluralCategory {
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
                    ("en", {
                        fn rule_en(po: PluralOperands) -> PluralCategory {
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
                    ("de", {
                        fn rule_de(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_de
                    }),
                    ("cs", {
                        fn rule_cs(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_cs
                    }),
                    ("ga", {
                        fn rule_ga(po: PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ga
                    }),
                    ("in", {
                        fn rule_in(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_in
                    }),
                    ("mn", {
                        fn rule_mn(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_mn
                    }),
                    ("uk", {
                        fn rule_uk(po: PluralOperands) -> PluralCategory {
                            if (po.i % 10 == 3 && po.i % 100 != 13) {
                                PluralCategory::FEW
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_uk
                    }),
                    ("bg", {
                        fn rule_bg(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_bg
                    }),
                    ("lt", {
                        fn rule_lt(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_lt
                    }),
                    ("mr", {
                        fn rule_mr(po: PluralOperands) -> PluralCategory {
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
                    ("ur", {
                        fn rule_ur(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ur
                    }),
                    ("gsw", {
                        fn rule_gsw(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_gsw
                    }),
                    ("pa", {
                        fn rule_pa(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_pa
                    }),
                    ("root", {
                        fn rule_root(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_root
                    }),
                    ("fi", {
                        fn rule_fi(po: PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_fi
                    }),
                ]),
            };
            LANGUAGES.get(lang_code).cloned().ok_or(())
        }
    }
}
