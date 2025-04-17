use std::{fmt::Display, process::ExitCode};

pub mod io;
pub mod auth;
pub mod tool;
pub mod loc;
pub mod exec;

use tokio_postgres::{connect, Client, NoTls};
use tool::log::{LoggerLevel, LoggerRedirect, LOG};
use loc::{make_log_path, PROG_NAME};

use clap::{Subcommand, Parser};
use tool::version::CUR_VERSION;

#[derive(Parser, Debug)]
struct Arguments {
    #[command(subcommand)]
    command: Option<Commands>,
    #[arg(short, long, help="When this is active, the stdout will not include program status.")]
    quiet: bool,
    #[arg(short, long, help="When this is active, the erorrs will not be put in stderr")]
    no_error: bool,
    #[arg(short, long, help="All information, errors, and messages are displayed in stdout and stderr")]
    debug: bool
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Ensures that the database is operational
    Validate,
    /// Validates the database, and displays how many objects are stored in it.
    Info,
    /// Validates the database, and runs the app to begin serving requests.
    Run,
    /// Displays the app's current version
    Version
}
impl Display for Commands {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Validate => "Validate",
                Self::Info => "Info",
                Self::Run => "Run",
                Self::Version => "Version"
            }
        )
    }
}

fn load_logger(command: &Arguments) -> Result<(), ExitCode> {
    let log_path = make_log_path();
    let level: LoggerLevel;
    let redirect: LoggerRedirect;
    if command.debug || cfg!(debug_assertions) {
        println!("Debug mode activated.");
        level = LoggerLevel::Debug;
        redirect = LoggerRedirect::new(Some(LoggerLevel::Debug), true);
    }
    else if command.quiet {
        level = LoggerLevel::Info; 
        redirect = LoggerRedirect::new(None, !command.no_error);
    }
    else {
        level = LoggerLevel::Info;
        redirect = LoggerRedirect::new(Some(LoggerLevel::Info), !command.no_error);
    }
    
    if let Err(e) = LOG.open(log_path, level, redirect) {
        eprintln!("Unable to open log '{e}'.");
        return Err(ExitCode::FAILURE);
    }

    Ok( () )
}

async fn load_database() -> Result<Client, ExitCode> {
    log_info!("Attempting to start up connection.");
    log_info!("Obtaining password from env variable...");
    let password = match exec::get_db_password() {
        Ok(v) => v,
        Err(e) => {
            log_critical!("Unable to get password '{e}'");
            return Err(ExitCode::FAILURE)
        }
    };

    let (client, conn) = match connect(&format!("host=localhost user=postgres password={password}"), NoTls).await {
        Ok(v) => v,
        Err(e) => {
            log_critical!("Unable to open db {}", e);
            return Err(ExitCode::FAILURE)
        }
    };

    tokio::spawn(async move {
        if let Err(e) = conn.await {
            log_critical!("Connection failed '{e}'")
        }
    });

    Ok( client )
}

#[tokio::main]
pub async fn main() -> Result<(), ExitCode> {
    let command = Arguments::parse();

    load_logger(&command)?;

    log_info!("Starting up {}, Version {}", PROG_NAME, CUR_VERSION);

    let mut client = load_database().await?;

    let to_run: Commands;
    if let Some(command) = command.command {
        log_info!("Processing top level command '{}'", &command);
        to_run = command;
    }
    else {
        to_run = Commands::Run;
    }

    match to_run {
        Commands::Run => exec::run(&mut client).await?,
        Commands::Info => exec::info(&mut client).await?,
        Commands::Validate => exec::validate(&mut client).await?,
        Commands::Version => {
            println!("{}, Version {}", PROG_NAME, CUR_VERSION);
        }
    }

    log_info!("All tasks completed successfully. Goodbye");
    Ok( () )
}
