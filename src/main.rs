use actix_files::Files;
use actix_web::{http::header, web, App, HttpServer};
use tokio;

use bore_cli::client::Client;
use dotenv::dotenv;
use std::sync::Arc;
use std::sync::Mutex;

mod server;
use actix_cors::Cors;

use secret_santa::SecretSantaGame;
use server::routes::routes;
use server::utils::open_browser;

const LOCAL_PORT: u16 = 8080;
use actix_web_static_files::ResourceFiles;

include!(concat!(env!("OUT_DIR"), "/generated.rs"));


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("info"));
    dotenv().ok();

    // random port between 1000 and 2000
    let bore_port = rand::random::<u16>() % 1000 + 1000;

    // create game logic
    let game = SecretSantaGame::default();
    let game_data = Arc::new(Mutex::new(game));
    let secret_santa_game = web::Data::new(game_data);

    let tunnel_url = format!("http://tunnel.guibeira.com:{}", bore_port);

    #[cfg(debug_assertions)]
    let localhost_url = format!("http://localhost:{}", LOCAL_PORT);

    #[cfg(debug_assertions)]
    log::info!("Starting server on {}", localhost_url);

    #[cfg(not(debug_assertions))]
    log::info!("Starting server on {}", tunnel_url);

    // run command to open bore url in browser
    if cfg!(debug_assertions) {
        log::info!("Running in debug mode");
    } else {
        log::info!("Running in release mode");
        // init tunneling client
        tokio::spawn(async move {
            let client = Client::new("localhost", LOCAL_PORT, "tunnel.guibeira.com", bore_port, Some("santa"))
                .await
                .unwrap();
            client.listen().await.unwrap();
        });

    }

    if cfg!(not(debug_assertions)) {
        log::info!("Opening browser");
        let url = format!("https://tunnel.guibeira.com/{}", bore_port);
        open_browser(&url).await;
    }

    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin(&tunnel_url)
            .allowed_origin("http://localhost:8000")
            .allowed_origin("http://localhost:8080")
            .allowed_origin("http://127.0.0.1:8000")
            .allowed_origin("http://127.0.0.1:8080")
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![
                header::CONTENT_TYPE,
                header::AUTHORIZATION,
                header::ACCEPT,
                header::ORIGIN,
            ])
            .supports_credentials();
        let generated = generate();
        let mut app = App::new()
            .wrap(actix_web::middleware::Logger::default())
            .app_data(secret_santa_game.clone())
            .service(web::scope("/secret-santa").configure(routes))
            .wrap(cors);

        if cfg!(debug_assertions) {
            app = app.service(Files::new("/", "./front/dist/").index_file("index.html"));
        } else {
            app = app.service(ResourceFiles::new("/", generated));
        }
        app
    })
    .bind(("127.0.0.1", LOCAL_PORT))?
    .run()
    .await

}
