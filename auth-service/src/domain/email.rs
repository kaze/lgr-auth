use validator::ValidateEmail;

// A tuple struct wrapping a String.
// The inner field is private, so the only way to construct an Email
// is through `Email::parse` — enforcing the validity invariant at the type level.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Email(String);

impl Email {
    // Returns Ok(Email) only when `s` passes email validation.
    // Callers receive a type-level proof of validity — no separate
    // "is_valid" check can be forgotten later.
    pub fn parse(s: String) -> Result<Self, String> {
        if s.validate_email() {
            Ok(Self(s))
        } else {
            Err(format!("{:?} is not a valid email address", s))
        }
    }
}

// AsRef<str> exposes the inner string as a &str without giving callers
// a mutable handle or ownership. It integrates with Rust's standard
// "accept anything string-like" convention: functions that take `impl AsRef<str>`
// will accept &Email transparently.
impl AsRef<str> for Email {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fake::faker::internet::en::SafeEmail;
    use fake::Fake;
    use quickcheck::TestResult;
    use quickcheck_macros::quickcheck;
    use rand::SeedableRng;

    // --- valid input ---

    #[test]
    fn valid_email_parses_correctly() {
        // fake generates realistic RFC-compliant email addresses.
        // We seed the RNG so the test is deterministic across runs.
        let mut rng = rand::rngs::StdRng::seed_from_u64(42);
        let email: String = SafeEmail().fake_with_rng(&mut rng);
        assert!(Email::parse(email).is_ok());
    }

    // --- invalid input edge cases ---

    #[test]
    fn empty_string_is_rejected() {
        assert!(Email::parse("".to_string()).is_err());
    }

    #[test]
    fn missing_at_sign_is_rejected() {
        assert!(Email::parse("notanemail.com".to_string()).is_err());
    }

    #[test]
    fn missing_domain_is_rejected() {
        assert!(Email::parse("user@".to_string()).is_err());
    }

    #[test]
    fn missing_local_part_is_rejected() {
        assert!(Email::parse("@domain.com".to_string()).is_err());
    }

    #[test]
    fn whitespace_only_is_rejected() {
        assert!(Email::parse("   ".to_string()).is_err());
    }

    // --- property-based test ---

    // quickcheck generates hundreds of arbitrary String values and checks
    // that our property holds for each one.  Any string that contains no '@'
    // character can never be a valid email, so we assert it always fails.
    // Strings that do contain '@' are discarded (not counted as failures)
    // because they might or might not be valid emails.
    #[quickcheck]
    fn no_at_sign_always_fails(s: String) -> TestResult {
        if s.contains('@') {
            return TestResult::discard();
        }
        TestResult::from_bool(Email::parse(s).is_err())
    }

    // Property: a successfully parsed Email, when read back via AsRef,
    // equals the original input string.
    #[test]
    fn as_ref_returns_original_string() {
        let raw = "user@example.com".to_string();
        let email = Email::parse(raw.clone()).unwrap();
        assert_eq!(email.as_ref(), raw.as_str());
    }
}
