use criterion::black_box;
use criterion::criterion_group;
use criterion::criterion_main;
use criterion::Criterion;

use cldr_pluralrules_parser::parse_plural_rule;

const STRINGS: &[&str] = &[
    "n % 10 = 0 or n % 10 = 5..9 or n % 100 = 11..14 @integer 0, 5~19, 100, 1000, 10000, 100000, 1000000,   … @decimal 0.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 100.0, 1000.0, 10000.0, 100000.0, 1000000.0, …",
    "n % 10 = 2..4 and n % 100 != 12..14 @integer 2~4, 22~24, 32~34, 42~44, 52~54, 62, 102, 1002, … @deci  mal 2.0, 3.0, 4.0, 22.0, 23.0, 24.0, 32.0, 33.0, 102.0, 1002.0, …",
    "n % 10 = 1 and n % 100 != 11 @integer 1, 21, 31, 41, 51, 61, 71, 81, 101, 1001, … @decimal 1.0, 21.0  , 31.0, 41.0, 51.0, 61.0, 71.0, 81.0, 101.0, 1001.0, …",
    "n % 100 = 11..99 @integer 11~26, 111, 1011, … @decimal 11.0, 12.0, 13.0, 14.0, 15.0, 16.0, 17.0, 18.  0, 111.0, 1011.0, …",
];

fn cldr_parse_plural_rule(c: &mut Criterion) {
    c.bench_function("parse_plural_rules", |b| {
        b.iter(|| {
            for s in STRINGS {
                let _ = parse_plural_rule(black_box(s));
            }
        })
    });
}

criterion_group!(benches, cldr_parse_plural_rule,);
criterion_main!(benches);
