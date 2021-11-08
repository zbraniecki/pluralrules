# Make Plural Rules

`make_pluralrules` is a code generator application that turns a [Unicode CLDR plural rules](https://unicode.org/reports/tr35/tr35-numbers.html#Language_Plural_Rules) AST into Rust code.

[![crates.io](https://img.shields.io/crates/v/make_pluralrules.svg)](https://crates.io/crates/make_pluralrules)
[![Build Status](https://travis-ci.org/zbraniecki/pluralrules.svg?branch=master)](https://travis-ci.org/zbraniecki/pluralrules)
[![Coverage Status](https://coveralls.io/repos/github/zbraniecki/pluralrules/badge.svg?branch=master)](https://coveralls.io/github/zbraniecki/pluralrules?branch=master)

The application is intended to generate code necessary for calculating correct plural rules categories.

Status
------

The generator currently generates code for cardinal plural rules from CLDR 34 into Rust 1.31 and above.

Launch make_pluralrules with:

```
cargo run -- -i <./path/to/cldr.json>... -o <./path/to/output.rs>
```

Local Development
-----------------

    cargo build
    cargo test

If you want to update the test fixtures to match your latest changes, please use:

	cargo regenerate_fixtures_within | cargo regenerate_fixtures

When submitting a PR please use  `cargo fmt`.

Contributors
------------

* [manishearth](https://github.com/manishearth)

Thank you to all contributors!

[CLDR]: https://cldr.unicode.org/
[PluralRules]: https://cldr.unicode.org/index/cldr-spec/plural-rules
[LDML Language Plural Rules Syntax]: https://unicode.org/reports/tr35/tr35-numbers.html#Language_Plural_Rules
