pub struct ValidationRule<T>
{
    pub predicate: Box<dyn Fn(&T) -> bool>,
    pub text: String,
}

impl<T> ValidationRule<T> 
{
    pub fn new(predicate: Box<dyn Fn(&T) -> bool>, text: String) -> Self {
        Self {
            predicate,
            text,
        }
    }
}