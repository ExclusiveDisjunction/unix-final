use std::fmt::Display;

use serde::{Serialize, Deserialize};
use sea_query::*;

use super::book_org::{Author, BookGroup, Genre};

#[derive(PartialEq, Eq, Clone, Copy, Debug, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum BookRating {
    One = 1,
    Two = 2,
    Three = 3,
    Four = 4,
    Five = 5
}
impl TryFrom<i8> for BookRating {
    type Error = ();
    fn try_from(value: i8) -> Result<Self, Self::Error> {
        match value {
            1 => Ok( Self::One ),
            2 => Ok( Self::Two ),
            3 => Ok( Self::Three ),
            4 => Ok( Self::Four ),
            5 => Ok( Self::Five ),
            _ => Err( () )
        }
    }
}
impl From<BookRating> for i8 {
    fn from(value: BookRating) -> Self {
        value as i8
    }
}
impl Display for BookRating {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{} star{}",
            match self {
                Self::One => "One",
                Self::Two => "Two",
                Self::Three => "Three",
                Self::Four => "Four",
                Self::Five => "Five"
            },
            if matches!(self, Self::One) { 
                ""
            } else {
                "s"
            }
        )
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
#[enum_def]
pub struct RawBook {
    id: u32,
    title: String,
    description: String,
    rating: i8,
    is_favorite: bool
}

pub fn get_raw_books(conn: &mut postgres::Client) -> Result<Vec<RawBook>, postgres::Error> {
    let query = Query::select()
        .column(RawBookIden::Id)
        .column(RawBookIden::Title)
        .column(RawBookIden::Description)
        .column(RawBookIden::Rating)
        .column(RawBookIden::IsFavorite)
        .from(RawBookIden::Table)
        .build(PostgresQueryBuilder);

    let result = conn.query(&query.0, &[])?;
    let mut return_result = vec![];
    for row in result {
        let id: u32 = row.get(0);
        let title: &str = row.get(1);
        let desc: &str = row.get(2);
        let rating: i8 = row.get(3);
        let is_favorite: bool = row.get(4);

        return_result.push(
            RawBook {
                id,
                title: title.to_string(),
                description: desc.to_string(),
                rating,
                is_favorite
            }
        )
    }

    Ok( return_result )
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
struct Book {
    id: u32,
    title: String,
    description: String,
    rating: BookRating,
    is_favorite: bool,
    groups: Vec<BookGroup>,
    authors: Vec<Author>,
    genres: Vec<Genre>
}
