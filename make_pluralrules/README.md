# Make Plural Rules

`make_pluralrules` is a coe generator applications that turns a [CLDR plural rules][] AST into Rust code.

[![crates.io](http://meritbadge.herokuapp.com/make_pluralrules)](https://crates.io/crates/make_pluralrules)
[![Build Status](https://travis-ci.org/unclenachoduh/pluralrules.svg?branch=master)](https://travis-ci.org/unclenachoduh/pluralrules)
[![Coverage Status](https://coveralls.io/repos/github/unclenachoduh/pluralrules/badge.svg?branch=master)](https://coveralls.io/github/unclenachoduh/pluralrules?branch=master)

The application is intended to generate code necessary for calculating correct plural rules categories.

Status
------

The generator currently generates code for cardinal plural rules from CLDR 33 into Rust 1.27 and above.

Local Development
-----------------

    cargo build
    cargo test

When submitting a PR please use  `cargo fmt`.

[CLDR]: http://cldr.unicode.org/
[PluralRules]: http://cldr.unicode.org/index/cldr-spec/plural-rules
[LDML Language Plural Rules Syntax]: http://unicode.org/reports/tr35/tr35-numbers.html#Language_Plural_Rules
