use crate::rule::ValidationRule;

pub struct Validator;

impl Validator {
    pub fn validate<T>(value: T, rules: Vec<ValidationRule<T>>) -> Result<(), String> {
        for rule in rules {
            if !(rule.predicate)(&value) {
                return Err(rule.text);
            }
        }

        Ok(())
    }
}