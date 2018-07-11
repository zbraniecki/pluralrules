pub mod operands;
mod rules;

#[derive(Debug, Eq, PartialEq)]
pub enum PluralCategory {
    ONE,
    TWO,
    FEW,
    MANY,
    OTHER
}

pub fn select_plural_category<S: ToString>(lang: &str, number: S) -> PluralCategory {
    let f = rules::get_pr(lang);
    let ops = operands::PluralOperands::new(number);
    f(ops)
}

#[cfg(test)]
mod tests {
    use super::select_plural_category;
    use super::PluralCategory;
    #[test]
    fn it_works() {
        assert_eq!(select_plural_category("naq", 1), PluralCategory::ONE);
        assert_eq!(select_plural_category("naq", 2), PluralCategory::TWO);
        assert_eq!(select_plural_category("naq", 5), PluralCategory::OTHER);
    }
}
