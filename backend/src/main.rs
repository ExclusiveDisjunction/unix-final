use std::process::ExitCode;

pub mod io;
pub mod auth;
pub mod tool;
pub mod loc;

use io::book::{AuthorGroup, BookOwner, GenreGroup, GroupBinding};
use io::book_org::{Author, BookGroup, Genre};
use io::usr::RawUser;
use io::{book::Book, db::create_table_for};
use tokio_postgres::{Client, connect, NoTls};
use tool::log::{LoggerLevel, LoggerRedirect, LOG};
use loc::{make_log_path, PROG_NAME};

use clap::{Subcommand, Parser};
use tool::version::CUR_VERSION;

pub fn get_db_password() -> Result<String, std::env::VarError> {
    std::env::var("POSTGRES_PASSWORD")
}

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

pub async fn create_tables(conn: &mut tokio_postgres::Client) -> Result<(), postgres::Error> {
    create_table_for::<Book>        (conn).await?;
    create_table_for::<RawUser>     (conn).await?;
    create_table_for::<Genre>       (conn).await?;  
    create_table_for::<BookGroup>   (conn).await?;
    create_table_for::<Author>      (conn).await?;

    create_table_for::<GenreGroup>  (conn).await?;
    create_table_for::<BookOwner>   (conn).await?;
    create_table_for::<GroupBinding>(conn).await?;
    create_table_for::<AuthorGroup> (conn).await?;

    Ok( () )
}

pub async fn validate(conn: &mut Client) -> Result<(), ExitCode> {
    if let Err(e) = create_tables(conn).await {
        log_critical!("Unable to create database tables '{e}'");
        return Err(ExitCode::FAILURE);
    }

    Ok( () )
}
pub async fn info(conn: &mut Client) -> Result<(), ExitCode> {
    validate(conn).await?;

    todo!("Still in progess");

    //Ok( () )
}
pub async fn run(conn: &mut Client) -> Result<(), ExitCode> {
    validate(conn).await?;

    let mut signal = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::interrupt()).unwrap();
    println!("Starting to wait...");
    let _ = signal.recv().await;

    println!("Kill signal received. Stopping.");

    Ok( () )
}

#[tokio::main]
pub async fn main() -> Result<(), ExitCode> {
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

    log_info!("Starting up {}, Version {}", PROG_NAME, CUR_VERSION);

    log_info!("Attempting to start up connection.");
    log_info!("Obtaining password from env variable...");
    let password = match get_db_password() {
        Ok(v) => v,
        Err(e) => {
            log_critical!("Unable to get password '{e}'");
            return Err(ExitCode::FAILURE)
        }
    };

    let (mut client, conn) = match connect(&format!("host=localhost user=postgres password={password}"), NoTls).await {
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


    match command.command {
        None | Some(Commands::Run) => run(&mut client).await?,
        Some(Commands::Info) => info(&mut client).await?,
        Some(Commands::Validate) => validate(&mut client).await?
    }

    log_info!("All tasks completed successfully. Goodbye");
    Ok( () )
}
