#![feature(test)]

extern crate test;

use intl_pluralrules::{IntlPluralRules, PluralRuleType};
use test::Bencher;
use unic_langid::LanguageIdentifier;

#[bench]
fn bench_create(b: &mut Bencher) {
    let langid_uk: LanguageIdentifier = "uk".parse().expect("Parsing failed.");
    let langid_de: LanguageIdentifier = "de".parse().expect("Parsing failed.");
    let langid_sk: LanguageIdentifier = "sk".parse().expect("Parsing failed.");
    b.iter(|| {
        IntlPluralRules::create(langid_uk.clone(), PluralRuleType::CARDINAL).unwrap();
        IntlPluralRules::create(langid_de.clone(), PluralRuleType::CARDINAL).unwrap();
        IntlPluralRules::create(langid_sk.clone(), PluralRuleType::ORDINAL).unwrap();
    });
}

#[bench]
fn bench_select(b: &mut Bencher) {
    let langid_pl: LanguageIdentifier = "pl".parse().expect("Parsing failed.");
    let ipr = IntlPluralRules::create(langid_pl.clone(), PluralRuleType::CARDINAL).unwrap();

    b.iter(|| {
        ipr.select(1).unwrap();
        ipr.select(2).unwrap();
        ipr.select(5).unwrap();
    });
}

#[bench]
fn bench_total(b: &mut Bencher) {
    b.iter(|| {
        let langid_pl: LanguageIdentifier = "pl".parse().expect("Parsing failed.");
        let ipr = IntlPluralRules::create(langid_pl.clone(), PluralRuleType::CARDINAL).unwrap();
        ipr.select(1).unwrap();
        ipr.select(2).unwrap();
        ipr.select(3).unwrap();
        ipr.select(4).unwrap();
        ipr.select(5).unwrap();
        ipr.select(25).unwrap();
        ipr.select(134).unwrap();
        ipr.select(5090).unwrap();
        ipr.select(910293019).unwrap();
        ipr.select(5.2).unwrap();
        ipr.select(-0.2).unwrap();
        ipr.select("12.06").unwrap();
    });
}
