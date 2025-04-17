use std::{collections::HashMap, sync::{Arc, RwLock}};

use serde::{de, de::Visitor, ser::SerializeStruct, Deserialize, Serialize};
use sea_query::*;
use lazy_static::lazy_static;

use super::book_org::{Author, AuthorIden, DbGroup, DbGroupIden, Genre, GenreIden};
use super::usr::{DbUser, RawUserIden};
use super::db::{arc_wrap, select, DatabaseRepr, DatabaseInsertable, DatabaseQueryable, DatabaseTableCreatable, DatabaseUpdatable};
use crate::tool::lock::{RwProvider, OptionRwProvider, ProtectedAccess};

#[derive(Clone, Debug, PartialEq)]
#[enum_def]
pub struct DbBook {
    id: i32,
    group: i32,
    title: String,
    desc: String,
    rating: i16,
    is_favorite: bool,
    genre: Option<i32>,
    author: i32,  
    is_new: bool
}  
impl From<DbBook> for Book {
    fn from(value: DbBook) -> Self {
        Self {
            id: value.id,
            title: value.title,
            description: value.desc,
            rating: value.rating,
            is_favorite: value.is_favorite,
            groups: vec![],
            authors: vec![],
            genres: vec![]
        }
    }
}
impl DatabaseRepr for DbBook {
    type Identity = DbBookIden;

    fn is_new(&self) -> bool {
        self.is_new
    }
    
    fn all_columns() -> &'static [Self::Identity] {
        &[
            DbBookIden::Id,
            DbBookIden::Group,
            DbBookIden::Title,
            DbBookIden::Desc,
            DbBookIden::Rating,
            DbBookIden::IsFavorite,
            DbBookIden::Genre,
            DbBookIden::Author
        ]
    }
    fn table() -> Self::Identity {
        DbBookIden::Table
    }
}
impl DatabaseQueryable for DbBook {
    fn parse(row: postgres::Row) -> Self {
        let id: i32  = row.get(0);
        let group:       i32         = row.get(1);
        let title:       &str        = row.get(2);
        let desc:        &str        = row.get(3);
        let rating:      i16         = row.get(4);
        let is_favorite: bool        = row.get(5);
        let genre:       Option<i32> = row.get(6);
        let author:      i32         = row.get(7);
        
        Self {
            id,
            group,
            title: title.to_string(),
            desc: desc.to_string(),
            rating,
            is_favorite,
            genre,
            author,
            is_new: false
        }
    }
}
impl DatabaseTableCreatable for DbBook {
    fn create_columns(build: &mut TableCreateStatement) {
        use DbBookIden::*;
        build
            .col(
                ColumnDef::new(Id)
                    .integer()
                    .not_null()
                    .primary_key()
            )
            .col(
                ColumnDef::new(Group)
                    .integer()
                    .not_null()
            )
            .col(
                ColumnDef::new(Title)
                    .text()
                    .not_null()
            )
            .col(
                ColumnDef::new(Desc)
                    .text()
                    .not_null()
                    .default("")
            )
            .col(
                ColumnDef::new(Rating)
                    .small_integer()
                    .not_null()
                    .default(3)
            )
            .col(
                ColumnDef::new(IsFavorite)
                    .boolean()
                    .not_null()
                    .default(false)
            ).col(
                ColumnDef::new(Genre)
                    .integer()
            )
            .col(
                ColumnDef::new(Author)
                    .integer()
                    .not_null()
            ).foreign_key(
                ForeignKeyCreateStatement::new()
                    .to(DbGroupIden::Table, DbGroupIden::Id)
                    .from(Table, Group)
            ).foreign_key(
                ForeignKeyCreateStatement::new()
                    .to(GenreIden::Table, GenreIden::Id)
                    .from(Table, Genre)
            ).foreign_key(
                ForeignKeyCreateStatement::new()
                    .to(AuthorIden::Table, AuthorIden::Id)
                    .from(Table, Author)
            );
    }
}
impl DatabaseInsertable for DbBook {
    fn insert_values(&self) -> Vec<sea_query::SimpleExpr> {
        vec![
            self.id.into(),
            self.group.into(),
            (&self.title).into(),
            (&self.desc).into(),
            self.rating.into(),
            self.is_favorite.into(),
            self.genre.into(),
            self.author.into()
        ]
    }
}
impl DatabaseUpdatable for DbBook {
    fn id_col() -> Self::Identity {
        DbBookIden::Id
    }
    fn id_val(&self) -> SimpleExpr {
        self.id.into()
    }
}

#[derive(Deserialize, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all="lowercase")]
pub enum BookFields{
    Id,
    Title,
    Description,
    Rating,
    IsFavorite,
    Author,
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Book {
    id: i32,
    title: String,
    description: String,
    rating: i16,
    is_favorite: bool,
    author: Option<Arc<Author>> //Type Author?
}

impl Serialize for Book {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        let mut s = serializer.serialize_struct("Book", 6)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("title", &self.title)?;
        s.serialize_field("description", &self.description)?;
        s.serialize_field("rating", &self.rating)?;
        s.serialize_field("is_favorite", &self.is_favorite)?;

        s.serialize_field("author", &self.author.as_deref())?;

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
            "genres",
            "owners"
        ];

        deserializer.deserialize_struct("Book", FIELDS, BookVisitor)
    }
}
impl Book {
    pub fn new(id: i32, title: String, description: String, rating: i16, is_favorite: bool, is_new: bool) -> Self {
        Self {
            id,
            title,
            description,
            rating,
            is_favorite,
            is_new,
            groups: vec![],
            authors: vec![],
            genres: vec![]
        }
    }
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
        let rating = seq.next_element()?
            .ok_or_else(|| de::Error::invalid_length(3, &self))?;
        let is_favorite = seq.next_element()?
            .ok_or_else(|| de::Error::invalid_length(4, &self))?;
        let author = seq.next_element()?
            .ok_or_else(|| de::Error::invalid_length(5, &self))?;

        let t_author: Option<Arc<Author>>;
        if let Some(author) = author {
            t_author = Some(Arc::new(author));
        }
        else {
            t_author = None;
        }
        
        Ok(
            Book {
                id,
                title,
                description: desc,
                rating,
                is_favorite,
                author: t_author
            }
        )

    }
    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::MapAccess<'de>, {
        let mut id = None;
        let mut title = None;
        let mut desc = None;
        let mut rating = None;
        let mut is_favorite = None;
        let mut author = None;

        while let Some(key) = map.next_key()? {
            match key {
                BookFields::Id          => Self::get_or_fail(map.next_value()?, "id",          &mut id         )?,
                BookFields::Title       => Self::get_or_fail(map.next_value()?, "title",       &mut title      )?,
                BookFields::Description => Self::get_or_fail(map.next_value()?, "description", &mut desc       )?,
                BookFields::Rating      => Self::get_or_fail(map.next_value()?, "rating",      &mut rating     )?,
                BookFields::IsFavorite  => Self::get_or_fail(map.next_value()?, "is_favorite", &mut is_favorite)?,
                BookFields::Author      => Self::get_or_fail(map.next_value()?, "author",      &mut author     )?,
            }
        }

        let id                = id         .ok_or_else(|| de::Error::missing_field("id")          )?;
        let title          = title      .ok_or_else(|| de::Error::missing_field("title")       )?;
        let desc           = desc       .ok_or_else(|| de::Error::missing_field("description") )?;
        let rating            = rating     .ok_or_else(|| de::Error::missing_field("rating")      )?;
        let is_favorite      = is_favorite.ok_or_else(|| de::Error::missing_field("is_favorite") )?;
        let author = author     .ok_or_else(|| de::Error::missing_field("author")     )?;

        let t_author: Option<Arc<Author>>;
        if let Some(author) = author {
            t_author = Some(Arc::new(author));
        }
        else {
            t_author = None;
        }

        Ok(
            Book {
                id,
                title,
                description: desc,
                rating,
                is_favorite,
                author: t_author
            }
        )
    }
}

pub struct LoadedContext {
    books: Vec<DbBook>,
    groups: Vec<DbGroup>,
    users: Vec<DbUser>,
    genres: Vec<Genre>,
    authors: Vec<Author>
}

pub struct ActiveContext {
    pub users: Vec<User>,
    pub auth_user: Vec<AuthUser>,
    pub genres: Vec<Arc<Genre>>,
    pub authors: Vec<Arc<Author>>
}
impl ActiveContext {
   
}

pub async fn get_all_db_data(conn: &mut tokio_postgres::Client) -> Result<LoadedContext, postgres::Error> {
    let books:   Vec<DbBook>    = select(conn).await?;
    let groups:  Vec<DbGroup>   = select(conn).await?;
    let genres:  Vec<Genre>     = select(conn).await?;
    let authors: Vec<Author>    = select(conn).await?;
    let users:   Vec<DbUser>    = select(conn).await?;

    Ok(
        LoadedContext {
            books,
            groups,
            users,
            genres,
            authors
        }
    )
}

pub fn activate_context(loaded: LoadedContext) -> ActiveContext {
    let mut books: HashMap<i32, Book> = HashMap::new();
    for book in loaded.books {
        books.insert(book.id, book);
    }

    let mut groups: HashMap<String, Arc<BookGroup>> = HashMap::new();
    for group in loaded.groups {
        groups.insert(group.name.clone(), Arc::new(group));
    }
    
    let mut genres: HashMap<String, Arc<Genre>> = HashMap::new();
    for genre in loaded.genres {
        genres.insert(genre.name.clone(), Arc::new(genre));
    }
    
    let mut authors: HashMap<i32, Arc<Author>> = HashMap::new();
    for author in loaded.authors {
        authors.insert(author.id, Arc::new(author));
    }

    let mut users: HashMap<String, Arc<DbUser>> = HashMap::new();
    for user in loaded.users {
        users.insert(user.username.clone(), Arc::new(user));
    }

    for pair in loaded.group_bindings {
        match (books.get_mut(&pair.book_id), groups.get(&pair.group_name)) {
            (Some(book), Some(group)) => book.groups.push(Arc::clone(group)),
            _ => continue
        }
    }

    for pair in loaded.genre_groups {
        match (books.get_mut(&pair.book_id), genres.get(&pair.genre_name)) {
            (Some(book), Some(v)) => book.genres.push(Arc::clone(v)),
            _ => continue
        }
    }

    for pair in loaded.author_groups {
        match (books.get_mut(&pair.book_id), authors.get(&pair.author_id)) {
            (Some(book), Some(v)) => book.authors.push(Arc::clone(v)),
            _ => continue
        }
    }

    /*
    for pair in loaded.book_owners {
        match (books.get_mut(&pair.book_id), users.get(&pair.username)) {
            (Some(book), Some(v)) => book.owners.push(Arc::clone(v)),
            _ => continue
        }
    }
    */

    ActiveContext {
        books: books    .into_values().map(Arc::new).collect(),
        groups: groups  .into_values().collect(),
        genres: genres  .into_values().collect(),
        authors: authors.into_values().collect(),
        users: users    .into_values().collect()
    }
}

pub struct ContextProvider {
    data: Arc<RwLock<Option<ActiveContext>>>
}
impl Default for ContextProvider {
    fn default() -> Self {
        Self {
            data: Arc::new(RwLock::new(None))
        }
    }
}
impl RwProvider for ContextProvider {
    type Data = Option<ActiveContext>;
    fn access_raw(&self) -> ProtectedAccess<'_, Arc<RwLock<Self::Data>>> {
        ProtectedAccess::new(&self.data)
    }
}
impl OptionRwProvider<ActiveContext> for ContextProvider { }

lazy_static! {
    pub static ref CONTEXT: ContextProvider = ContextProvider::default();
}