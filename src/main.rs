use actix_files::Files;
use tokio;
use tokio::process::Command;

use dotenv::dotenv;
use std::sync::Arc;
use std::sync::Mutex;
use bore_cli::client::Client;

mod server;
use actix_cors::Cors;
use actix_web::{http::header, web, App, HttpServer};

use secret_santa::SecretSantaGame;
use server::routes::routes;


const LOCAL_PORT: u16 = 8080;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // init tunneling client
    // random port
    let bore_port = rand::random::<u16>();
    tokio::spawn(async move {
        let client = Client::new("localhost", LOCAL_PORT, "bore.pub", bore_port, None).await.unwrap();
        client.listen().await.unwrap();
    });

    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    dotenv().ok();

    let game = SecretSantaGame::default();
    let game_data = Arc::new(Mutex::new(game));
    let secret_santa_game = web::Data::new(game_data);

    let bore_url = format!("http://bore.pub:{}", bore_port);
    let localhost_url = format!("http://localhost:{}", LOCAL_PORT);

    log::info!("Starting server on {}", bore_url);
    // run command to open bore url in browser
    Command::new("open")
        .arg(bore_url.clone())
        .output()
        .await.unwrap();

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
        App::new()
            .wrap(actix_web::middleware::Logger::default())
            .app_data(secret_santa_game.clone())
            .service(web::scope("/secret-santa").configure(routes))
            .service(Files::new("/", "./front/dist/").index_file("index.html"))
            .wrap(cors)
    })
    .bind(("127.0.0.1", LOCAL_PORT))?
    .run()
    .await
}
