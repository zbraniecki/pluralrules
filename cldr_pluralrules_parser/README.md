# CLDR PluralRules Parser

`cldr_pluralrules_parser` is a parser library for [CLDR][] [PluralRules][].

[![crates.io](http://meritbadge.herokuapp.com/cldr_pluralrules_parser)](https://crates.io/crates/cldr_pluralrules_parser)
[![Build Status](https://travis-ci.org/unclenachoduh/pluralrules.svg?branch=master)](https://travis-ci.org/unclenachoduh/pluralrules)
[![Coverage Status](https://coveralls.io/repos/github/unclenachoduh/pluralrules/badge.svg?branch=master)](https://coveralls.io/github/unclenachoduh/pluralrules?branch=master)

The library closely follows [LDML Language Plural Rules Syntax][] and is intended to be
used at build time or runtime to construct operations necessary for calculating
corret plural rules categories.

Status
------

The parser is functionally complete and follows the current version of the syntax.

It is lenient and open to contributions in the area of conformance, testing, and
rejecting invalid input.

Local Development
-----------------

    cargo build
    cargo test

When submitting a PR please use  `cargo fmt`.

[CLDR]: http://cldr.unicode.org/
[PluralRules]: http://cldr.unicode.org/index/cldr-spec/plural-rules
[LDML Language Plural Rules Syntax]: http://unicode.org/reports/tr35/tr35-numbers.html#Language_Plural_Rules
