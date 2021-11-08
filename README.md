# CLDR Plural Rules [![Build Status](https://travis-ci.org/zbraniecki/pluralrules.svg?branch=master)](https://travis-ci.org/zbraniecki/pluralrules) [![Coverage Status](https://coveralls.io/repos/github/zbraniecki/pluralrules/badge.svg?branch=master)](https://coveralls.io/github/zbraniecki/pluralrules?branch=master)

`cldr_pluralrules` is a collection of Rust crates for identifying the plural rule, according to [CLDR](https://github.com/unicode-cldr/cldr-core/blob/master/supplemental/plurals.json), for a given number.

The crates perform the following functions:

## intl_pluralrules [![crates.io](https://img.shields.io/crates/v/intl_pluralrules.svg)](https://crates.io/crates/intl_pluralrules)


This library returns the plural rule given numeric input.

## cldr_pluralrules_parser [![crates.io](https://img.shields.io/crates/v/cldr_pluralrules_parser.svg)](https://crates.io/crates/cldr_pluralrules_parser)


This library creates an AST from plural rules, according to [CLDR plural rule syntax](https://unicode.org/reports/tr35/tr35-numbers.html#Plural_rules_syntax).

## make_pluralrules [![crates.io](https://img.shields.io/crates/v/make_pluralrules.svg)](https://crates.io/crates/make_pluralrules)


This executable generates a Rust file with a public function representation of CLDR plural rules from a specified source.

## Status

### cldr_pluralrules_parser

The parser is fully compliant with UTS #35 version 33.

### make_pluralrules

Generates all cardinal plural rules for Rust 1.31 and above, based on CLDR 33.

### intl_pluralrules

Currently supports cardinal plural rules for all languages from CLDR 35.

Get Involved
------------

`cldr_pluralrules` is open-source, licensed under the Apache License, Version 2.0.  We encourage everyone to take a look at our code and we'll listen to your feedback.
