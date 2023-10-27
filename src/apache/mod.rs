use std::{fmt::Display, error::Error};
use std::process::{Command, Child};


use crate::domain::Domain;
use crate::webserver::WebServer;


pub(crate) mod http_config;
pub(crate) mod config_file;
pub(crate) mod configurator;

#[derive(Debug, Copy, Clone)]
pub enum ApacheError {
    NotInstalled,
    BadConfiguration,
    CannotReload,
}

impl Error for ApacheError {}

impl Display for ApacheError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::BadConfiguration => write!(f, "Bad configuration for Apache"),
            Self::CannotReload => write!(f, "Apache failed to reload"),
            Self::NotInstalled => write!(f, "Apache was not found"),
        }
    }
}

pub struct Apache;

impl<'a> WebServer<'a> for Apache {
    const WEBSERVER_BIN_PATH: &'a str = "/usr/bin/apache";
    const BINARY_NAME: &'a str = "apache";
}

impl Apache {
    const SITE_ENABLE_COMMAND: &str = "a2ensite";

    pub fn reload() -> Result<(), ApacheError> {
        Self::_reload(ApacheError::CannotReload)
    }

    pub fn enable_site(domain: &Domain) -> Result<(), ApacheError> {
        let output = Command::new(Self::SITE_ENABLE_COMMAND)
            .arg(domain.to_string())
            .spawn()
            .and_then(Child::wait_with_output)
            .map_err(|_| ApacheError::BadConfiguration)?;


        if !output.status.success() {
            return Err(ApacheError::BadConfiguration);
        }

        Ok(())
    }
}