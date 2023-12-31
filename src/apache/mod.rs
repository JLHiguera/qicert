use std::process::{Child, Command};
use std::{error::Error, fmt::Display};

use crate::domain::Domain;
use crate::webserver::WebServer;

pub(crate) mod config_file;
pub(crate) mod configurator;
pub(crate) mod http_config;

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
    const WEBSERVER_SBIN_PATH: &'a str = "/usr/sbin/apache2";
    const BINARY_NAME: &'a str = "apache2";
}

impl Apache {
    const SITE_ENABLE_COMMAND: &str = "a2ensite";

    pub fn reload() -> Result<(), ApacheError> {
        Self::_reload(ApacheError::CannotReload)
    }

    pub fn enable_site(domain: &Domain) -> Result<(), ApacheError> {
        let domain = format!("{}.{}", domain.get_name(), domain.get_tld());

        let output = Command::new(Self::SITE_ENABLE_COMMAND)
            .arg(&domain)
            .spawn()
            .and_then(Child::wait_with_output)
            .map_err(|_| ApacheError::BadConfiguration)?;

        if !output.status.success() {
            return Err(ApacheError::BadConfiguration);
        }

        Ok(())
    }
}
