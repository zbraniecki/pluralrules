# CLDR PluralRules Parser

`cldr_pluralrules_parser` is a parser library for [CLDR][] [PluralRules][].

[![crates.io](https://img.shields.io/crates/v/cldr_pluralrules_parser.svg)](https://crates.io/crates/cldr_pluralrules_parser)
[![Build Status](https://travis-ci.org/zbraniecki/pluralrules.svg?branch=master)](https://travis-ci.org/zbraniecki/pluralrules)
[![Coverage Status](https://coveralls.io/repos/github/zbraniecki/pluralrules/badge.svg?branch=master)](https://coveralls.io/github/zbraniecki/pluralrules?branch=master)

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

[CLDR]: https://cldr.unicode.org/
[PluralRules]: https://cldr.unicode.org/index/cldr-spec/plural-rules
[LDML Language Plural Rules Syntax]: https://unicode.org/reports/tr35/tr35-numbers.html#Language_Plural_Rules
