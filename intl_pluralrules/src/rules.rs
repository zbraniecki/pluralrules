#![allow(unused_variables, unused_parens)]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::float_cmp))]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::unreadable_literal))]
#![cfg_attr(feature = "cargo-clippy", allow(clippy::nonminimal_bool))]
use super::operands::PluralOperands;
use super::{PluralCategory, PluralRuleType};
use matches::matches;
use phf;
pub type PluralRule = fn(&PluralOperands) -> PluralCategory;
pub static CLDR_VERSION: usize = 36;
#[cfg_attr(tarpaulin, skip)]
pub fn get_locales(pr_type: PluralRuleType) -> &'static [&'static str] {
    match pr_type {
        PluralRuleType::CARDINAL => &[
            "af", "ak", "am", "an", "ar", "ars", "as", "asa", "ast", "az", "be", "bem", "bez",
            "bg", "bho", "bm", "bn", "bo", "br", "brx", "bs", "ca", "ce", "ceb", "cgg", "chr",
            "ckb", "cs", "cy", "da", "de", "dsb", "dv", "dz", "ee", "el", "en", "eo", "es", "et",
            "eu", "fa", "ff", "fi", "fil", "fo", "fr", "fur", "fy", "ga", "gd", "gl", "gsw", "gu",
            "guw", "gv", "ha", "haw", "he", "hi", "hr", "hsb", "hu", "hy", "ia", "id", "ig", "ii",
            "in", "io", "is", "it", "iu", "iw", "ja", "jbo", "jgo", "ji", "jmc", "jv", "jw", "ka",
            "kab", "kaj", "kcg", "kde", "kea", "kk", "kkj", "kl", "km", "kn", "ko", "ks", "ksb",
            "ksh", "ku", "kw", "ky", "lag", "lb", "lg", "lkt", "ln", "lo", "lt", "lv", "mas", "mg",
            "mgo", "mk", "ml", "mn", "mo", "mr", "ms", "mt", "my", "nah", "naq", "nb", "nd", "ne",
            "nl", "nn", "nnh", "no", "nqo", "nr", "nso", "ny", "nyn", "om", "or", "os", "osa",
            "pa", "pap", "pl", "prg", "ps", "pt", "pt-PT", "rm", "ro", "rof", "root", "ru", "rwk",
            "sah", "saq", "sc", "scn", "sd", "sdh", "se", "seh", "ses", "sg", "sh", "shi", "si",
            "sk", "sl", "sma", "smi", "smj", "smn", "sms", "sn", "so", "sq", "sr", "ss", "ssy",
            "st", "su", "sv", "sw", "syr", "ta", "te", "teo", "th", "ti", "tig", "tk", "tl", "tn",
            "to", "tr", "ts", "tzm", "ug", "uk", "ur", "uz", "ve", "vi", "vo", "vun", "wa", "wae",
            "wo", "xh", "xog", "yi", "yo", "yue", "zh", "zu",
        ],
        PluralRuleType::ORDINAL => &[
            "af", "am", "an", "ar", "as", "az", "be", "bg", "bn", "bs", "ca", "ce", "cs", "cy",
            "da", "de", "dsb", "el", "en", "es", "et", "eu", "fa", "fi", "fil", "fr", "fy", "ga",
            "gd", "gl", "gsw", "gu", "he", "hi", "hr", "hsb", "hu", "hy", "ia", "id", "in", "is",
            "it", "iw", "ja", "ka", "kk", "km", "kn", "ko", "kw", "ky", "lo", "lt", "lv", "mk",
            "ml", "mn", "mo", "mr", "ms", "my", "nb", "ne", "nl", "or", "pa", "pl", "prg", "ps",
            "pt", "ro", "root", "ru", "sc", "scn", "sd", "sh", "si", "sk", "sl", "sq", "sr", "sv",
            "sw", "ta", "te", "th", "tk", "tl", "tr", "uk", "ur", "uz", "vi", "yue", "zh", "zu",
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
                    (0, 52),
                    (3, 77),
                    (0, 21),
                    (0, 20),
                    (1, 85),
                    (3, 6),
                    (2, 60),
                    (0, 135),
                    (0, 106),
                    (1, 87),
                    (3, 50),
                    (2, 67),
                    (0, 22),
                    (1, 66),
                    (0, 3),
                    (0, 44),
                    (1, 134),
                    (1, 25),
                    (0, 142),
                    (1, 49),
                    (2, 109),
                    (2, 84),
                    (0, 12),
                    (4, 6),
                    (0, 3),
                    (1, 177),
                    (0, 0),
                    (1, 179),
                    (0, 2),
                    (0, 1),
                    (7, 182),
                    (0, 3),
                    (0, 9),
                    (1, 139),
                    (1, 45),
                    (0, 1),
                    (1, 42),
                    (0, 118),
                    (9, 166),
                    (0, 207),
                    (0, 1),
                    (0, 1),
                    (34, 91),
                ]),
                entries: ::phf::Slice::Static(&[
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
                    ("ii", {
                        fn rule_ii(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ii
                    }),
                    ("bo", {
                        fn rule_bo(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_bo
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
                    ("yue", {
                        fn rule_yue(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_yue
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
                    ("my", {
                        fn rule_my(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_my
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
                    ("kea", {
                        fn rule_kea(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_kea
                    }),
                    ("in", {
                        fn rule_in(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_in
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
                    ("ms", {
                        fn rule_ms(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ms
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
                    ("mr", {
                        fn rule_mr(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_mr
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
                    ("dz", {
                        fn rule_dz(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_dz
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
                    ("km", {
                        fn rule_km(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_km
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
                    ("th", {
                        fn rule_th(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_th
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
                    ("id", {
                        fn rule_id(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_id
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
                    ("zh", {
                        fn rule_zh(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_zh
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
                    ("ses", {
                        fn rule_ses(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ses
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
                    ("ceb", {
                        fn rule_ceb(po: &PluralOperands) -> PluralCategory {
                            if (po.v == 0 && (po.i == 1 || po.i == 2 || po.i == 3))
                                || (po.v == 0 && po.i % 10 != 4 && po.i % 10 != 6 && po.i % 10 != 9)
                                || (po.v != 0 && po.f % 10 != 4 && po.f % 10 != 6 && po.f % 10 != 9)
                            {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ceb
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
                    ("osa", {
                        fn rule_osa(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_osa
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
                    ("kde", {
                        fn rule_kde(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_kde
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
                    ("yo", {
                        fn rule_yo(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_yo
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
                    ("wo", {
                        fn rule_wo(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_wo
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
                    ("ja", {
                        fn rule_ja(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ja
                    }),
                    ("ig", {
                        fn rule_ig(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ig
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
                    ("an", {
                        fn rule_an(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_an
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
                    ("kw", {
                        fn rule_kw(po: &PluralOperands) -> PluralCategory {
                            if (po.i % 100 == 3
                                || po.i % 100 == 23
                                || po.i % 100 == 43
                                || po.i % 100 == 63
                                || po.i % 100 == 83)
                            {
                                PluralCategory::FEW
                            } else if (po.n != 1.0
                                && (po.i % 100 == 1
                                    || po.i % 100 == 21
                                    || po.i % 100 == 41
                                    || po.i % 100 == 61
                                    || po.i % 100 == 81))
                            {
                                PluralCategory::MANY
                            } else if (po.n == 1.0) {
                                PluralCategory::ONE
                            } else if (po.i % 100 == 2
                                || po.i % 100 == 22
                                || po.i % 100 == 42
                                || po.i % 100 == 62
                                || po.i % 100 == 82)
                                || (po.i % 1000 == 0
                                    && (po.i % 100000 == 40000
                                        || po.i % 100000 == 60000
                                        || po.i % 100000 == 80000
                                        || matches!(po.i, 1000..=20000)))
                                || (po.n != 0.0 && po.i % 1000000 == 100000)
                            {
                                PluralCategory::TWO
                            } else if (po.n == 0.0) {
                                PluralCategory::ZERO
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_kw
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
                    ("jv", {
                        fn rule_jv(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_jv
                    }),
                    ("ro", {
                        fn rule_ro(po: &PluralOperands) -> PluralCategory {
                            if (po.v != 0) || (po.n == 0.0) || (matches!(po.i, 2..=19)) {
                                PluralCategory::FEW
                            } else if (po.i == 1 && po.v == 0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ro
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
                    ("bho", {
                        fn rule_bho(po: &PluralOperands) -> PluralCategory {
                            if (matches!(po.i, 0..=1) && po.f == 0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_bho
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
                    ("mo", {
                        fn rule_mo(po: &PluralOperands) -> PluralCategory {
                            if (po.v != 0) || (po.n == 0.0) || (matches!(po.i, 2..=19)) {
                                PluralCategory::FEW
                            } else if (po.i == 1 && po.v == 0) {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_mo
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
                    ("su", {
                        fn rule_su(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_su
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
                    ("vi", {
                        fn rule_vi(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_vi
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
                    ("to", {
                        fn rule_to(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_to
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
                    ("root", {
                        fn rule_root(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_root
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
                    ("jbo", {
                        fn rule_jbo(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_jbo
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
                    ("lkt", {
                        fn rule_lkt(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_lkt
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
                    ("nqo", {
                        fn rule_nqo(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_nqo
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
                    ("sg", {
                        fn rule_sg(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_sg
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
                    ("ko", {
                        fn rule_ko(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ko
                    }),
                    ("lo", {
                        fn rule_lo(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_lo
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
                    ("sah", {
                        fn rule_sah(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_sah
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
                ]),
            };
            LANGUAGES.get(lang_code).cloned().ok_or(())
        }
        PluralRuleType::ORDINAL => {
            static LANGUAGES: phf::Map<&'static str, PluralRule> = ::phf::Map {
                key: 732231254413039614,
                disps: ::phf::Slice::Static(&[
                    (0, 1),
                    (0, 0),
                    (0, 3),
                    (0, 12),
                    (0, 7),
                    (0, 1),
                    (5, 54),
                    (0, 91),
                    (8, 41),
                    (1, 1),
                    (1, 43),
                    (1, 14),
                    (7, 52),
                    (58, 82),
                    (9, 82),
                    (0, 67),
                    (1, 0),
                    (2, 24),
                    (3, 65),
                    (1, 18),
                ]),
                entries: ::phf::Slice::Static(&[
                    ("sw", {
                        fn rule_sw(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_sw
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
                    ("bg", {
                        fn rule_bg(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_bg
                    }),
                    ("hr", {
                        fn rule_hr(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_hr
                    }),
                    ("fi", {
                        fn rule_fi(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_fi
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
                    ("ta", {
                        fn rule_ta(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ta
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
                    ("fa", {
                        fn rule_fa(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_fa
                    }),
                    ("mn", {
                        fn rule_mn(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_mn
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
                    ("ko", {
                        fn rule_ko(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ko
                    }),
                    ("ps", {
                        fn rule_ps(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ps
                    }),
                    ("hsb", {
                        fn rule_hsb(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_hsb
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
                    ("ur", {
                        fn rule_ur(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ur
                    }),
                    ("fy", {
                        fn rule_fy(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_fy
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
                    ("de", {
                        fn rule_de(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_de
                    }),
                    ("ia", {
                        fn rule_ia(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ia
                    }),
                    ("prg", {
                        fn rule_prg(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_prg
                    }),
                    ("km", {
                        fn rule_km(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_km
                    }),
                    ("pl", {
                        fn rule_pl(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_pl
                    }),
                    ("ml", {
                        fn rule_ml(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ml
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
                    ("te", {
                        fn rule_te(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_te
                    }),
                    ("sk", {
                        fn rule_sk(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_sk
                    }),
                    ("root", {
                        fn rule_root(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_root
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
                    ("el", {
                        fn rule_el(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_el
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
                    ("sh", {
                        fn rule_sh(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_sh
                    }),
                    ("lv", {
                        fn rule_lv(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_lv
                    }),
                    ("cs", {
                        fn rule_cs(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_cs
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
                    ("yue", {
                        fn rule_yue(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_yue
                    }),
                    ("et", {
                        fn rule_et(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_et
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
                    ("lt", {
                        fn rule_lt(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_lt
                    }),
                    ("af", {
                        fn rule_af(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_af
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
                    ("eu", {
                        fn rule_eu(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_eu
                    }),
                    ("ru", {
                        fn rule_ru(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ru
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
                    ("am", {
                        fn rule_am(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_am
                    }),
                    ("pa", {
                        fn rule_pa(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_pa
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
                    ("tr", {
                        fn rule_tr(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_tr
                    }),
                    ("ja", {
                        fn rule_ja(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ja
                    }),
                    ("es", {
                        fn rule_es(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_es
                    }),
                    ("pt", {
                        fn rule_pt(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_pt
                    }),
                    ("sl", {
                        fn rule_sl(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_sl
                    }),
                    ("in", {
                        fn rule_in(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_in
                    }),
                    ("uz", {
                        fn rule_uz(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_uz
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
                    ("sd", {
                        fn rule_sd(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_sd
                    }),
                    ("nb", {
                        fn rule_nb(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_nb
                    }),
                    ("he", {
                        fn rule_he(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_he
                    }),
                    ("iw", {
                        fn rule_iw(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_iw
                    }),
                    ("dsb", {
                        fn rule_dsb(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_dsb
                    }),
                    ("ce", {
                        fn rule_ce(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ce
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
                    ("ar", {
                        fn rule_ar(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ar
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
                    ("is", {
                        fn rule_is(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_is
                    }),
                    ("gl", {
                        fn rule_gl(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_gl
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
                    ("th", {
                        fn rule_th(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_th
                    }),
                    ("si", {
                        fn rule_si(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_si
                    }),
                    ("zu", {
                        fn rule_zu(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_zu
                    }),
                    ("id", {
                        fn rule_id(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_id
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
                    ("gsw", {
                        fn rule_gsw(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_gsw
                    }),
                    ("bs", {
                        fn rule_bs(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_bs
                    }),
                    ("kw", {
                        fn rule_kw(po: &PluralOperands) -> PluralCategory {
                            if (po.n == 5.0) || (po.i % 100 == 5) {
                                PluralCategory::MANY
                            } else if (matches!(po.i, 1..=4) && po.f == 0)
                                || (matches!(po.i, 1..=4)
                                    || matches!(po.i, 21..=24)
                                    || matches!(po.i, 41..=44)
                                    || matches!(po.i, 61..=64)
                                    || matches!(po.i, 81..=84))
                            {
                                PluralCategory::ONE
                            } else {
                                PluralCategory::OTHER
                            }
                        };
                        rule_kw
                    }),
                    ("da", {
                        fn rule_da(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_da
                    }),
                    ("an", {
                        fn rule_an(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_an
                    }),
                    ("nl", {
                        fn rule_nl(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_nl
                    }),
                ]),
            };
            LANGUAGES.get(lang_code).cloned().ok_or(())
        }
    }
}
