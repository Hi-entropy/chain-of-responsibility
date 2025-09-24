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

#[cfg(test)]
mod validator_test {
    use super::*;

    #[test]
    fn validates_correct_primitive_value() {
        let rules = vec![
            ValidationRule::new(
                Box::new(|x: &i32| *x > 0),
                "Value should be greater than 0".to_string()
            ),
            ValidationRule::new(
                Box::new(|x: &i32| *x < 100),
                "Value should be less than 100".to_string()
            )
        ];

        let value = 30;
        let result = Validator::validate(value, rules);

        assert!(result.is_ok());
    }
}