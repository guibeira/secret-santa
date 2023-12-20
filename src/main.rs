use dotenv::dotenv;
use std::sync::Arc;
use std::sync::Mutex;

mod server;
use actix_web::{web, App, HttpServer};

use secret_santa::SecretSantaGame;
use server::routes::routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    dotenv().ok();

    let game = SecretSantaGame::new();
    let game_data = Arc::new(Mutex::new(game));
    let secret_santa_game = web::Data::new(game_data);

    HttpServer::new(move || {
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .app_data(secret_santa_game.clone())
            .service(web::scope("/secret-santa").configure(routes))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
