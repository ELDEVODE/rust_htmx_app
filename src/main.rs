use actix_files as fs;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::Deserialize;
use serde_json::Value;
use std::env;
use tera::Tera;

async fn index(tmpl: web::Data<Tera>) -> impl Responder {
    let ctx = tera::Context::new();
    let rendered = tmpl.render("index.html", &ctx).unwrap();
    HttpResponse::Ok().content_type("text/html").body(rendered)
}

async fn get_joke() -> impl Responder {
    let client = reqwest::Client::new();
    let res = client
        .get("https://official-joke-api.appspot.com/random_joke")
        .send()
        .await
        .unwrap()
        .json::<Value>()
        .await
        .unwrap();

    let setup = res["setup"].as_str().unwrap_or("No setup available");
    let punchline = res["punchline"]
        .as_str()
        .unwrap_or("No punchline available");

    HttpResponse::Ok().body(format!(
        r#"<div class='flex flex-col justify-center items-center'>
        <p class='text-xl mb-4'>{}</p>
        <p class='text-lg font-bold'>{}</p>
        <button 
            hx-post="/like-joke"
            hx-swap="outerHTML"
            class="bg-green-500 hover:bg-green-600 text-white font-bold py-2 px-4 rounded transition duration-300 mt-3"
        >
            Like this Joke 👍
        </button>
        </div>
        "#,
        setup, punchline
    ))
}

async fn like_joke() -> impl Responder {
    HttpResponse::Ok().body(
        "
        <p class='text-green-500 font-bold mt-4' >
            You liked this Joke!
        </p>
    ",
    )
}

#[derive(Deserialize)]
struct HelloQuery {
    name: String,
}

async fn hello(query: web::Query<HelloQuery>) -> impl Responder {
    let name = &query.name;
    println!("Received name: {}", name);
    HttpResponse::Ok().body(format!("Welcome, {}!", name))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let tera = Tera::new("templates/**/*").unwrap();

    let port = std::env::var("PORT")
        .unwrap_or_else(|_| "8080".to_string())
        .parse()
        .expect("Invalid port number");

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(tera.clone()))
            .service(fs::Files::new("/static", "./static").show_files_listing())
            .route("/", web::get().to(index))
            .route("/get-joke", web::get().to(get_joke))
            .route("/like-joke", web::post().to(like_joke))
            .route("/api/HttpExample", web::get().to(hello))
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
