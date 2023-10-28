use std::{
    error::Error,
    fmt::Display,
    path::PathBuf,
    process::{Command, Stdio, Child},
};

use crate::{domain::Domain, webroot::WebRoot};

#[derive(Debug)]
pub enum CertBotError {
    NotInstalled,
    ProcessFailure,
}

impl Error for CertBotError {}

impl Display for CertBotError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CertBotError::NotInstalled => write!(f, "Certbot is not installed"),
            CertBotError::ProcessFailure => write!(f, "Certbot failed to create certificate"),
        }
    }
}

pub struct Certer;

impl Certer {
    const CERTBOT_BIN_PATH: &str = "/usr/bin/certbot";

    pub fn is_installed() -> bool {
        PathBuf::from(Self::CERTBOT_BIN_PATH).is_file()
    }

    pub fn run(domain: &Domain) -> Result<(), CertBotError> {
        if !Self::is_installed() {
            return Err(CertBotError::NotInstalled);
        }

        let root = WebRoot::build_path_string(domain);

        Command::new("certbot")
            .arg("certonly")
            .arg("--webroot")
            .arg("-w")
            //.arg(root.as_str())
            .arg("/var/www/.well-known/challenge")
            .arg("-d")
            .arg(domain.to_string().as_str())
            .stdout(Stdio::piped())
            .spawn()
            .and_then(Child::wait_with_output)
            .map_err(|_| CertBotError::ProcessFailure)?;

        Ok(())
    }
}
