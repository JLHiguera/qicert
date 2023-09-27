use std::{error::Error, fmt::Display, path::PathBuf, process::{Command, Child}};

#[derive(Debug)]
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

pub struct Nginx;

impl Nginx {
    const NGINX_BIN_PATH: &str = "/usr/sbin/nginx";

    pub fn reload() -> Result<(), NginxError> {
        let output = Command::new("systemctl")
            .arg("reload")
            .arg("nginx")
            .spawn()
            .and_then(Child::wait_with_output)
            .map_err(|_| NginxError::CannotReload)?;

        if !output.status.success() {
            return Err(NginxError::CannotReload);
        }

        Ok(())
    }

    pub fn check() -> Result<(), NginxError> {
        let output = Command::new("nginx")
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

    pub fn is_installed() -> bool {
        PathBuf::from(Self::NGINX_BIN_PATH).is_file()
    }
}
