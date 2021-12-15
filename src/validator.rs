use async_graphql::CustomValidator;
use lazy_static::lazy_static;
use fancy_regex::Regex;

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

lazy_static! {
    static ref RE: Regex =
        Regex::new(r##"(?=.*[a-z])(?=.*[A-Z])(?=.*[0-9])(?=.*[ !"#$%&'()*+,./:;<=>?@^_`{|}~\[\]\\-])(?=.{8,24})"##).unwrap();
}

pub struct PasswordValidator {}

impl PasswordValidator {
    pub fn new() -> Self {
        PasswordValidator {}
    }
}

impl CustomValidator<String> for PasswordValidator {
    fn check(&self, value: &String) -> Result<(), String> {
        if RE.is_match(value).unwrap_or(false) {
            Ok(())
        } else {
            Err(String::from("invalid password"))
        }
    }
}
