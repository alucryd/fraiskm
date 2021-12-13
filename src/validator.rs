use async_graphql::CustomValidator;

pub struct EmailValidator {}

impl EmailValidator {
    pub fn new() -> Self {
        EmailValidator {}
    }
}

impl CustomValidator<String> for EmailValidator {
    fn check(&self, value: &String) -> Result<(), String> {
        if mailchecker::is_valid(value) {
            Ok(())
        } else {
            Err(String::from("invalid email"))
        }
    }
}
