use actix_web::{error::ErrorInternalServerError, Error, Result};
use liquid::{object, Object};

use crate::db::adb_con;

pub fn get_all_main_categories() -> Result<Object, Error> {
    let s = "SELECT * FROM hauptkat";
    let c = adb_con()?;
    let mut query = c.prepare(s).map_err(ErrorInternalServerError)?;
    let mut rows = query.query([]).map_err(ErrorInternalServerError)?;

    let mut a = Vec::<Object>::new();
    while let Some(r) = rows.next().map_err(ErrorInternalServerError)? {
        let name: String = r.get("name").unwrap();
        let id = r.get::<_, u32>("id").unwrap();
        let id = id.to_string();
        let subcat = get_sub_categories(&id)?;
        let cat_obj = object!({ "name": name, "id": id , "sc": subcat});
        a.push(cat_obj);
    }
    Ok(object!({ "kats": &a[..] }))
}

pub fn get_sub_categories(main_cat: &str) -> Result<Object, Error> {
    let s = "SELECT * FROM unterkat WHERE hauptkatid = ?";
    let c = adb_con()?;
    let mut query = c.prepare(s).map_err(ErrorInternalServerError)?;
    let mut rows = query.query([main_cat]).map_err(ErrorInternalServerError)?;

    let mut a = Vec::<Object>::new();

    while let Some(r) = rows.next().map_err(ErrorInternalServerError)? {
        let name: String = r.get("name").unwrap();
        let sub_obj = object!({ "name": name });
        a.push(sub_obj);
    }
    Ok(object!({ "subkats": &a[..] }))
}
