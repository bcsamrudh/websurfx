//! Main module of the application
//!
//! This module contains the main function which handles the logging of the application to the
//! stdout and handles the command line arguments provided and launches the `websurfx` server.

use std::net::TcpListener;
use websurfx::{config::parser::Config, run};

/// The function that launches the main server and registers all the routes of the website.
///
/// # Error
///
/// Returns an error if the port is being used by something else on the system and is not
/// available for being used for other applications.
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize the parsed config file.
    let config = Config::parse(true).unwrap();

    log::info!(
        "started server on port {} and IP {}",
        config.port,
        config.binding_ip
    );
    log::info!(
        "Open http://{}:{}/ in your browser",
        config.binding_ip,
        config.port,
    );

    let listener = TcpListener::bind((config.binding_ip.clone(), config.port))?;

    run(listener, config)?.await
}
