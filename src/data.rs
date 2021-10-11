use actix_web::{Error, Result};
use liquid::{
    model::{value, Value},
    object, Object,
};
use rusqlite::Row;

use crate::db::{adb_execute, adb_execute_batch, adb_query_vec, adb_select};

pub fn create_tables() -> Result<(), Error> {
    let s = "CREATE TABLE konto (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT);
    CREATE TABLE hauptkat (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT, notiz TEXT, anzeige TEXT);
    CREATE TABLE unterkat (id INTEGER PRIMARY KEY AUTOINCREMENT, hauptkatid INTEGER, name TEXT, notiz TEXT, anzeige TEXT);
    CREATE TABLE unterkat_monat (id INTEGER PRIMARY KEY AUTOINCREMENT, subkatid INTEGER, jahr TEXT, monat TEXT, betrag REAL);
    CREATE TABLE zahlempf (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT UNIQUE, unterkatid INTEGER, anzeigen TEXT);
    CREATE TABLE eintrag (id INTEGER PRIMARY KEY AUTOINCREMENT, kontoid INTEGER, unterkatid INTEGER, splitid INTEGER, zahlempf TEXT, datum DATE, betrag REAL);
    CREATE TABLE split (id INTEGER PRIMARY KEY AUTOINCREMENT, kontoid INTEGER, unterkatid INTEGER, zahlempfid INTEGER, betrag REAL);
    CREATE TABLE wiederkehrend (id INTEGER PRIMARY KEY AUTOINCREMENT, typ INTEGER, nextdatum DATE, kontoid INTEGER, unterkatid INTEGER, splitid INTEGER, zahlempfid INTEGER, betrag REAL, richtung TEXT, zielbetrag REAL);";

    adb_execute_batch(s)
}

pub fn get_all_main_categories(_year: u16, _month: u16) -> Result<Value, Error> {
    let sql = "SELECT * FROM hauptkat";
    let a: Vec<Object> = adb_query_vec(sql, [], fetch_main_category)?;
    Ok(value!(&a[..]))
}

pub fn get_main_category(_year: u16, _month: u16, id: u16) -> Result<Object, Error> {
    let sql = "SELECT * FROM hauptkat where id = ?";
    let a: Vec<Object> = adb_query_vec(sql, [id], fetch_main_category)?;
    Ok(a.first().unwrap().clone())
}

pub fn get_sub_categories(main_cat: &str) -> Result<Value, Error> {
    let sql = "SELECT * FROM unterkat WHERE hauptkatid = ?";
    let a: Vec<Object> = adb_query_vec(sql, [main_cat], |r| {
        let name: String = r.get("name").unwrap();
        let id: u16 = r.get("id").unwrap();
        object!({ "name": name , "id": id})
    })?;
    Ok(value!(&a[..]))
}

pub fn fetch_main_category(r: &Row) -> Object {
    let name: String = r.get("name").unwrap();
    let id = r.get::<_, u32>("id").unwrap();
    let id = id.to_string();
    let subcat = get_sub_categories(&id).unwrap();
    object!({ "name": name, "id": id , "sc": subcat})
}

pub fn create_main_category(name: String) -> Result<(), Error> {
    let sql = "Insert into hauptkat (name) values (?)";
    adb_execute(sql, [name])
}

pub fn create_sub_category(name: String, maincat_id: u16) -> Result<(), Error> {
    let sql = "Insert into unterkat (name, hauptkatid) values (?, ?)";
    adb_execute(sql, [name, maincat_id.to_string()])
}

pub fn delete_sub_category(subcat_id: u16) -> Result<(), Error> {
    let sql = "DELETE FROM unterkat WHERE id = ?";
    adb_execute(sql, [subcat_id])
}

pub fn get_payee_datalist() -> Result<Value, Error> {
    let sql = "SELECT name FROM zahlempf GROUP BY name";
    let a: Vec<String> = adb_query_vec(sql, [], |r| {
        format!(
            "<option value=\"{}\">\n",
            r.get::<_, String>("name").unwrap()
        )
    })?;
    Ok(value!(a.into_iter().collect::<String>()))
}

pub fn get_payee_id(name: String) -> Result<u32, Error> {
    adb_select("SELECT id FROM zahlempf WHERE name = ?", &[&name], |r| {
        r.get::<_, u32>("id")
    })
}

pub fn add_transact(
    subcatid: String,
    payee: String,
    date: String,
    amount: String,
) -> Result<(), Error> {
    let sql = "INSERT OR IGNORE INTO zahlempf (name) values (?)";
    adb_execute(sql, &[&payee])?;

    let payee_id = get_payee_id(payee)?;

    let sql = "INSERT INTO eintrag (kontoid, unterkatid, splitid, zahlempf, datum, betrag)
        values (0, ?, 0, ?, ?, ?)";

    adb_execute(sql, &[&subcatid, &payee_id.to_string(), &date, &amount])
}
