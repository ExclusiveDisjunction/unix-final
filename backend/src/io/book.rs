use std::{collections::HashMap, sync::{Arc, RwLock}};

use serde::{de, de::Visitor, ser::SerializeStruct, Deserialize, Serialize};
use sea_query::*;
use lazy_static::lazy_static;

use super::book_org::{Author, AuthorIden, BookGroup, BookGroupIden, Genre, GenreIden};
use super::usr::{RawUser, RawUserIden};
use super::db::{arc_wrap, get_from_db, DatabaseCallable};
use crate::tool::lock::{RwProvider, OptionRwProvider, ProtectedAccess};

#[derive(Deserialize, Iden, Clone, Copy, Debug, PartialEq, Eq)]
#[serde(rename_all="lowercase")]
pub enum BookIden {
    Id,
    Title,
    Description,
    Rating,
    IsFavorite,
    Groups,
    Authors,
    Genres,
    Table
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Book {
    id: u32,
    title: String,
    description: String,
    rating: i8,
    is_favorite: bool,
    groups: Vec<Arc<BookGroup>>, //Type: BookGroup
    authors: Vec<Arc<Author>>, //Type: Author
    genres: Vec<Arc<Genre>> //Type: Genre
}
impl DatabaseCallable for Book {
    type Identity = BookIden;
    fn all_columns() -> &'static [Self::Identity] {
        &[
            BookIden::Id,
            BookIden::Title,
            BookIden::Description,
            BookIden::Rating,
            BookIden::IsFavorite
        ]
    }
    fn table() -> Self::Identity {
        BookIden::Table
    }
    fn parse(row: postgres::Row) -> Self {
        let id: u32 = row.get(0);
        let title: &str = row.get(1);
        let description: &str = row.get(2);
        let rating: i8 = row.get(3);
        let is_favorite: bool = row.get(4);
        
        Self {
            id,
            title: title.to_string(),
            description: description.to_string(),
            rating,
            is_favorite,
            groups: vec![],
            authors: vec![],
            genres: vec![]
        }
    }
    fn create_columns(build: &mut TableCreateStatement) {
        use BookIden::*;
        build
            .col(
                ColumnDef::new(Id)
                    .unsigned()
                    .not_null()
                    .primary_key()
            )
            .col(
                ColumnDef::new(Title)
                    .text()
                    .not_null()
            )
            .col(
                ColumnDef::new(Description)
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
            );
    }
}
impl Serialize for Book {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where
            S: serde::Serializer {
        let mut s = serializer.serialize_struct("Book", 8)?;
        s.serialize_field("id", &self.id)?;
        s.serialize_field("title", &self.title)?;
        s.serialize_field("description", &self.description)?;
        s.serialize_field("rating", &self.rating)?;
        s.serialize_field("is_favorite", &self.is_favorite)?;

        let groups: Vec<&BookGroup> = self.groups .iter().map(|x| x.as_ref()).collect();
        let authors: Vec<&Author>   = self.authors.iter().map(|x|    x.as_ref()).collect();
        let genres: Vec<&Genre>     = self.genres .iter().map(|x|     x.as_ref()).collect();
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
            "genres",
            "owners"
        ];

        deserializer.deserialize_struct("Book", FIELDS, BookVisitor)
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
        let groups = seq.next_element()?
            .ok_or_else(|| de::Error::invalid_length(5, &self))?;
        let authors = seq.next_element()?
            .ok_or_else(|| de::Error::invalid_length(6, &self))?;
        let genres = seq.next_element()?
            .ok_or_else(|| de::Error::invalid_length(7, &self))?;

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
    fn visit_map<A>(self, mut map: A) -> Result<Self::Value, A::Error>
        where
            A: serde::de::MapAccess<'de>, {
        let mut id = None;
        let mut title = None;
        let mut desc = None;
        let mut rating = None;
        let mut is_favorite = None;
        let mut groups = None;
        let mut authors = None;
        let mut genres = None;

        while let Some(key) = map.next_key()? {
            match key {
                BookIden::Id          => Self::get_or_fail(map.next_value()?, "id",          &mut id         )?,
                BookIden::Title       => Self::get_or_fail(map.next_value()?, "title",       &mut title      )?,
                BookIden::Description => Self::get_or_fail(map.next_value()?, "description", &mut desc       )?,
                BookIden::Rating      => Self::get_or_fail(map.next_value()?, "rating",      &mut rating     )?,
                BookIden::IsFavorite  => Self::get_or_fail(map.next_value()?, "is_favorite", &mut is_favorite)?,
                BookIden::Groups      => Self::get_or_fail(map.next_value()?, "groups",      &mut groups     )?,
                BookIden::Authors     => Self::get_or_fail(map.next_value()?, "authors",     &mut authors    )?,
                BookIden::Genres      => Self::get_or_fail(map.next_value()?, "genres",      &mut genres     )?,
                BookIden::Table       => continue
            }
        }

        let id                = id         .ok_or_else(|| de::Error::missing_field("id")          )?;
        let title          = title      .ok_or_else(|| de::Error::missing_field("title")       )?;
        let desc           = desc       .ok_or_else(|| de::Error::missing_field("description") )?;
        let rating             = rating     .ok_or_else(|| de::Error::missing_field("rating")      )?;
        let is_favorite      = is_favorite.ok_or_else(|| de::Error::missing_field("is_favorite") )?;
        let groups = groups     .ok_or_else(|| de::Error::missing_field("groups")      )?;
        let authors   = authors    .ok_or_else(|| de::Error::missing_field("authors")     )?;
        let genres     = genres     .ok_or_else(|| de::Error::missing_field("genres")      )?;

        let groups = arc_wrap(groups );
        let authors   = arc_wrap(authors);
        let genres     = arc_wrap(genres );

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
#[enum_def]
pub struct GenreGroup {
    book_id: u32,
    genre_name: String
}
impl DatabaseCallable for GenreGroup {
    type Identity = GenreGroupIden;
    fn all_columns() -> &'static [Self::Identity] {
        &[
            GenreGroupIden::BookId,
            GenreGroupIden::GenreName
        ]
    }
    fn table() -> Self::Identity {
        GenreGroupIden::Table
    }
    fn parse(row: postgres::Row) -> Self {
        let book_id: u32 = row.get(0);
        let genre_name: &str = row.get(1);

        Self {
            book_id,
            genre_name: genre_name.to_string()
        }
    }
    fn create_columns(build: &mut TableCreateStatement) {
        build
            .col(
            ColumnDef::new(GenreGroupIden::BookId)
                .unsigned()
                .not_null()
            )
            .col(
                ColumnDef::new(GenreGroupIden::GenreName)
                .text()
                .not_null()
            )
            .primary_key(
                IndexCreateStatement::new()
                    .col(GenreGroupIden::BookId)
                    .col(GenreGroupIden::GenreName)
            )
            .foreign_key(
                ForeignKeyCreateStatement::new()
                    .from(GenreIden::Table, GenreIden::Name)
                    .to(GenreGroupIden::Table, GenreGroupIden::GenreName)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
            )
            .foreign_key(
                ForeignKeyCreateStatement::new()
                    .from(BookIden::Table, BookIden::Id)
                    .to(GenreGroupIden::Table, GenreGroupIden::BookId)
                    .on_delete(ForeignKeyAction::Cascade)
                    .on_update(ForeignKeyAction::Cascade)
            );
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[enum_def]
pub struct GroupBinding {
    book_id: u32,
    group_name: String
}
impl DatabaseCallable for GroupBinding {
    type Identity = GroupBindingIden;
    fn all_columns() -> &'static [Self::Identity] {
        &[
            GroupBindingIden::BookId,
            GroupBindingIden::GroupName
        ]
    }
    fn table() -> Self::Identity {
        GroupBindingIden::Table
    }
    fn parse(row: postgres::Row) -> Self {
        let book_id: u32 = row.get(0);
        let group_name: &str = row.get(1);

        Self {
            book_id,
            group_name: group_name.to_string()
        }
    }
    fn create_columns(build: &mut TableCreateStatement) {
        build
            .col(
                ColumnDef::new(GroupBindingIden::BookId)
                    .unsigned()
                    .not_null()
            )
            .col(
                ColumnDef::new(GroupBindingIden::GroupName)
                    .text()
                    .not_null()
            )
            .primary_key(
                IndexCreateStatement::new()
                    .col(GroupBindingIden::BookId)
                    .col(GroupBindingIden::GroupName)
            )
            .foreign_key(
                ForeignKeyCreateStatement::new()
                    .from(BookGroupIden::Table, BookGroupIden::Name)
                    .from(GroupBindingIden::Table, GroupBindingIden::GroupName)
            )
            .foreign_key(
                ForeignKeyCreateStatement::new()
                    .from(BookIden::Table, BookIden::Id)
                    .from(GroupBindingIden::Table, GroupBindingIden::BookId)
            );
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[enum_def]
pub struct AuthorGroup {
    book_id: u32,
    author_id: u32
}
impl DatabaseCallable for AuthorGroup {
    type Identity = AuthorGroupIden;
    fn all_columns() -> &'static [Self::Identity] {
        &[
            AuthorGroupIden::BookId,
            AuthorGroupIden::AuthorId
        ]
    }
    fn table() -> Self::Identity {
        AuthorGroupIden::Table
    }
    fn parse(row: postgres::Row) -> Self {
        let book_id: u32 = row.get(0);
        let author_id: u32 = row.get(1);

        Self {
            book_id,
            author_id
        }
    }
    fn create_columns(build: &mut TableCreateStatement) {
        build
            .col(
                ColumnDef::new(Self::Identity::BookId)
                    .unsigned()
                    .not_null()
            )
            .col(
                ColumnDef::new(Self::Identity::AuthorId)
                    .unsigned()
                    .not_null()
            )
            .primary_key(
                IndexCreateStatement::new()
                    .col(Self::Identity::BookId)
                    .col(Self::Identity::AuthorId)
            )
            .foreign_key(
                ForeignKeyCreateStatement::new()
                    .from(AuthorIden::Table, AuthorIden::Id)
                    .to(Self::Identity::Table, Self::Identity::AuthorId)
            )
            .foreign_key(
                ForeignKeyCreateStatement::new()
                    .from(BookIden::Table, BookIden::Id)
                    .to(Self::Identity::Table, Self::Identity::BookId)
            );
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
#[enum_def]
pub struct BookOwner {
    book_id: u32,
    username: String
}
impl DatabaseCallable for BookOwner {
    type Identity = BookOwnerIden;
    fn all_columns() -> &'static [Self::Identity] {
        &[
            BookOwnerIden::BookId,
            BookOwnerIden::Username
        ]
    }
    fn table() -> Self::Identity {
        BookOwnerIden::Table
    }
    fn parse(row: postgres::Row) -> Self {
        let book_id: u32 = row.get(0);
        let username: &str = row.get(1);

        Self {
            book_id,
            username: username.to_string()
        }
    }
    fn create_columns(build: &mut TableCreateStatement) {
        build
            .col(
                ColumnDef::new(Self::Identity::BookId)
                    .unsigned()
                    .not_null()
            )
            .col(
                ColumnDef::new(Self::Identity::Username)
                    .text()
                    .not_null()
            )
            .primary_key(
                IndexCreateStatement::new()
                    .col(Self::Identity::BookId)
                    .col(Self::Identity::Username)
            )
            .foreign_key(
                ForeignKeyCreateStatement::new()
                    .from(RawUserIden::Table, RawUserIden::Username)
                    .to(Self::Identity::Table, Self::Identity::Username)
            )
            .foreign_key(
                ForeignKeyCreateStatement::new()
                    .from(BookIden::Table, BookIden::Id)
                    .to(Self::Identity::Table, Self::Identity::BookId)
            );
    }
}

pub struct LoadedContext {
    books: Vec<Book>,
    groups: Vec<BookGroup>,
    genres: Vec<Genre>,
    authors: Vec<Author>,
    users: Vec<RawUser>,
    genre_groups: Vec<GenreGroup>,
    author_groups: Vec<AuthorGroup>,
    group_bindings: Vec<GroupBinding>,
    //book_owners: Vec<BookOwner>
}

pub struct ActiveContext {
    pub books: Vec<Arc<Book>>,
    pub groups: Vec<Arc<BookGroup>>,
    pub genres: Vec<Arc<Genre>>,
    pub authors: Vec<Arc<Author>>,
    pub users: Vec<Arc<RawUser>>
}
impl ActiveContext {
    pub fn get_users(&self) -> Vec<RawUser> {
        self.users.iter().map(|x| x.as_ref()).cloned().collect()
    }
}

pub fn get_all_db_data(conn: &mut postgres::Client) -> Result<LoadedContext, postgres::Error> {
    let books:   Vec<Book>      = get_from_db(conn)?;
    let groups:  Vec<BookGroup> = get_from_db(conn)?;
    let genres:  Vec<Genre>     = get_from_db(conn)?;
    let authors: Vec<Author>    = get_from_db(conn)?;
    let users:   Vec<RawUser>   = get_from_db(conn)?;

    let genre_groups:   Vec<GenreGroup>   = get_from_db(conn)?;
    let author_groups:  Vec<AuthorGroup>  = get_from_db(conn)?;
    let group_bindings: Vec<GroupBinding> = get_from_db(conn)?;
    //let book_owners:    Vec<BookOwner>    = get_from_db(conn)?;

    Ok(
        LoadedContext {
            books,
            groups,
            genres,
            authors,
            users,
            genre_groups,
            author_groups,
            group_bindings,
            //book_owners
        }
    )
}

pub fn activate_context(loaded: LoadedContext) -> ActiveContext {
    let mut books: HashMap<u32, Book> = HashMap::new();
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
    
    let mut authors: HashMap<u32, Arc<Author>> = HashMap::new();
    for author in loaded.authors {
        authors.insert(author.id, Arc::new(author));
    }

    let mut users: HashMap<String, Arc<RawUser>> = HashMap::new();
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