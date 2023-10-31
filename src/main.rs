#![cfg(unix)]

mod apache;
mod certer;
mod configuration_file;
mod domain;
mod nginx;
mod webroot;
mod webserver;
use std::error::Error;

use crate::domain::Domain;

use clap::{Parser, ValueEnum};

#[derive(Debug, Clone, Copy, ValueEnum)]
enum WebServers {
    Apache,
    Nginx,
}

#[derive(Parser)]
#[command(name = "qicert")]
#[command(author = "Jose Higuera <contact@higuera.dev>")]
#[command(about = "A very simple tool built as a wrapper on top of certbot 
    with nginx and manual certification in mind")]
#[command(version, long_about = None)]
struct Cli {
    #[arg(value_enum)]
    webserver: WebServers,
    #[arg(short = 'd', long)]
    domain: String,

    #[arg(short = 's', long)]
    subdomain: Option<String>,

    #[arg(short = 't', long)]
    tld: String,
}

fn main() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();

    let name = cli.domain;
    let tld = cli.tld;
    let subdomain = cli.subdomain;

    let domain = Domain::new(name, tld, subdomain.as_deref())?;

    match cli.webserver {
        WebServers::Apache => handle_apache(&domain)?,
        WebServers::Nginx => handle_nginx(&domain)?,
    }

    Ok(())
}

fn handle_apache(domain: &Domain) -> Result<(), Box<dyn Error>> {
    use apache::configurator::Configurator;
    Configurator::append_or_create(&domain)?;

    Ok(())
}

fn handle_nginx(domain: &Domain) -> Result<(), Box<dyn Error>> {
    use crate::nginx::configurator::Configurator;
    Configurator::append_or_create(&domain)?;

    Ok(())
}
