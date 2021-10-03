use actix_web::{Error, Result};
use liquid::{
    model::{value, Value},
    object, Object,
};

use crate::db::{adb_execute_batch, adb_query_vec};

pub fn create_tables() -> Result<(), Error> {
    let s = "CREATE TABLE konto (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT);
    CREATE TABLE hauptkat (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT, notiz TEXT, anzeige TEXT);
    CREATE TABLE unterkat (id INTEGER PRIMARY KEY AUTOINCREMENT, hauptkatid INTEGER, name TEXT, notiz TEXT, anzeige TEXT);
    CREATE TABLE unterkat_monat (id INTEGER PRIMARY KEY AUTOINCREMENT, subkatid INTEGER, jahr TEXT, monat TEXT, betrag REAL);
    CREATE TABLE zahlempf (id INTEGER PRIMARY KEY AUTOINCREMENT, name TEXT, unterkatid INTEGER, anzeigen TEXT);
    CREATE TABLE eintrag (id INTEGER PRIMARY KEY AUTOINCREMENT, kontoid INTEGER, unterkatid INTEGER, splitid INTEGER, zahlempf TEXT, datum DATE, betrag REAL, richtung TEXT);
    CREATE TABLE split (id INTEGER PRIMARY KEY AUTOINCREMENT, kontoid INTEGER, unterkatid INTEGER, zahlempfid INTEGER, betrag REAL);
    CREATE TABLE wiederkehrend (id INTEGER PRIMARY KEY AUTOINCREMENT, typ INTEGER, nextdatum DATE, kontoid INTEGER, unterkatid INTEGER, splitid INTEGER, zahlempfid INTEGER, betrag REAL, richtung TEXT, zielbetrag REAL);";

    adb_execute_batch(s)
}

pub fn get_all_main_categories(year: u16, month: u16) -> Result<Value, Error> {
    let sql = "SELECT * FROM hauptkat";
    let a: Vec<Object> = adb_query_vec(sql, [], |r| {
        let name: String = r.get("name").unwrap();
        let id = r.get::<_, u32>("id").unwrap();
        let id = id.to_string();
        let subcat = get_sub_categories(&id).unwrap();
        object!({ "name": name, "id": id , "sc": subcat})
    })?;
    Ok(value!(&a[..]))
}

pub fn get_sub_categories(main_cat: &str) -> Result<Value, Error> {
    let sql = "SELECT * FROM unterkat WHERE hauptkatid = ?";
    let a: Vec<Object> = adb_query_vec(sql, [main_cat], |r| {
        let name: String = r.get("name").unwrap();
        object!({ "name": name })
    })?;
    Ok(value!(&a[..]))
}
