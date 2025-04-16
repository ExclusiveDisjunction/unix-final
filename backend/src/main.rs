use std::process::ExitCode;

pub mod error;
pub mod lock;
pub mod log;
pub mod msg;
pub mod net;
pub mod version;
pub mod loc;
pub mod usr;
pub mod auth;
pub mod book;
pub mod book_org;
pub mod db;

use log::{LoggerLevel, LoggerRedirect, LOG};
use loc::{make_log_path, PROG_NAME};

use clap::{Subcommand, Parser};
use version::CUR_VERSION;

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
    Validate,
    Info,
    Run
}

pub async fn validate() -> Result<(), ExitCode> {
    todo!() 
}
pub async fn info() -> Result<(), ExitCode> {
    validate().await?;

    Ok( () )
}
pub async fn run() -> Result<(), ExitCode> {
    validate().await?;

    Ok( () )
}

#[tokio::main]
async fn main() -> Result<(), ExitCode> {
    let command = Arguments::parse();

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

    match command.command {
        None | Some(Commands::Run) => run().await?,
        Some(Commands::Info) => info().await?,
        Some(Commands::Validate) => validate().await?
    }

    log_info!("Starting up {}, Version {}", PROG_NAME, CUR_VERSION);

    log_info!("All tasks completed successfully. Goodbye");
    Ok( () )
}
