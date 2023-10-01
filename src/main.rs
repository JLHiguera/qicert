#![cfg(unix)]

mod certer;
mod domain;
mod nginx;
mod webroot;
mod apache;

use std::error::Error;

use crate::nginx::configurator::Configurator;
use crate::domain::Domain;

use clap::Parser;

#[derive(Parser)]
#[command(name = "qicert")]
#[command(author = "Jose Higuera <contact@higuera.dev>")]
#[command(about = "A very simple tool built as a wrapper on top of certbot 
    with nginx and manual certification in mind")]
#[command(version, long_about = None)]
struct Cli {
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

    Configurator::append_or_create(&domain)?;

    Ok(())
}
