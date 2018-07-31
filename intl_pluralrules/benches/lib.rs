#![feature(test)]

extern crate intl_pluralrules;
extern crate test;

use intl_pluralrules::{IntlPluralRules, PluralRuleType};
use test::Bencher;

#[bench]
fn bench_create(b: &mut Bencher) {
    b.iter(|| {
        IntlPluralRules::create("uk", PluralRuleType::CARDINAL).unwrap();
        IntlPluralRules::create("de", PluralRuleType::CARDINAL).unwrap();
        IntlPluralRules::create("sk", PluralRuleType::ORDINAL).unwrap();
    });
}

#[bench]
fn bench_select(b: &mut Bencher) {
    let ipr = IntlPluralRules::create("pl", PluralRuleType::CARDINAL).unwrap();

    b.iter(|| {
        ipr.select(1).unwrap();
        ipr.select(2).unwrap();
        ipr.select(5).unwrap();
    });
}

#[bench]
fn bench_total(b: &mut Bencher) {
    b.iter(|| {
        let ipr = IntlPluralRules::create("pl", PluralRuleType::CARDINAL).unwrap();
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
