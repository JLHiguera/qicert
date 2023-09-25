use std::{str::FromStr, fmt::Display};

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Tld(String);

impl Display for Tld {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Tld {
    pub fn get(&self) -> String {
        self.0.to_owned()
    }

    pub fn is_valid<S: AsRef<str>>(value: S) -> bool {
        fn inner(value: &str) -> bool {
            if value.is_empty() {
                return false;
            }

            if value.starts_with('.') || value.ends_with('.') {
                return false;
            }


            value.chars().all(Tld::is_valid_char)
        }

        inner(value.as_ref())
    }

    fn is_valid_char(char: char) -> bool {
        matches!(char,  'a'..='z' | '0'..='9'| '.')
    }
}
#[derive(Debug)]
pub enum TldError {
    InvalidCharset,
    TooShort,
}

impl Display for TldError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::InvalidCharset => write!(f, "The TLD has an invalid character"),
            Self::TooShort => write!(f, "The TLD is too short"),
        }
    }
}

impl FromStr for Tld {
    type Err = TldError;
    
    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let value = value.to_lowercase();

        if value.len() < 2 {
            return Err(Self::Err::TooShort);
        }

        if ! Self::is_valid(value.as_str()) {
            return Err(Self::Err::InvalidCharset);
        }

        Ok(Self(value))
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn valid_tld_from_str() {
        let tlds = vec![
            ("com", true),
            ("com.", false),
            (".com", false),
            (".net.", false),
            ("com.ca", true),
            ("com.ca.", false),
            ("com-ca", false),
            ("com ca", false),
            ("COM", true),
            ("a", false),
            ("1", false),
            ("!net#$@", false),
            ("", false),
        ];

        for (value, expected) in tlds {
            let tld = Tld::from_str(value);

            assert_eq!(tld.is_ok(), expected, "value used: {value} with expected result: {expected}");
        }
    }

    #[test]
    fn valid_tld_matches_expected_str() {
        let tlds = vec![
            ("net", "net"),
            ("NET", "net"),
            ("Net", "net"),
            ("nET", "net"),
            ("EDU.cA", "edu.ca"),
        ];

        for (value, expected) in tlds {
            let tld = Tld::from_str(value);

            assert!(tld.is_ok());

            if let Ok(tld) = tld  {
                assert_eq!(tld.to_string(), expected);
            }
        }
    }
}