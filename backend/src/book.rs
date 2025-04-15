use std::fmt::Display;
use std::sync::Arc;

use serde::{de, de::Visitor, ser::SerializeStruct, Deserialize, Serialize};
use sea_query::*;

use crate::book_org::arc_wrap;

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
impl RawBook {
    pub fn as_ref(&self) -> RefRawBook<'_> {
        RefRawBook {
            id: self.id,
            title: &self.title,
            description: &self.description,
            rating: self.rating,
            is_favorite: self.is_favorite
        }
    }
}

#[derive(Serialize, Deserialize, Clone, Debug, PartialEq, Eq, Hash)]
pub struct RefRawBook<'a> {
    id: u32,
    title: &'a str,
    description: &'a str,
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

#[derive(Deserialize)]
#[serde(rename_all="lowercase")]
enum BookFields {
    Id,
    Title,
    Description,
    Rating,
    IsFavorite,
    Groups,
    Authors,
    Genres
}

struct BookVisitor;
impl BookVisitor {
    fn get_or_fail<A, E>(v: A, name: &'static str, dest: &mut Option<A>) -> Result<(), E> where E: de::Error {
        if dest.is_some() {
            return Err(de::Error::duplicate_field(name))
        }

        *dest = Some(v);
        Ok( () )
    } 
}
impl<'de> Visitor<'de> for BookVisitor {
    type Value = Book;

    fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        formatter.write_str("struct Book")
    }

    fn visit_seq<A>(self, mut seq: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::SeqAccess<'de>, {
        let id = seq.next_element()?
                .ok_or_else(|| de::Error::invalid_length(0, &self))?;
        let title = seq.next_element()?
            .ok_or_else(|| de::Error::invalid_length(1, &self))?;
        let desc = seq.next_element()?
            .ok_or_else(|| de::Error::invalid_length(2, &self))?;
        let rating: i8 = seq.next_element()?
            .ok_or_else(|| de::Error::invalid_length(3, &self))?;
        let is_favorite = seq.next_element()?
            .ok_or_else(|| de::Error::invalid_length(4, &self))?;
        let groups = seq.next_element()?
            .ok_or_else(|| de::Error::invalid_length(5, &self))?;
        let authors = seq.next_element()?
            .ok_or_else(|| de::Error::invalid_length(6, &self))?;
        let genres = seq.next_element()?
            .ok_or_else(|| de::Error::invalid_length(7, &self))?;

        let groups = arc_wrap(groups);
        let authors = arc_wrap(authors);
        let genres = arc_wrap(genres);

        let rating = BookRating::try_from(rating)
            .map_err(|_x| de::Error::custom("the rating is invalid"))?;
        
        Ok(
            Book {
                id,
                title,
                description: desc,
                rating,
                is_favorite,
                groups,
                authors,
                genres
            }
        )

    }
    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::MapAccess<'de>, {
        let mut id = None;
        let mut title = None;
        let mut desc = None;
        let mut rating: Option<i8> = None;
        let mut is_favorite = None;
        let mut groups = None;
        let mut authors = None;
        let mut genres = None;


        while let Some(key) = map.next_key()? {
            match key {
                BookFields::Id =>          Self::get_or_fail(map.next_value()?, "id",          &mut id         )?,
                BookFields::Title =>       Self::get_or_fail(map.next_value()?, "title",       &mut title      )?,
                BookFields::Description => Self::get_or_fail(map.next_value()?, "description", &mut desc       )?,
                BookFields::Rating =>      Self::get_or_fail(map.next_value()?, "rating",      &mut rating     )?,
                BookFields::IsFavorite =>  Self::get_or_fail(map.next_value()?, "is_favorite", &mut is_favorite)?,
                BookFields::Groups =>      Self::get_or_fail(map.next_value()?, "groups",      &mut groups     )?,
                BookFields::Authors =>     Self::get_or_fail(map.next_value()?, "authors",     &mut authors    )?,
                BookFields::Genres =>      Self::get_or_fail(map.next_value()?, "genres",      &mut genres     )?,
            }
        }

        let id = id.ok_or_else(|| de::Error::missing_field("id"))?;
        let title = title.ok_or_else(|| de::Error::missing_field("title"))?;
        let desc = desc.ok_or_else(|| de::Error::missing_field("description"))?;
        let rating = rating.ok_or_else(|| de::Error::missing_field("rating"))?;
        let is_favorite = is_favorite.ok_or_else(|| de::Error::missing_field("is_favorite"))?;
        let groups = groups.ok_or_else(|| de::Error::missing_field("groups"))?;
        let authors = authors.ok_or_else(|| de::Error::missing_field("authors"))?;
        let genres = genres.ok_or_else(|| de::Error::missing_field("genres"))?;

        let rating = BookRating::try_from(rating)
            .map_err(|_x| de::Error::custom("the rating is invalid"))?;

        let groups = arc_wrap(groups);
        let authors = arc_wrap(authors);
        let genres = arc_wrap(genres);

        Ok(
            Book {
                id,
                title,
                description: desc,
                rating,
                is_favorite,
                groups,
                authors,
                genres
            }
        )
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
struct Book {
    id: u32, //Type: unsigned int
    title: String,
    description: String,
    rating: BookRating, //Type: short Int
    is_favorite: bool,
    groups: Vec<Arc<BookGroup>>, //Type: BookGroup
    authors: Vec<Arc<Author>>, //Type: Author
    genres: Vec<Arc<Genre>> //Type: Genre
}
impl Serialize for Book {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        let mut s = serializer.serialize_struct("Book", 8)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("title", &self.title)?;
        s.serialize_field("description", &self.description)?;
        s.serialize_field("rating", &(self.rating as i8))?;
        s.serialize_field("is_favorite", &self.is_favorite)?;

        let groups: Vec<&BookGroup> = self.groups.iter().map(|x| &*(*x)).collect();
        let authors: Vec<&Author> = self.authors.iter().map(|x| &*(*x)).collect();
        let genres: Vec<&Genre> = self.genres.iter().map(|x| &*(*x)).collect();
        s.serialize_field("groups", &groups)?;
        s.serialize_field("authors", &authors)?;
        s.serialize_field("genres", &genres)?;

        s.end()
    }
}
impl<'de> Deserialize<'de> for Book {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
        where
            D: serde::Deserializer<'de> {
        const FIELDS: &[&str] = &[
            "id",
            "title", 
            "description", 
            "rating", 
            "is_favorite", 
            "groups", 
            "authors", 
            "genres"
        ];

        deserializer.deserialize_struct("Book", FIELDS, BookVisitor)
    }
}
