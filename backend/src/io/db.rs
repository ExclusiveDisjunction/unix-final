use postgres::types::ToSql;
use sea_query::*;

use std::{fmt::{Debug, Display}, iter::zip, sync::Arc};

use crate::log_debug;

pub enum SqlError {
    Db(postgres::Error),
    Sql(sea_query::error::Error)
}
impl From<postgres::Error> for SqlError{
    fn from(value: postgres::Error) -> Self {
        Self::Db(value)
    }
}
impl From<sea_query::error::Error> for SqlError {
    fn from(value: sea_query::error::Error) -> Self {
        Self::Sql(value)
    }
}
impl Debug for SqlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x: &dyn Debug = match self {
            Self::Db(v) => v,
            Self::Sql(v) => v
        };

        x.fmt(f)
    }
}
impl Display for SqlError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x: &dyn Display = match self {
            Self::Db(v) => v,
            Self::Sql(v) => v
        };

        x.fmt(f)
    }
}

pub trait DatabaseRepr : Sized {
    type Identity: Iden + 'static;

    fn is_new(&self) -> bool;

    fn all_columns() -> &'static [Self::Identity];
    fn table() -> Self::Identity;
}
pub trait DatabaseQueryable: DatabaseRepr {
    fn parse(row: postgres::Row) -> Self;
}
pub trait DatabaseTableCreatable: DatabaseQueryable {
    fn create_columns(build: &mut TableCreateStatement);
}
pub trait DatabaseInsertable: DatabaseRepr {
    fn insert_values(&self) -> Vec<sea_query::SimpleExpr>;
}
pub trait DatabaseUpdatable: DatabaseInsertable {
    fn id_col() -> Self::Identity;
    fn id_val(&self) -> SimpleExpr;
}

pub async fn select<T>(conn: &mut tokio_postgres::Client) -> Result<Vec<T>, postgres::Error> where T: DatabaseQueryable, T::Identity: Clone {
    let mut query = Query::select();
    for col in T::all_columns() {
        query.column(col.clone());
    }

    query.from(T::table());
    let (query_string, _) = query.build(PostgresQueryBuilder);

    let result = conn.query(&query_string, &[]).await?;
    let mut return_result = vec![];
    for row in result {
        return_result.push(
            T::parse(row)
        )
    }

    Ok ( return_result )
}
pub async fn create_table<T>(conn: &mut tokio_postgres::Client) -> Result<(), postgres::Error> where T: DatabaseTableCreatable {
    let mut table_base = Table::create();
    let statement_build = table_base
        .table(T::table())
        .if_not_exists();

    T::create_columns(statement_build);

    let statement = statement_build.build(PostgresQueryBuilder);

    conn.execute(&statement, &[]).await?;

    Ok( () )
}

pub fn split_for_update_insert<T, V>(values: &Vec<T>) -> (Vec<&V>, Vec<&V>) where T: AsRef<V>, V: DatabaseRepr {
    let mut insert: Vec<&V> = vec![];
    let mut update: Vec<&V> = vec![];

    for value in values {
        let reference = value.as_ref();
        if reference.is_new() {
            insert.push(reference)
        }
        else {
            update.push(reference)
        }
    }

    (insert, update)
}

fn map_value_to_type(value: Value) -> Box<(dyn ToSql + Sync)> {
    use Value::*;

    match value {
        BigInt(Some(v)) => Box::new(v),
        BigInt(None) => Box::new(Option::<i64>::None),
        
        Int(Some(v)) => Box::new(v),
        Int(None) =>  Box::new(Option::<i32>::None),

        SmallInt(Some(v)) => Box::new(v),
        SmallInt(None) => Box::new(Option::<i16>::None),

        TinyInt(Some(v)) => Box::new(v),
        TinyInt(None) => Box::new(Option::<i8>::None),

        Unsigned(Some(v)) => Box::new(v),
        Unsigned(None) => Box::new(Option::<u32>::None),

        Bool(Some(v)) => Box::new(v),
        Bool(None) => Box::new(Option::<bool>::None),

        Bytes(Some(v)) => v,
        Bytes(None) => Box::new(Option::<Vec<u8>>::None),

        String(Some(v)) => v,
        String(None) => Box::new(Option::<std::string::String>::None),

        _ => panic!("Unsupported data type")
    }
}

pub async fn insert<T>(conn: &mut tokio_postgres::Client, values: Vec<&T>) -> Result<(), SqlError> where T: DatabaseInsertable, T::Identity: Clone {
    let mut insert_statement = Query::insert();
    let builder = insert_statement
        .into_table(T::table())
        .columns(T::all_columns().to_vec());

    for item in values {
        builder.values(item.insert_values()).map_err(SqlError::from)?;
    }

    let (statement, values) = builder.build(PostgresQueryBuilder);
    let values: Vec<Box<(dyn ToSql + Sync)>> = values.into_iter()
        .map(map_value_to_type)
        .collect();

    let sql_values: Vec<&(dyn ToSql + Sync)> = values
        .iter()
        .map(|x| x.as_ref())
        .collect();   

    log_debug!("Running insert statement: {}", &statement);
    conn.execute(&statement, &sql_values).await.map_err(SqlError::from)?;
    
    Ok( () )
}
pub async fn update<T>(conn: &mut tokio_postgres::Client, values: Vec<&T>) -> Result<(), postgres::Error> where T: DatabaseUpdatable, T::Identity: Clone {
    for value in values {
        let mut base_statement = Query::update();
        let builder = base_statement
            .from(T::table());

        let elements = value.insert_values();
        let cols = T::all_columns().iter().cloned();
        let zipped: Vec<(T::Identity, SimpleExpr)> = zip(cols, elements).collect();

        builder.values(zipped);

        builder.and_where(Expr::col(T::id_col()).eq( value.id_val() ));

        let (statement, values) = builder.build(PostgresQueryBuilder);
        let values: Vec<Box<(dyn ToSql + Sync)>> = values.into_iter()
        .map(map_value_to_type)
        .collect();

        let sql_values: Vec<&(dyn ToSql + Sync)> = values
            .iter()
            .map(|x| x.as_ref())
            .collect();   

        log_debug!("Running update statement: '{}'", &statement);
        conn.execute(&statement, &sql_values).await?;
    }

    Ok( () )
}

pub fn arc_wrap<T>(vals: Vec<T>) -> Vec<Arc<T>> {
    vals.into_iter().map(Arc::new).collect()
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