use sea_query::*;

use std::sync::Arc;

pub trait DatabaseCallable: Sized {
    type Identity: Iden + 'static;
    fn all_columns() -> &'static [Self::Identity];
    fn create_columns(build: &mut TableCreateStatement);
    fn table() -> Self::Identity;
    fn parse(row: postgres::Row) -> Self;
}
pub fn get_from_db<T>(conn: &mut postgres::Client) -> Result<Vec<T>, postgres::Error> where T: DatabaseCallable, T::Identity: Clone {
    let mut query = Query::select();
    for col in T::all_columns() {
        query.column(col.clone());
    }

    query.from(T::table());
    let (query_string, _) = query.build(PostgresQueryBuilder);

    let result = conn.query(&query_string, &[])?;
    let mut return_result = vec![];
    for row in result {
        return_result.push(
            T::parse(row)
        )
    }

    Ok ( return_result )
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

pub fn create_table_for<T>(conn: &mut postgres::Client) -> Result<(), postgres::Error> where T: DatabaseCallable {
    let mut table_base = Table::create();
    let statement_build = table_base
        .table(T::table())
        .if_not_exists();

    T::create_columns(statement_build);

    let statement = statement_build.build(PostgresQueryBuilder);

    conn.execute(&statement, &[])?;

    Ok( () )
}

pub fn arc_wrap<T>(vals: Vec<T>) -> Vec<Arc<T>> {
    vals.into_iter().map(Arc::new).collect()
}