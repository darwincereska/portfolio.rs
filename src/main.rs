#[macro_use] extern crate rocket;
use rocket::fs::{FileServer,NamedFile};
#[get("/home")]
async fn home() -> Option<NamedFile> {
    let file: &str = "src/pages/home.html";
    NamedFile::open(file).await.ok()
}

// Routes
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
        .register("/", catchers![not_found])
        .mount("/", routes![index, four_o_four, home,about])
        .mount("/public", FileServer::from("src/static"))
}