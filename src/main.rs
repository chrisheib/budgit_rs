use actix_files::NamedFile;
use actix_session::{CookieSession, Session};
use actix_web::{
    error::{ErrorInternalServerError, ErrorNotFound},
    get,
    http::header::{ContentDisposition, DispositionParam, DispositionType},
    web, App, Error, HttpResponse, HttpServer,
};
use data::create_tables;
use liquid::{object, Object, ParserBuilder};
use std::path::Path;

use crate::data::get_all_main_categories;

mod data;
mod db;

const GL_PORT: i16 = 83i16;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .wrap(CookieSession::signed(&[0; 32]).secure(false))
            .service(net_main)
            .service(net_init)
            .service(net_set_month)
            .service(net_asset)
            .service(net_404)
    })
    .bind(format!(":{}", GL_PORT))?
    .bind(format!("localhost:{}", GL_PORT))?
    .run()
    .await
}

#[get("/")]
async fn net_main(session: Session) -> Result<HttpResponse, Error> {
    let month = session.get::<u16>("month")?.unwrap_or(0);
    let year = session.get::<u16>("year")?.unwrap_or(21);

    let a = get_all_main_categories(year, month)?;
    let r = render_with_theme(
        "html/index.liquid",
        object!({"month": month, "year": year, "categories": a }),
    )?;
    Ok(HttpResponse::Ok().body(r))
}

#[get("/month/{year}/{month}")]
async fn net_set_month(
    session: Session,
    web::Path((year, month)): web::Path<(u32, u32)>,
) -> Result<HttpResponse, Error> {
    session.set("year", year)?;
    session.set("month", month)?;
    Ok(HttpResponse::Found().header("Location", "/").finish())
}

fn render_with_theme(path: &str, data: Object) -> Result<String, Error> {
    let doc = std::fs::read_to_string(path)?;
    let theme = std::fs::read_to_string("html/theme.liquid")?;
    let theme_data = object!({ "content": doc });

    let r = ParserBuilder::with_stdlib()
        .build()
        .map_err(ErrorInternalServerError)?
        .parse(&theme)
        .map_err(ErrorInternalServerError)?
        .render(&theme_data)
        .map_err(ErrorInternalServerError)?;

    let r = ParserBuilder::with_stdlib()
        .build()
        .map_err(ErrorInternalServerError)?
        .parse(&r)
        .map_err(ErrorInternalServerError)?
        .render(&data)
        .map_err(ErrorInternalServerError)?;

    Ok(r)
}

#[get("/{path}")]
async fn net_asset(web::Path(path): web::Path<String>) -> Result<NamedFile, Error> {
    let p = &format!("assets/{}", path);
    let p = Path::new(p);

    if let Ok(file) = NamedFile::open(p) {
        let filename = p
            .file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default();

        Ok(file
            .use_last_modified(true)
            .set_content_disposition(ContentDisposition {
                disposition: DispositionType::Inline,
                parameters: vec![DispositionParam::Filename(filename.to_owned())],
            }))
    } else {
        Err(ErrorNotFound("No pages here."))
    }
}

#[get("/init")]
async fn net_init() -> Result<String, Error> {
    create_tables()?;
    Ok("Init successfull!".to_string())
}

#[get("/*")]
async fn net_404() -> Result<String, Error> {
    Err(ErrorNotFound("No pages here."))
}

pub fn errconv<T>(r: stable_eyre::Result<T>) -> Result<T, Error> {
    match r {
        Ok(o) => Ok(o),
        Err(e) => Err(ErrorInternalServerError(e)),
    }
}
