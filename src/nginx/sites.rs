use std::path::PathBuf;

pub struct Sites;

impl Sites {
    const SITES_AVAILABLE: &str = "/etc/nginx/sites-available";
    const SITES_ENABLED: &str = "/etc/nginx/sites-enabled";

    pub fn paths() -> (PathBuf, PathBuf) {
        let sites_available = PathBuf::from(Self::SITES_AVAILABLE);
        let sites_enabled = PathBuf::from(Self::SITES_ENABLED);

        (sites_available, sites_enabled)
    }
}
