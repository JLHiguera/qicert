use std::{error::Error, path::PathBuf, process::{Child, Command}};

pub(crate) trait WebServer<'a> {
    const WEBSERVER_SBIN_PATH: &'a str;
    const BINARY_NAME: &'a str;

    fn _reload<E: Error + Copy>(reload_err: E) -> Result<(), E> {
        let output = Command::new("systemctl")
            .arg("reload")
            .arg(Self::BINARY_NAME)
            .spawn()
            .and_then(Child::wait_with_output)
            .map_err(|_| reload_err)?;

        if !output.status.success() {
            return Err(reload_err);
        }

        Ok(())
    }

    fn is_installed() -> bool {
        PathBuf::from(Self::WEBSERVER_SBIN_PATH).is_file()
    }
}