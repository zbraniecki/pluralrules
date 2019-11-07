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
                    (0, 5),
                    (0, 139),
                    (0, 18),
                    (0, 133),
                    (1, 15),
                    (0, 4),
                    (0, 0),
                    (0, 10),
                    (0, 0),
                    (0, 191),
                    (2, 167),
                    (0, 0),
                    (0, 189),
                    (0, 1),
                    (3, 69),
                    (0, 116),
                    (0, 20),
                    (0, 20),
                    (4, 6),
                    (0, 206),
                    (1, 3),
                    (0, 48),
                    (9, 180),
                    (10, 142),
                    (5, 195),
                    (0, 50),
                    (0, 24),
                    (1, 76),
                    (1, 3),
                    (19, 83),
                    (0, 28),
                    (2, 105),
                    (9, 201),
                    (0, 15),
                    (0, 110),
                    (6, 9),
                    (0, 136),
                    (0, 107),
                    (42, 10),
                    (0, 20),
                    (0, 133),
                    (0, 7),
                    (0, 1),
                ]),
                entries: ::phf::Slice::Static(&[
                    ("dz", {
                        fn rule_dz(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_dz
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
                    ("osa", {
                        fn rule_osa(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_osa
                    }),
                    ("lo", {
                        fn rule_lo(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_lo
                    }),
                    ("wo", {
                        fn rule_wo(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_wo
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
                    ("yue", {
                        fn rule_yue(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_yue
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
                    ("id", {
                        fn rule_id(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_id
                    }),
                    ("my", {
                        fn rule_my(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_my
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
                    ("lkt", {
                        fn rule_lkt(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_lkt
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
                    ("jv", {
                        fn rule_jv(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_jv
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
                    ("nqo", {
                        fn rule_nqo(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_nqo
                    }),
                    ("in", {
                        fn rule_in(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_in
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
                    ("su", {
                        fn rule_su(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_su
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
                    ("sah", {
                        fn rule_sah(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_sah
                    }),
                    ("ko", {
                        fn rule_ko(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ko
                    }),
                    ("bm", {
                        fn rule_bm(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_bm
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
                    ("jw", {
                        fn rule_jw(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_jw
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
                    ("jbo", {
                        fn rule_jbo(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_jbo
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
                    ("ig", {
                        fn rule_ig(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ig
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
                    ("zh", {
                        fn rule_zh(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_zh
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
                    ("bo", {
                        fn rule_bo(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_bo
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
                    ("to", {
                        fn rule_to(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_to
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
                    ("km", {
                        fn rule_km(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_km
                    }),
                    ("ses", {
                        fn rule_ses(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ses
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
                    ("kea", {
                        fn rule_kea(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_kea
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
                    ("yo", {
                        fn rule_yo(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_yo
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
                    ("root", {
                        fn rule_root(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_root
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
                    ("ms", {
                        fn rule_ms(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ms
                    }),
                    ("vi", {
                        fn rule_vi(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_vi
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
                    ("kde", {
                        fn rule_kde(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_kde
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
                    ("th", {
                        fn rule_th(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_th
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
                    ("ii", {
                        fn rule_ii(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ii
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
                    ("sg", {
                        fn rule_sg(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_sg
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
                    ("ja", {
                        fn rule_ja(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ja
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
                ]),
            };
            LANGUAGES.get(lang_code).cloned().ok_or(())
        }
        PluralRuleType::ORDINAL => {
            static LANGUAGES: phf::Map<&'static str, PluralRule> = ::phf::Map {
                key: 732231254413039614,
                disps: ::phf::Slice::Static(&[
                    (2, 0),
                    (0, 5),
                    (0, 2),
                    (0, 52),
                    (0, 5),
                    (0, 2),
                    (0, 21),
                    (0, 57),
                    (3, 58),
                    (2, 8),
                    (0, 13),
                    (2, 11),
                    (6, 94),
                    (1, 75),
                    (0, 48),
                    (73, 85),
                    (0, 88),
                    (0, 0),
                    (14, 22),
                    (1, 22),
                ]),
                entries: ::phf::Slice::Static(&[
                    ("pt", {
                        fn rule_pt(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_pt
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
                    ("de", {
                        fn rule_de(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_de
                    }),
                    ("cs", {
                        fn rule_cs(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_cs
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
                    ("my", {
                        fn rule_my(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_my
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
                    ("te", {
                        fn rule_te(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_te
                    }),
                    ("root", {
                        fn rule_root(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_root
                    }),
                    ("sw", {
                        fn rule_sw(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_sw
                    }),
                    ("sd", {
                        fn rule_sd(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_sd
                    }),
                    ("zu", {
                        fn rule_zu(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_zu
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
                    ("sh", {
                        fn rule_sh(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_sh
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
                    ("eu", {
                        fn rule_eu(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_eu
                    }),
                    ("el", {
                        fn rule_el(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_el
                    }),
                    ("zh", {
                        fn rule_zh(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_zh
                    }),
                    ("fy", {
                        fn rule_fy(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_fy
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
                    ("nb", {
                        fn rule_nb(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_nb
                    }),
                    ("uz", {
                        fn rule_uz(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_uz
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
                    ("gl", {
                        fn rule_gl(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_gl
                    }),
                    ("pl", {
                        fn rule_pl(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_pl
                    }),
                    ("mn", {
                        fn rule_mn(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_mn
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
                    ("prg", {
                        fn rule_prg(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_prg
                    }),
                    ("da", {
                        fn rule_da(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_da
                    }),
                    ("ia", {
                        fn rule_ia(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ia
                    }),
                    ("km", {
                        fn rule_km(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_km
                    }),
                    ("ky", {
                        fn rule_ky(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ky
                    }),
                    ("ps", {
                        fn rule_ps(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ps
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
                    ("dsb", {
                        fn rule_dsb(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_dsb
                    }),
                    ("ur", {
                        fn rule_ur(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ur
                    }),
                    ("he", {
                        fn rule_he(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_he
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
                    ("ce", {
                        fn rule_ce(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ce
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
                    ("sr", {
                        fn rule_sr(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_sr
                    }),
                    ("es", {
                        fn rule_es(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_es
                    }),
                    ("ar", {
                        fn rule_ar(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ar
                    }),
                    ("pa", {
                        fn rule_pa(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_pa
                    }),
                    ("ko", {
                        fn rule_ko(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ko
                    }),
                    ("ta", {
                        fn rule_ta(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ta
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
                    ("yue", {
                        fn rule_yue(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_yue
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
                    ("iw", {
                        fn rule_iw(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_iw
                    }),
                    ("is", {
                        fn rule_is(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_is
                    }),
                    ("hr", {
                        fn rule_hr(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_hr
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
                    ("lt", {
                        fn rule_lt(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_lt
                    }),
                    ("in", {
                        fn rule_in(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_in
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
                    ("an", {
                        fn rule_an(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_an
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
                    ("hsb", {
                        fn rule_hsb(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_hsb
                    }),
                    ("id", {
                        fn rule_id(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_id
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
                    ("et", {
                        fn rule_et(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_et
                    }),
                    ("sk", {
                        fn rule_sk(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_sk
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
                    ("fi", {
                        fn rule_fi(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_fi
                    }),
                    ("ml", {
                        fn rule_ml(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ml
                    }),
                    ("sl", {
                        fn rule_sl(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_sl
                    }),
                    ("th", {
                        fn rule_th(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_th
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
                    ("lv", {
                        fn rule_lv(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_lv
                    }),
                    ("bs", {
                        fn rule_bs(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_bs
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
                    ("si", {
                        fn rule_si(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_si
                    }),
                    ("bg", {
                        fn rule_bg(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_bg
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
                    ("ru", {
                        fn rule_ru(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_ru
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
                    ("nl", {
                        fn rule_nl(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_nl
                    }),
                    ("fa", {
                        fn rule_fa(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_fa
                    }),
                    ("kn", {
                        fn rule_kn(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_kn
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
                    ("am", {
                        fn rule_am(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_am
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
                    ("af", {
                        fn rule_af(po: &PluralOperands) -> PluralCategory {
                            {
                                PluralCategory::OTHER
                            }
                        };
                        rule_af
                    }),
                ]),
            };
            LANGUAGES.get(lang_code).cloned().ok_or(())
        }
    }
}
