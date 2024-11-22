#[macro_use] extern crate rocket;

use rocket::{
    fairing::AdHoc, fs::{FileServer, NamedFile}, http::Header, response::Response
};
use rocket::form::Form;
use serde_json::json;
use reqwest::Client;
#[rocket::async_trait]
pub trait Compression {
    fn compress(&self) -> Response<'static>;
}

#[get("/home")]
async fn home() -> Option<NamedFile> {
    let file: &str = "src/pages/home.html";
    NamedFile::open(file).await.ok()
}

#[get("/contact")]
async fn contact() -> Option<NamedFile> {
    let file: &str = "src/pages/contact.html";
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

#[derive(FromForm)]
struct ContactForm {
    name: String,
    email: String,
    message: String,
}

#[post("/contact", data = "<form>")]
async fn contact_form(form: Form<ContactForm>) -> &'static str {
    const WEBHOOK_URL: &str = "https://discord.com/api/webhooks/1309413812390199336/DmaOILGLgu8Z0RXGjhhwu5mYxFcl-L4jNtvFaINrrsogzWZjRRCQ2neAVyISu12JKgoD";
    
    let client = Client::new();
    
    let message = json!({
        "embeds": [{
            "title": "@everyone New Contact Form Submission",
            "color": 16711935, // #FF00FF in decimal
            "fields": [
                {
                    "name": "Name",
                    "value": form.name,
                    "inline": true
                },
                {
                    "name": "Email",
                    "value": form.email,
                    "inline": true
                },
                {
                    "name": "Message",
                    "value": form.message
                }
            ]
        }]
    });

    match client.post(WEBHOOK_URL)
        .json(&message)
        .send()
        .await {
            Ok(_) => "Message sent successfully!",
            Err(_) => "Failed to send message."
        }
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
        .mount("/", routes![index, four_o_four, home, about, contact])
        .mount("/public", FileServer::from("src/static"))
        .mount("/api", routes![contact_form])
}
