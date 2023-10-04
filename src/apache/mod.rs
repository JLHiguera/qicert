use std::{fmt::Display, error::Error};

use crate::webserver::WebServer;


pub(crate) mod http_config;
pub(crate) mod config_file;

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
    const BINARY_NAME: &'a str = "Apache";
}

impl Apache {
    pub fn reload() -> Result<(), ApacheError> {
        Self::_reload(ApacheError::CannotReload)
    }
}