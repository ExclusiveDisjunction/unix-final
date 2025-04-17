use crate::{log_info, log_error, log_critical};

use crate::io::book::{DbBook, Book, get_all_db_data, activate_context};
use crate::io::book_org::{Author, DbGroup, Genre};
use crate::io::usr::DbUser;
use crate::io::db::{create_table, update};

use std::process::ExitCode;

use tokio_postgres::Client;

pub fn get_db_password() -> Result<String, std::env::VarError> {
    std::env::var("POSTGRES_PASSWORD")
}

pub async fn create_tables(conn: &mut tokio_postgres::Client) -> Result<(), postgres::Error> {
    log_info!("Running create scripts");
    create_table::<DbBook>     (conn).await?;
    create_table::<DbUser>     (conn).await?;
    create_table::<Genre>      (conn).await?;  
    create_table::<DbGroup>    (conn).await?;
    create_table::<Author>     (conn).await?;

    log_info!("Create successful.");

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

    let loaded_context = match get_all_db_data(conn).await {
        Ok(v) => v,
        Err(e) => {
            log_error!("Unable to open context '{e}'");
            return Err(ExitCode::FAILURE);
        }
    };

    let active = activate_context(loaded_context);
    println!("Database objects:");
    //println!("\t  Books  {}", active.books  .len());
    println!("\t  Users  {}", active.users  .len());
    //println!("\t Groups  {}", active.groups .len());
    println!("\t Genres  {}", active.genres .len());
    println!("\tAuthors  {}", active.authors.len());

    Ok( () )
}
pub async fn run(conn: &mut Client) -> Result<(), ExitCode> {
    validate(conn).await?;

    let new = Book::new(0, "Hello".to_string(), "Nothing to say".to_string(), 5, true, false);
    let new = vec![&new];
    if let Err(e) = update(conn, new).await {
        log_error!("Unable to update because '{e}'");
    }

    let mut signal = tokio::signal::unix::signal(tokio::signal::unix::SignalKind::interrupt()).unwrap();
    println!("Starting to wait...");
    let _ = signal.recv().await;

    println!("Kill signal received. Stopping.");

    Ok( () )
}