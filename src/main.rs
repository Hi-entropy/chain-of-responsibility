mod rule;
mod validator;
use crate::rule::ValidationRule;

fn main() {
    let rules = vec![
        ValidationRule::new(
            Box::new(|x: &i32| *x > 10),
            "Value should be greater than 10".to_string()
        ),
        ValidationRule::new(
            Box::new(|x: &i32| *x < 25),
            "Value should be less than 25".to_string()
        )
    ];
    let validated = validator::Validator::validate(12, rules);

    match validated {
        Ok(_) => println!("The data is valid"),
        Err(e) => println!("{e}") 
    }
}
