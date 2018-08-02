# INTL Plural Rules

`intl_pluralrules` categorizes numbers by plural operands. See [Unicode Plural Rules](http://unicode.org/reports/tr35/tr35-numbers.html#Language_Plural_Rules)


[![crates.io](http://meritbadge.herokuapp.com/intl_pluralrules)](https://crates.io/crates/intl_pluralrules)
[![Build Status](https://travis-ci.org/unclenachoduh/pluralrules.svg?branch=master)](https://travis-ci.org/unclenachoduh/pluralrules)
[![Coverage Status](https://coveralls.io/repos/github/unclenachoduh/pluralrules/badge.svg?branch=master)](https://coveralls.io/github/unclenachoduh/pluralrules?branch=master)

This library is intended to be used to find the plural category of numeric input.

Status
------

Currently produces operands compliant with CLDR 33 into Rust 1.27 and above.

**Using External CLDR Data**

If you would like to use plural rules that are not the specified version above, you can regenerate intl_pluralrules's internal rules with the command:

	cargo regenerate-data

You will need to replace the JSON files under `/cldr_data/` with your own CLDR-compliant JSON files.

Local Development
-----------------

    cargo build
    cargo test

When submitting a PR please use  `cargo fmt`.

[CLDR]: http://cldr.unicode.org/
[PluralRules]: http://cldr.unicode.org/index/cldr-spec/plural-rules
[LDML Language Plural Rules Syntax]: http://unicode.org/reports/tr35/tr35-numbers.html#Language_Plural_Rules