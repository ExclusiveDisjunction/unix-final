use serde::{Serialize, Deserialize};

use sea_query::*;

use super::db::DatabaseCallable;

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Debug)]
#[enum_def]
pub struct BookGroup {
    pub name: String,
    pub desc: String
}
impl DatabaseCallable for BookGroup {
    type Identity = BookGroupIden;
    fn all_columns() -> &'static [Self::Identity] {
        &[
            BookGroupIden::Name,
            BookGroupIden::Desc
        ]
    }
    fn table() -> Self::Identity {
        BookGroupIden::Table
    }
    fn parse(row: postgres::Row) -> Self {
        let name: &str = row.get(0);
        let desc: &str = row.get(1);

        Self {
            name: name.to_string(),
            desc: desc.to_string()
        }
    }
    fn create_columns(build: &mut TableCreateStatement) {
        build
            .col(
                ColumnDef::new(BookGroupIden::Name)
                    .not_null()
                    .primary_key()
                )
            .col(
                ColumnDef::new(BookGroupIden::Desc)
                    .text()
                    .not_null()
                );
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Debug)]
#[enum_def]
pub struct Genre {
    pub name: String,
    pub desc: String,
    pub age_group: i8
}
impl DatabaseCallable for Genre {
    type Identity = GenreIden;
    fn all_columns() -> &'static [Self::Identity] {
        &[
            GenreIden::Name,
            GenreIden::Desc,
            GenreIden::AgeGroup
        ]
    }
    fn table() -> Self::Identity {
        GenreIden::Table
    }
    fn parse(row: postgres::Row) -> Self {
        let name: &str = row.get(0);
        let desc: &str = row.get(1);
        let age_group: i8 = row.get(2);

        Self {
            name: name.to_string(),
            desc: desc.to_string(),
            age_group
        }
    }
    fn create_columns(build: &mut TableCreateStatement) {
        build
            .col(
                ColumnDef::new(GenreIden::Name)
                    .text()
                    .primary_key()
            )
            .col(
                ColumnDef::new(GenreIden::Desc)
                    .text()
                    .default("")
                    .not_null()
            )
            .col(
                ColumnDef::new(GenreIden::AgeGroup)
                    .small_integer()
                    .not_null()
            );
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Debug)]
#[enum_def]
pub struct Author {
    pub id: u32,
    pub first_name: String,
    pub last_name: String,
    pub rating: i8,
    pub comment: String
}  
impl DatabaseCallable for Author {
    type Identity = AuthorIden;
    fn all_columns() -> &'static [Self::Identity] {
        &[
            AuthorIden::Id,
            AuthorIden::FirstName,
            AuthorIden::LastName,
            AuthorIden::Rating,
            AuthorIden::Comment
        ]
    }
    fn table() -> Self::Identity {
        AuthorIden::Table
    }
    fn parse(row: postgres::Row) -> Self {
        let id: u32 = row.get(0);
        let first_name: &str = row.get(1);
        let last_name: &str = row.get(2);
        let rating: i8 = row.get(3);
        let comment: &str = row.get(4);

        Self {
            id,
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            rating,
            comment: comment.to_string()
        }
    }
    fn create_columns(build: &mut TableCreateStatement) {
        build
            .col(
                ColumnDef::new(AuthorIden::Id)
                    .unsigned()
                    .primary_key()
            )
            .col(
                ColumnDef::new(AuthorIden::FirstName)
                    .text()
                    .not_null()
            )
            .col(
                ColumnDef::new(AuthorIden::LastName)
                    .text()
                    .not_null()
            )
            .col(
                ColumnDef::new(AuthorIden::Rating)
                    .small_integer()
                    .not_null()
            )
            .col(
                ColumnDef::new(AuthorIden::Comment)
                    .text()
                    .default("")
                    .not_null()
            );
            //.unique_key([AuthorIden::FirstName, AuthorIden::LastName])
    }
}