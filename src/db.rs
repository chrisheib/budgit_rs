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

fn db_execute(sql: &str) -> Result<()> {
    let conn = db_con()?;
    conn.execute(sql, [])
        .wrap_err(format!("db_execute: {}", sql))?;
    Ok(())
}

pub fn adb_execute(sql: &str) -> actix_web::Result<(), Error> {
    errconv(db_execute(sql))
}

fn db_con() -> Result<Connection> {
    Connection::open("budgit_rs.sqlite").wrap_err("get_db_con")
}

pub fn adb_con() -> actix_web::Result<Connection, Error> {
    errconv(db_con())
}

pub fn adb_create_tables() -> Result<(), Error> {
    let s = "CREATE TABLE konto (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT);
    CREATE TABLE hauptkat (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT, notiz TEXT, anzeige TEXT);
    CREATE TABLE unterkat (id INTEGER PRIMARY KEY AUTOINCREMENT, hauptkatid INTEGER, name TEXT, notiz TEXT, anzeige TEXT);
    CREATE TABLE unterkat_monat (id INTEGER PRIMARY KEY AUTOINCREMENT, subkatid INTEGER, jahr TEXT, monat TEXT, betrag REAL);
    CREATE TABLE zahlempf (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT, unterkatid INTEGER, anzeigen TEXT);
    CREATE TABLE eintrag (id INTEGER PRIMARY KEY AUTOINCREMENT, kontoid INTEGER, unterkatid INTEGER, splitid INTEGER, zahlempf TEXT, datum DATE, betrag REAL, richtung TEXT);
    CREATE TABLE split (id INTEGER PRIMARY KEY AUTOINCREMENT, kontoid INTEGER, unterkatid INTEGER, zahlempfid INTEGER, betrag REAL);
    CREATE TABLE wiederkehrend (id INTEGER PRIMARY KEY AUTOINCREMENT, typ INTEGER, nextdatum DATE, kontoid INTEGER, unterkatid INTEGER, splitid INTEGER, zahlempfid INTEGER, betrag REAL, richtung TEXT, zielbetrag REAL);";

    adb_execute(s)
}
