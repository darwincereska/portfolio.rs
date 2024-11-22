#[macro_use] extern crate rocket;
use rocket::{
    fairing::AdHoc, fs::{FileServer, NamedFile}, http::Header, response::Response
};

#[rocket::async_trait]
pub trait Compression {
    fn compress(&self) -> Response<'static>;
}

#[get("/home")]
async fn home() -> Option<NamedFile> {
    let file: &str = "src/pages/home.html";
    NamedFile::open(file).await.ok()
}

#[get("/")]
async fn index() -> Option<NamedFile> {
    let file: &str = "src/pages/index.html";
    NamedFile::open(file).await.ok()
}

#[get("/about")]
async fn about() -> Option<NamedFile> {
    let file: &str = "src/pages/about.html";
    NamedFile::open(file).await.ok()
}

#[get("/404")]
async fn four_o_four() -> Option<NamedFile> {
    let file: &str = "src/pages/404.html";
    NamedFile::open(file).await.ok()
}

#[catch(404)]
async fn not_found() -> Option<NamedFile> {
    let file: &str = "pages/404.html";
    NamedFile::open(file).await.ok()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .attach(AdHoc::on_response("Compression", |_, response| {
            Box::pin(async move {
                if let Some(accept_encoding) = response.headers().get_one("Accept-Encoding") {
                    if accept_encoding.contains("gzip") {
                        response.set_header(Header::new("Content-Encoding", "gzip"));
                    }
                }
            })
        }))
        .register("/", catchers![not_found])
        .mount("/", routes![index, four_o_four, home, about])
        .mount("/public", FileServer::from("src/static"))
}
