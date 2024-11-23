use actix_web::{http::header, web, App, HttpServer};
use tokio;
use tokio::process::Command;

use dotenv::dotenv;
use std::sync::Arc;
use std::sync::Mutex;
use bore_cli::client::Client;

mod server;
use actix_cors::Cors;

use secret_santa::SecretSantaGame;
use server::routes::routes;


const LOCAL_PORT: u16 = 8080;
use actix_web_static_files::ResourceFiles;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    dotenv().ok();

    // random port
    let bore_port = rand::random::<u16>();

    // create game login
    let game = SecretSantaGame::default();
    let game_data = Arc::new(Mutex::new(game));
    let secret_santa_game = web::Data::new(game_data);

    let bore_url = format!("http://bore.pub:{}", bore_port);
    let localhost_url = format!("http://localhost:{}", LOCAL_PORT);

    log::info!("Starting server on {}", bore_url);
    // run command to open bore url in browser
    if cfg!(debug_assertions) {
        log::info!("Running in debug mode");
    } else {
        log::info!("Running in release mode");
        // init tunneling client
        tokio::spawn(async move {
            let client = Client::new("localhost", LOCAL_PORT, "bore.pub", bore_port, None).await.unwrap();
            client.listen().await.unwrap();
        });
        Command::new("open")
            .arg(bore_url.clone())
            .output()
            .await
            .unwrap();
    }

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&bore_url)
            .allowed_origin(&localhost_url)
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
                header::ORIGIN,
            ])
            .supports_credentials();
        let generated = generate();
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .app_data(secret_santa_game.clone())
            .service(web::scope("/secret-santa").configure(routes))
            .service(ResourceFiles::new("/", generated))
            .wrap(cors)
    })
    .bind(("127.0.0.1", LOCAL_PORT))?
    .run()
    .await
}

