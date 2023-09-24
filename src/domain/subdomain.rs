use std::fmt::Display;

#[derive(Debug)]
pub struct SubDomain(String);

impl Display for SubDomain {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl SubDomain {
    pub fn is_valid(subdomain: &str) -> bool {
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
