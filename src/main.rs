mod rule;
mod validator;
use crate::rule::ValidationRule;

fn main() {
    let rule1 = ValidationRule::new(
        Box::new(|x: &i32| *x > 10),
        "Number should be greater than 10".to_string()
    );

    let rule2 = ValidationRule::new(
        Box::new(|x: &i32| *x > 25),
        "Number should be less than 25".to_string()
    );

    let rules = vec![
        rule1,
        rule2
    ];
    let validated = validator::Validator::validate(12, rules);

    match validated {
        Ok(_) => println!("The data is valid"),
        Err(e) => println!("{e}") 
    }
}
