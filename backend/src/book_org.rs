use serde::{Serialize, Deserialize};
use std::fmt::Display;

use sea_query::*;

use std::sync::Arc;

use crate::book::BookRating;

#[derive(PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum AgeGroup {
    Child,
    Teenager,
    YoungAdult,
    Adult
}
impl Display for AgeGroup {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Child => "Child",
                Self::Teenager => "Teenager",
                Self::YoungAdult => "Young Adult",
                Self::Adult => "Adult"
            }
        )
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Debug)]
#[enum_def]
pub struct BookGroup {
    name: String,
    desc: String
}

pub fn get_all_book_groups(conn: &mut postgres::Client) -> Result<Vec<BookGroup>, postgres::Error> {
    let query = Query::select()
        .column(BookGroupIden::Name)
        .column(BookGroupIden::Desc)
        .from(BookGroupIden::Table)
        .build(PostgresQueryBuilder);

    let result = conn.query(&query.0, &[])?;
    let mut return_result = vec![];
    for row in result {
        let name: &str = row.get(0);
        let desc: &str = row.get(1);

        return_result.push(
            BookGroup {
                name: name.to_string(),
                desc: desc.to_string()
            }
        )
    }

    Ok( return_result )
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Debug)]
pub struct Genre {
    name: String,
    desc: String,
    age_group: AgeGroup
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[enum_def]
pub struct RawGenre {
    name: String,
    desc: String,
    age_group: i8
}

pub fn get_all_raw_genre(conn: &mut postgres::Client) -> Result<Vec<RawGenre>, postgres::Error> {
    let query = Query::select()
        .column(RawGenreIden::Name)
        .column(RawGenreIden::Desc)
        .column(RawGenreIden::AgeGroup)
        .from(RawGenreIden::Table)
        .build(PostgresQueryBuilder);

    let result = conn.query(&query.0, &[])?;
    let mut return_result = vec![];
    for row in result {
        let name: &str = row.get(0);
        let desc: &str = row.get(1);
        let age_group: i8 = row.get(2);

        return_result.push(
            RawGenre {
                name: name.to_string(),
                desc: desc.to_string(),
                age_group
            }
        )
    }

    Ok( return_result )
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Debug)]
pub struct Author {
    id: u32,
    first_name: String,
    last_name: String,
    rating: BookRating,
    comment: String
}  

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[enum_def]
pub struct RawAuthor {
    id: u32,
    first_name: String,
    last_name: String,
    rating: i8,
    comment: String
}
impl TryFrom<RawAuthor> for Author {
    type Error = <BookRating as TryFrom<i8>>::Error;
    fn try_from(value: RawAuthor) -> Result<Self, Self::Error> {
        Ok(
            Author {
                id: value.id,
                first_name: value.first_name,
                last_name: value.last_name,
                rating: value.rating.try_into()?,
                comment: value.comment
            }
        )
    }
}
impl From<Author> for RawAuthor {
    fn from(value: Author) -> Self {
        Self {
            id: value.id,
            first_name: value.first_name,
            last_name: value.last_name,
            rating: value.rating.into(),
            comment: value.comment
        }
    }
}

pub fn get_all_raw_authors(conn: &mut postgres::Client) -> Result<Vec<RawAuthor>, postgres::Error> {
    let query = Query::select()
        .column(RawAuthorIden::Id)
        .column(RawAuthorIden::FirstName)
        .column(RawAuthorIden::LastName)
        .column(RawAuthorIden::Rating)
        .column(RawAuthorIden::Comment)
        .from(RawAuthorIden::Table)
        .build(PostgresQueryBuilder);

    let result = conn.query(&query.0, &[])?;
    let mut return_result = vec![];
    for row in result {
        let id: u32 = row.get(0);
        let first_name: &str = row.get(1);
        let last_name: &str = row.get(2);
        let rating: i8 = row.get(3);
        let comment: &str = row.get(4);

        return_result.push(
            RawAuthor {
                id,
                first_name: first_name.to_string(),
                last_name: last_name.to_string(),
                rating,
                comment: comment.to_string()
            }
        )
    }

    Ok( return_result )
}

pub fn parse_raw<T, V>(values: Vec<T>) -> (Vec<V>, Vec<<T as TryInto<V>>::Error>) where T: TryInto<V> {
    let mut result = vec![];
    let mut errors = vec![];

    for value in values {
        match value.try_into() {
            Ok(user) => result.push(user),
            Err(e) => errors.push(e)
        }
    }

    (result, errors)
}

pub fn arc_wrap<T>(vals: Vec<T>) -> Vec<Arc<T>> {
    vals.into_iter().map(Arc::new).collect()
}