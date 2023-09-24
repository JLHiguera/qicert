use std::path::PathBuf;
use std::{fmt::Display, error::Error};
use crate::domain::Domain;
use std::fs;
use std::process::Command;

#[derive(Debug)]
pub enum WebRootError {
    DoesNotExist,
    Permissions,
    AlreadyExists,
    CreationFailure,
}

impl Display for WebRootError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::DoesNotExist => write!(f, "Webroot does not exists."),
            Self::Permissions => write!(f, "Permission error when creating webroot"),
            Self::AlreadyExists => write!(f, "Webroot already exists"),
            Self::CreationFailure => write!(f, "Webroot could not be created"),
        }
    }
}

impl Error for WebRootError {}

pub struct WebRoot;

impl WebRoot {
    const BASE_PATH: &str = "/var/www";

    fn create(domain: &Domain) -> Result<(), WebRootError> {
        if Self::exists(domain) {
            return Err(WebRootError::AlreadyExists);
        }

        let mut dir = fs::DirBuilder::new();

        dir.recursive(true)
            .create(Self::build_pathbuf(domain))
            .map_err(|_| WebRootError::CreationFailure)?;

        Ok(())
    }

    fn create_dummy_html(domain: &Domain) -> Result<(), WebRootError> {
        if ! Self::exists(domain) {
            return Err(WebRootError::CreationFailure);
        }

        let mut root_path = Self::build_pathbuf(domain);

        root_path.push("index.html");

        fs::write(root_path, "<p>hello</p>")
            .map_err(|_| WebRootError::CreationFailure)?;

        Ok(())
    }

    pub fn create_and_set_chown(domain: &Domain) -> Result<(), WebRootError> {
        if Self::has_files(domain) {
            return Err(WebRootError::AlreadyExists);
        }

        Self::create(domain)?;

        Self::create_dummy_html(domain)?;

        Self::chown_to_www(domain)?;

        Ok(())
    }

    fn exists(domain: &Domain) -> bool {
        let webroot_path = Self::build_pathbuf(domain);

        webroot_path.is_dir()
    }

    fn has_files(domain: &Domain) -> bool {
        if ! Self::exists(domain) {
            return false;
        }

        let webroot_path = Self::build_pathbuf(domain);

        webroot_path.read_dir()
            .map(|e| e.count() > 0)
            .expect("FIXME: could not read webroot directory")
    }

    pub fn build_path_string(domain: &Domain) -> String {
        Self::build_pathbuf(domain).to_owned()
            .to_string_lossy()
            .to_string()
    }

    pub fn build_pathbuf(domain: &Domain) -> PathBuf {
        let mut path = PathBuf::from(Self::BASE_PATH);

        path.push(domain.to_string());

        path.push("public");

        path
    }

    fn chown_to_www(domain: &Domain) -> Result<(), WebRootError> {
        if ! Self::exists(domain) {
            return Err(WebRootError::DoesNotExist);
        }

        let path = Self::build_path_string(domain);

        let _ = Command::new("chown")
            .arg("-R")
            .arg("www-data:www-data")
            .arg(path.as_str())
            .spawn()
            .and_then(|mut p| p.wait())
            .map_err(|_| WebRootError::Permissions);

        Ok(())
    }

}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn expected_path() {
        let expected_path = PathBuf::from("/var/www/example.com/public");

        let domain = Domain::new("example", "com", None).unwrap();

        let webroot = WebRoot::build_pathbuf(&domain);

        assert_eq!(expected_path, webroot);
    }
}