// Same newtype pattern as Email: the private inner field means the only
// construction path is through `parse`, giving the type-level guarantee
// that every Password value has been validated.
#[derive(Debug, Clone, PartialEq)]
pub struct Password(String);

impl Password {
    pub fn parse(s: String) -> Result<Self, String> {
        if s.len() >= 8 {
            Ok(Self(s))
        } else {
            Err("Password must be at least 8 characters long".to_string())
        }
    }
}

impl AsRef<str> for Password {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn valid_password_parses_correctly() {
        assert!(Password::parse("password1".to_string()).is_ok());
    }

    #[test]
    fn exactly_eight_chars_is_valid() {
        assert!(Password::parse("12345678".to_string()).is_ok());
    }

    #[test]
    fn seven_chars_is_rejected() {
        assert!(Password::parse("1234567".to_string()).is_err());
    }

    #[test]
    fn empty_string_is_rejected() {
        assert!(Password::parse("".to_string()).is_err());
    }

    #[test]
    fn as_ref_returns_original_string() {
        let raw = "supersecret".to_string();
        let password = Password::parse(raw.clone()).unwrap();
        assert_eq!(password.as_ref(), raw.as_str());
    }
}
