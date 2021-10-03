use crate::errconv;
use actix_web::Error;
use rusqlite::{Connection, Params};
use stable_eyre::{eyre::Context, Result};

#[allow(dead_code)]
pub fn adb_select<T, P, F>(sql: &str, params: P, f: F) -> Result<T, Error>
where
    P: Params,
    F: FnOnce(&rusqlite::Row<'_>) -> std::result::Result<T, rusqlite::Error>,
{
    errconv(db_select(sql, params, f))
}

fn db_select<T, P, F>(sql: &str, params: P, f: F) -> Result<T>
where
    P: Params,
    F: FnOnce(&rusqlite::Row<'_>) -> std::result::Result<T, rusqlite::Error>,
{
    let c = db_con()?;
    let mut stmt = c.prepare(sql)?;
    stmt.query_row(params, f).wrap_err("query_row")
}

pub fn db_query_vec<T, P, F>(sql: &str, params: P, f: F) -> Result<Vec<T>>
where
    P: Params,
    F: Fn(&rusqlite::Row<'_>) -> T,
{
    let c = db_con()?;
    let mut query = c.prepare(sql)?;
    let mut rows = query.query(params)?;

    let mut a = Vec::<T>::new();
    while let Some(r) = rows.next()? {
        a.push(f(r))
    }
    Ok(a)
}

pub fn adb_query_vec<T, P, F>(sql: &str, params: P, f: F) -> actix_web::Result<Vec<T>, Error>
where
    P: Params,
    F: Fn(&rusqlite::Row<'_>) -> T,
{
    errconv(db_query_vec(sql, params, f))
}

#[allow(dead_code)]
fn db_uint32_read(sql: &str) -> Result<u32> {
    let c = db_con()?;
    c.query_row::<u32, _, _>(sql, [], |row| row.get(0))
        .wrap_err(format!("db_uint32_read: {}", sql))
}

#[allow(dead_code)]
pub fn adb_uint32_read(sql: &str) -> actix_web::Result<u32, Error> {
    errconv(db_uint32_read(sql))
}

#[allow(dead_code)]
fn db_str_read(sql: &str) -> Result<String> {
    let c = db_con()?;
    c.query_row::<String, _, _>(sql, [], |row| row.get(0))
        .wrap_err(format!("db_str_read: {}", sql))
}

#[allow(dead_code)]
pub fn adb_str_read(sql: &str) -> actix_web::Result<String, Error> {
    errconv(db_str_read(sql))
}

#[allow(dead_code)]
fn db_execute(sql: &str) -> Result<()> {
    let conn = db_con()?;
    conn.execute(sql, [])
        .wrap_err(format!("db_execute: {}", sql))?;
    Ok(())
}

#[allow(dead_code)]
pub fn adb_execute(sql: &str) -> actix_web::Result<(), Error> {
    errconv(db_execute(sql))
}

#[allow(dead_code)]
fn db_execute_batch(sql: &str) -> Result<()> {
    let conn = db_con()?;
    conn.execute_batch(sql)
        .wrap_err(format!("db_execute: {}", sql))?;
    Ok(())
}

#[allow(dead_code)]
pub fn adb_execute_batch(sql: &str) -> actix_web::Result<(), Error> {
    errconv(db_execute_batch(sql))
}

#[allow(dead_code)]
fn db_con() -> Result<Connection> {
    Connection::open("budgit_rs.sqlite").wrap_err("get_db_con")
}
