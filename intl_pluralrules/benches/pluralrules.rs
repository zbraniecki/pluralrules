use criterion::criterion_group;
use criterion::criterion_main;
use criterion::BenchmarkId;
use criterion::Criterion;

use intl_pluralrules::{operands::PluralOperands, PluralRuleType, PluralRules};
use unic_langid::{langid, LanguageIdentifier};

fn plural_rules(c: &mut Criterion) {
    let langs = &["uk", "de", "sk", "ar", "fr", "it", "en", "cs", "es", "zh"];
    let langs: Vec<LanguageIdentifier> = langs
        .iter()
        .map(|l| l.parse().expect("Parsing failed"))
        .collect();

    c.bench_with_input(
        BenchmarkId::new("construct", langs.len()),
        &langs,
        |b, langs| {
            b.iter(|| {
                for lang in langs {
                    PluralRules::create(lang.clone(), PluralRuleType::ORDINAL).unwrap();
                    PluralRules::create(lang.clone(), PluralRuleType::CARDINAL).unwrap();
                }
            });
        },
    );

    let samples = &[
        1, 2, 3, 4, 5, 25, 134, 910293019, 12, 1412, -12, 15, 2931, 31231, 3123, 13231, 91, 0, 231,
        -2, -45, 33, 728, 2, 291, 24, 479, 291, 778, 919, 93,
    ];

    let langid_pl = langid!("pl");
    let ipr = PluralRules::create(langid_pl.clone(), PluralRuleType::CARDINAL).unwrap();

    c.bench_with_input(
        BenchmarkId::new("select", samples.len()),
        samples,
        |b, samples| {
            b.iter(|| {
                for value in samples {
                    ipr.select(*value).unwrap();
                }
            });
        },
    );

    c.bench_function("total", |b| {
        b.iter(|| {
            let ipr = PluralRules::create(langid_pl.clone(), PluralRuleType::CARDINAL).unwrap();
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
        })
    });
}

fn float_operands(c: &mut Criterion) {
    let samples = &[
        0.23_f64,
        2.202_f64,
        0.015_f64,
        91.212_f64,
        5.5_f64,
        5.05_f64,
        212.0212_f64,
        5.0_f64,
    ];

    c.bench_function("parse_float_operands", move |b| {
        b.iter(|| {
            for sample in samples {
                let _ = PluralOperands::from(*sample);
            }
        })
    });
}

criterion_group!(benches, plural_rules, float_operands);
criterion_main!(benches);
