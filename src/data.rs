use actix_web::{Error, Result};
use liquid::{
    model::{value, Value},
    object, Object,
};

use crate::db::adb_query_vec;

pub fn get_all_main_categories() -> Result<Value, Error> {
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
