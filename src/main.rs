use actix_files::Files;
use dotenv::dotenv;
use std::sync::Arc;
use std::sync::Mutex;

mod server;
use actix_cors::Cors;
use actix_web::{http::header, web, App, HttpServer};

use secret_santa::SecretSantaGame;
use server::routes::routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    dotenv().ok();

    let game = SecretSantaGame::default();
    let game_data = Arc::new(Mutex::new(game));
    let secret_santa_game = web::Data::new(game_data);

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:8080")
            .allowed_origin("http://localhost:8000")
            .allowed_origin("http://127.0.0.1:8080")
            .allowed_origin("http://127.0.0.1:8000")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
            ])
            .supports_credentials();
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .app_data(secret_santa_game.clone())
            .service(web::scope("/secret-santa").configure(routes))
            .service(Files::new("/", "./front/dist/").index_file("index.html"))
            .wrap(cors)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
