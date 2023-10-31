pub mod config_file;
pub(crate) mod configurator;
pub mod http_config;
pub(crate) mod linker;
pub(crate) mod sites;

use std::{
    error::Error,
    fmt::Display,
    process::{Child, Command},
};

use crate::webserver::WebServer;

#[derive(Debug, Clone, Copy)]
pub enum NginxError {
    CannotReload,
    BadConfiguration,
    NotInstalled,
}

impl Error for NginxError {}

impl Display for NginxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            NginxError::CannotReload => write!(f, "Could not reload Nginx."),
            NginxError::BadConfiguration => write!(f, "Nginx -t failed. Bad Configuration."),
            NginxError::NotInstalled => write!(f, "Nginx was not found in /usr/sbin."),
        }
    }
}

impl<'a> WebServer<'a> for Nginx {
    const BINARY_NAME: &'a str = "nginx";
    const WEBSERVER_SBIN_PATH: &'a str = "/usr/sbin/nginx";
}

pub struct Nginx;

impl Nginx {
    pub fn reload() -> Result<(), NginxError> {
        Self::_reload(NginxError::CannotReload)
    }

    pub fn check() -> Result<(), NginxError> {
        let output = Command::new(Self::BINARY_NAME)
            .arg("-t")
            .spawn()
            .and_then(Child::wait_with_output)
            .map_err(|_| NginxError::BadConfiguration)?;

        if !output.status.success() {
            return Err(NginxError::BadConfiguration);
        }

        Ok(())
    }

    pub fn check_and_reload() -> Result<(), NginxError> {
        if !Self::is_installed() {
            return Err(NginxError::NotInstalled);
        }

        Self::check()?;
        Self::reload()?;

        Ok(())
    }
}
