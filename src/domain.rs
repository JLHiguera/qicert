use std::{fmt::Display, error::Error, str::FromStr};

#[derive(Debug, PartialEq, Eq)]
pub struct Domain {
    name: String,
    tld: String,
    subdomain: Option<String>,
}
#[derive(Debug)]
pub enum DomainError {
    MissingTld,
    InvalidName,
    InvalidSubdomain,
    InvalidTld,
}

impl Display for DomainError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        //write!(f, "{}", self)
        match self {
            Self::MissingTld => write!(f, "The Domain has no TLD"),
            Self::InvalidName => write!(f, "Invalid domain name"),
            Self::InvalidSubdomain => write!(f, "The subdomain given is invalid"),
            Self::InvalidTld => write!(f, "The TLD is invalid"),
        }
    }
}

#[derive(Debug)]
struct SubDomain(String);

impl Display for SubDomain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl SubDomain {
    fn is_valid(subdomain: &str) -> bool {
        if subdomain.is_empty() {
            return false;
        }

        if subdomain.starts_with('.') || subdomain.ends_with('.') {
            return false;
        }

        if subdomain.starts_with('-') || subdomain.ends_with('-') {
            return false;
        }

        subdomain.chars().all(Self::is_valid_char)
    }

    fn is_valid_char(char: char) -> bool {
        matches!(char, 'a'..='z' | '0'..='9' | '.' | '-')
    }
}

#[derive(Debug)]
struct DomainName(String);

impl DomainName {
    fn is_valid_char(char: char) -> bool {
        matches!(char, 'a'..='z' | '0'..='9' | '-')
    }

    fn is_valid(name: &str) -> bool {
        if name.is_empty() {
            return false;
        }

        if name.starts_with('-') | name.ends_with('-') {
            return false;
        }

        name.chars().all(Self::is_valid_char)
    }
}

impl Display for DomainName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl FromStr for DomainName {
    type Err = DomainError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        let value = value.to_lowercase();

        if ! Self::is_valid(value.as_str()) {
            return Err(Self::Err::InvalidName);
        }

        Ok(Self(value))
    }
}

impl std::ops::Add<Tld> for DomainName {
    type Output = Domain;

    fn add(self, rhs: Tld) -> Self::Output {
        let name = self;

        Self::Output {
            name: name.0,
            tld: rhs.to_string(),
            subdomain: None,
        }
    }
}


#[derive(Debug)]
struct Tld(String);

impl Display for Tld {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Tld {
    fn get(&self) -> String {
        self.0.to_owned()
    }

    // fn new(value: String) -> Self {
    //     Self(value)
    // }

    fn is_valid<S: AsRef<str>>(value: S) -> bool {
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
enum TldError {
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

impl Error for DomainError {}

impl Domain {
    pub fn new<S: Into<String>>(name: S, tld: S, subdomain: Option<String>) -> Result<Self, DomainError> {
        fn inner(name: String, tld: String, subdomain: Option<String>) -> Result<Domain, DomainError> {
            if ! DomainName::is_valid(&name) {
                return Err(DomainError::InvalidName);
            }

            if ! Tld::is_valid(&tld) {
                return Err(DomainError::InvalidTld);
            }

            let subdomain = match subdomain {
                Some(sub) if SubDomain::is_valid(&sub) => Some(sub),
                Some(_) => return Err(DomainError::InvalidSubdomain),
                None => None,
            };

            let domain = Domain {
                name, tld, subdomain,
            };

            Ok(domain)
        }
        
        inner(name.into(), tld.into(), subdomain)
    }

    #[cfg(test)]
    pub fn new_unchecked(domain: &str) -> Self {
        Self::from_str(domain).unwrap()
    }

    pub fn get_name(&self) -> String {
        self.name.to_owned()
    }

    pub fn get_tld(&self) -> String {
        self.tld.to_owned()
    }

    fn name_with_tld(&self) -> String {
        format!("{}.{}", self.name, self.tld)
    }

    fn has_subdomain(&self) -> bool {
        self.subdomain.is_some()
    }

    fn get_subdomain(&self) -> Option<String> {
        self.subdomain.clone()
    }

    fn is_valid_char(char: char) -> bool {
        matches!(char, 'a'..='z' | '0'..='9'| '.')
    }
    
    pub fn conf_file_name(&self) -> String {
        format!("{}.{}.conf", self.name, self.tld)
    }
}

#[cfg(test)]
impl TryFrom<&str> for Domain {
    type Error = DomainError;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Self::from_str(value)
    }
}

impl Display for Domain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.subdomain {
            Some(subdomain) => write!(f, "{}.{}.{}", subdomain, self.name, self.tld),
            None => write!(f, "{}.{}", self.name, self.tld),
        }
    }
}

#[cfg(test)]
impl FromStr for Domain {
    type Err = DomainError;

    fn from_str(domain: &str) -> Result<Self, Self::Err> {
        let domain = domain.to_lowercase();

        if domain.len() < 4 || !domain.chars().all(Self::is_valid_char) {
            return Err(DomainError::InvalidName);
        }

        let parts: Vec<&str> = domain.split('.').collect();
        
        if parts.iter().any(|s| s.is_empty()) {
            return Err(DomainError::InvalidName);
        }

        //FIXME: remove unwraps
        //FIXME: This only works for single word TLDs.
        match parts.len() {
            0 | 1 => Err(DomainError::MissingTld                                                    ),
            2 => Ok(Self { name: parts.first().unwrap().to_string(), 
                tld: parts.last().unwrap().to_string(),
                subdomain: None}),
            3 => Ok (Self {
                subdomain: Some(parts.first().unwrap().to_string()),
                name: parts.get(1).unwrap().to_string(),
                tld: parts.get(2).unwrap().to_string(),
            }),
            _ => Err(DomainError::InvalidName),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_domain_from_str() {
        let domains = vec![
            ("example.com", true),
            ("example.com ", false),
            ("www.example.com", true),
            (".", false),
            ("..", false),
            ("...", false),
            ("....", false),
            (".....", false),
            (".......", false),
            ("a.a", false),
            ("#.#", false),
            ("a.com", true),
            ("a#.com", false),
            ("#a.com", false),
            ("a!#@().com", false),
            ("abc.!#@()com", false),
            ("abc.com!#@()", false),
            ("", false),
            ("ww w.example.com", false),
            ("www. example.com", false),
            ("www.example .com", false),
            ("www.example. com", false),
            (r#"www.example.\ com"#, false),

        ];

        for (domain, expected) in domains {
            assert_eq!(Domain::try_from(domain).is_ok(), expected, "\n TESTING domain as {} and {}", domain, expected)
        };
    }

    #[test]
    fn build_from_parts() {
        let expected = Domain::new_unchecked("example.com");

        let name = DomainName::from_str("example").unwrap();

        let tld = Tld::from_str("com").unwrap();

        let domain = name + tld;

        assert_eq!(domain.to_string(), "example.com");

        assert_eq!(expected, domain);
    }

    #[test]
    fn build_from_parts_str() {
        let expected = Domain::new_unchecked("example.com");

        let domain = Domain::new("example", "com", None);

        assert!(domain.is_ok());
        
        if let Ok(domain) = domain {
            assert_eq!(domain, expected);
        }
    }

    #[test]
    fn build_from_parts_str_without_subdomain() {
        let domain_parts = vec![
            ("example", "com", true),
            ("example1", "com.mx", true),
            ("example", "edu.es", true),
            ("example", "com.mx.", false),
            ("", "com.mx", false),
            (".", "com", false),
            ("..", "com", false),
            (".", ".", false),
            ("example", "", false),
            ("example ", "com", false),
            ("example", ".com", false),
            ("example", "com ", false),
            ("example", "com.", false),
            ("example.", "com", false),
            ("example.", "com", false),
            (".example", "com", false),
            ("example-me", "com", true),
            ("-example", "com", false),
            ("example-", "com", false),
        ];

        for (domain_name, tld, expected) in domain_parts {
            let domain = Domain::new(domain_name, tld, None);

            assert_eq!(domain.is_ok(), expected, "domain tested was: {domain_name}, and {tld}");
        }
    }

    #[test]
    fn build_from_parts_str_with_subdomain() {
        let domain_parts = vec![
            ("example", "com", "test1", true),
            ("example", "com.mx", "", false),
            ("example", "com.mx", "staging01", true),
            ("example", "com.mx", "staging02.test", true),
            ("example", "net", "staging-02.test", true),
            ("example", "net", "staging02-", false),
        ];

        for (domain_name, tld, subdomain, expected) in domain_parts {
            let domain = Domain::new(domain_name, tld, Some(subdomain.to_string()));

            assert_eq!(domain.is_ok(), expected, "name: {domain_name}, tld: {tld}, subdomain: {subdomain}");
        }
    }

    #[test]
    fn build_from_parts_with_subdomain_then_into_string() {
        let domain_parts = vec![
            ("example", "com", "test1", "test1.example.com"),
            ("example", "com.mx", "staging01", "staging01.example.com.mx"),
            ("example", "com.mx", "staging02.test", "staging02.test.example.com.mx"),
            ("example", "net", "staging-02.test", "staging-02.test.example.net"),
            ("example", "net", "staging-02.test", "staging-02.test.example.net"),
        ];

        for (domain_name, tld, subdomain, expected) in domain_parts {
            let domain = Domain::new(domain_name, tld, Some(subdomain.to_string()));

            assert!(domain.is_ok());

            if let Ok(domain) = domain {
                assert_eq!(domain.to_string(), expected, "domain given: {domain},expected: {expected}");
            }
        }
    }
}