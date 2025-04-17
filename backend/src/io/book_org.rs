use serde::{Serialize, Deserialize};

use sea_query::*;

use super::db::{DatabaseRepr, DatabaseInsertable, DatabaseQueryable, DatabaseTableCreatable, DatabaseUpdatable};

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Debug)]
#[enum_def]
pub struct DbGroup {
    pub id: i32,
    pub name: String,
    pub desc: String,
    is_new: bool
}
impl DatabaseRepr for DbGroup {
    type Identity = DbGroupIden;
    fn is_new(&self) -> bool {
        self.is_new
    }
    
    fn all_columns() -> &'static [Self::Identity] {
        &[
            DbGroupIden::Id,
            DbGroupIden::Name,
            DbGroupIden::Desc
        ]
    }
    fn table() -> Self::Identity {
        DbGroupIden::Table
    }
}
impl DatabaseQueryable for DbGroup {
    fn parse(row: postgres::Row) -> Self {
        let id: i32 = row.get(0);
        let name: &str = row.get(1);
        let desc: &str = row.get(2);

        Self {
            id,
            name: name.to_string(),
            desc: desc.to_string(),
            is_new: false
        }
    }
}
impl DatabaseTableCreatable for DbGroup {
    fn create_columns(build: &mut TableCreateStatement) {
        build
            .col(
                ColumnDef::new(DbGroupIden::Id)
                    .integer()
                    .primary_key()
            )
            .col(
                ColumnDef::new(DbGroupIden::Name)
                    .text()
                    .not_null()
                )
            .col(
                ColumnDef::new(DbGroupIden::Desc)
                    .text()
                    .not_null()
                    .default("")
                );
    }
}
impl DatabaseInsertable for DbGroup {
    fn insert_values(&self) -> Vec<sea_query::SimpleExpr> {
        vec![
            (&self.name).into(),
            (&self.desc).into()
        ]
    }
}
impl DatabaseUpdatable for DbGroup {
    fn id_col() -> Self::Identity {
        DbGroupIden::Id
    }
    fn id_val(&self) -> SimpleExpr {
        self.id.into()
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Debug)]
#[enum_def]
pub struct Genre {
    pub id: i32,
    pub name: String,
    pub desc: String,
    pub age_group: i8,
    is_new: bool
}
impl DatabaseRepr for Genre {
    type Identity = GenreIden;
    fn is_new(&self) -> bool {
        self.is_new
    }
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
}
impl DatabaseQueryable for Genre {
    fn parse(row: postgres::Row) -> Self {
        let name: &str = row.get(0);
        let desc: &str = row.get(1);
        let age_group: i8 = row.get(2);

        Self {
            name: name.to_string(),
            desc: desc.to_string(),
            age_group,
            is_new: false
        }
    }
}
impl DatabaseTableCreatable for Genre {
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
                    .tiny_integer()
                    .not_null()
            );
    }
}
impl DatabaseInsertable for Genre {
    fn insert_values(&self) -> Vec<sea_query::SimpleExpr> {
        vec![
            (&self.name).into(),
            (&self.desc).into(),
            self.age_group.into()
        ]
    }
}
impl DatabaseUpdatable for Genre {
    fn id_col() -> Self::Identity {
        GenreIden::Name
    }
    fn id_val(&self) -> SimpleExpr {
        (&self.name).into()
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Hash, Clone, Debug)]
#[enum_def]
pub struct Author {
    pub id: i32,
    pub first_name: String,
    pub last_name: String,
    pub rating: i8,
    pub comment: String,
    is_new: bool
}  
impl DatabaseRepr for Author {
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
    fn is_new(&self) -> bool {
        self.is_new
    }
}
impl DatabaseQueryable for Author {
    fn parse(row: postgres::Row) -> Self {
        let id: i32 = row.get(0);
        let first_name: &str = row.get(1);
        let last_name: &str = row.get(2);
        let rating: i8 = row.get(3);
        let comment: &str = row.get(4);

        Self {
            id,
            first_name: first_name.to_string(),
            last_name: last_name.to_string(),
            rating,
            comment: comment.to_string(),
            is_new: false
        }
    }
}
impl DatabaseTableCreatable for Author {
    fn create_columns(build: &mut TableCreateStatement) {
        build
            .col(
                ColumnDef::new(AuthorIden::Id)
                    .integer()
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
                    .tiny_integer()
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
impl DatabaseInsertable for Author {
    fn insert_values(&self) -> Vec<sea_query::SimpleExpr> {
        vec![
            self.id.into(),
            (&self.first_name).into(),
            (&self.last_name).into(),
            self.rating.into(),
            (&self.comment).into()
        ]
    }
}
impl DatabaseUpdatable for Author {
    fn id_col() -> Self::Identity {
        AuthorIden::Id
    }
    fn id_val(&self) -> SimpleExpr {
        self.id.into()
    }
}