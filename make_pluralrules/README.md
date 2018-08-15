# Make Plural Rules

`make_pluralrules` is a coe generator applications that turns a [Unicode CLDR plural rules](http://unicode.org/reports/tr35/tr35-numbers.html#Language_Plural_Rules) AST into Rust code.

[![crates.io](http://meritbadge.herokuapp.com/make_pluralrules)](https://crates.io/crates/make_pluralrules)
[![Build Status](https://travis-ci.org/unclenachoduh/pluralrules.svg?branch=master)](https://travis-ci.org/unclenachoduh/pluralrules)
[![Coverage Status](https://coveralls.io/repos/github/unclenachoduh/pluralrules/badge.svg?branch=master)](https://coveralls.io/github/unclenachoduh/pluralrules?branch=master)

The application is intended to generate code necessary for calculating correct plural rules categories.

Status
------

The generator currently generates code for cardinal plural rules from CLDR 33 into Rust 1.27 and above.

Launch make_pluralrules with:

```
cargo run -- -i <./path/to/cldr.json>... -o <./path/to/output.rs>
```

Local Development
-----------------

    cargo build
    cargo test

If you want to update the test fixtures to match your latest changes, please use:

	cargo regenerate-fixtures_within | cargo regenerate-fixtures

When submitting a PR please use  `cargo fmt`.

Contributors
------------

* [manishearth](https://github.com/manishearth)

Thank you to all contributors!

[CLDR]: http://cldr.unicode.org/
[PluralRules]: http://cldr.unicode.org/index/cldr-spec/plural-rules
[LDML Language Plural Rules Syntax]: http://unicode.org/reports/tr35/tr35-numbers.html#Language_Plural_Rules
