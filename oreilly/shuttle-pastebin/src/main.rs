#[macro_use]
extern crate rocket;

mod paste_id;

use std::io::{self, ErrorKind};

use paste_id::PasteId;
use rocket::form::Form;
use rocket::http::uri::Absolute;
use rocket::response::content::{self, RawHtml};
use rocket::response::Redirect;
use rocket::Data;
use rocket::{data::ToByteUnit, tokio::fs::File};
use tokio::io::AsyncWriteExt;

const ID_LENGTH: usize = 5;
const HOST: Absolute<'static> = uri!("http://localhost:8000");

#[catch(404)]
fn not_found() -> &'static str {
    "NOT FOUND!"
}

#[shuttle_runtime::main]
async fn rocket() -> shuttle_rocket::ShuttleRocket {
    Ok(rocket::build()
        .mount(
            "/",
            routes![index, retrieve, upload, form, upload_form, delete],
        )
        .register("/", catchers![not_found])
        .into())
}

#[get("/")]
fn index() -> &'static str {
    "
    USAGE

      POST /

          accepts raw data in the body of the request and responds with a URL of
          a page containing the body's content

      GET /<id>

          retrieves the content for the paste with id `<id>`
    "
}

#[get("/form")]
async fn form() -> Option<RawHtml<File>> {
    File::open("form.html")
        .await
        .ok()
        .map(|f| content::RawHtml(f))
}

#[derive(Responder)]
enum DeleteError<'r> {
    #[response(status = 404)]
    NotFound(&'r str),
    #[response(status = 500)]
    Err(&'r str),
}

#[delete("/<id>")]
async fn delete<'a>(id: PasteId<'a>) -> Result<&'static str, DeleteError<'a>> {
    let filename = id.file_path();
    tokio::fs::remove_file(filename)
        .await
        .map(|_| "deleted")
        .map_err(|e| match e.kind() {
            ErrorKind::NotFound => DeleteError::NotFound("not found\n"),
            _ => DeleteError::Err("whoops\n"),
        })
}

#[get("/<id>")]
async fn retrieve(id: PasteId<'_>) -> Option<File> {
    let filename = id.file_path();
    File::open(&filename).await.ok()
}

#[derive(FromForm)]
struct PasteForm {
    body: String,
}

#[post("/", format = "multipart/form-data", data = "<form>", rank = 1)]
async fn upload_form(form: Form<PasteForm>) -> io::Result<Redirect> {
    let id = PasteId::new(ID_LENGTH);
    let filename = id.file_path();
    let mut f = File::create(filename).await?;
    f.write_all(form.body.as_bytes()).await?;
    let res = uri!(HOST, retrieve(id)).to_string();
    Ok(Redirect::to(res))
}

#[post("/", data = "<paste>", rank = 2)]
async fn upload(paste: Data<'_>) -> std::io::Result<String> {
    let id = PasteId::new(ID_LENGTH);
    let filename = id.file_path();
    paste.open(128.kilobytes()).into_file(filename).await?;
    Ok(uri!(HOST, retrieve(id)).to_string())
}
