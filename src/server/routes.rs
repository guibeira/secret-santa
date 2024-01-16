use std::sync::{Arc, Mutex};

use super::error::CustomError;
use super::models::{PickedResponse, PlayerInfo, Players, SantaGameInfo};
use actix_web::{web, HttpResponse};
use secret_santa::{Player, SecretSantaGame};

async fn index(game_data: web::Data<Arc<Mutex<SecretSantaGame>>>) -> SantaGameInfo {
    let game = game_data.lock().unwrap();
    SantaGameInfo {
        status: game.status.clone(),
        players: game
            .players
            .iter()
            .map(|player| PlayerInfo {
                name: player.name.clone(),
                has_picked: player.has_picked,
            })
            .collect(),
    }
}

async fn add_players(
    players: web::Json<Players>,
    game_data: web::Data<Arc<Mutex<SecretSantaGame>>>,
) -> Result<Players, CustomError> {
    let mut game = game_data.lock().unwrap();
    for player in players.names.iter() {
        let result = game.add_player(Player::new(player));
        if result.is_err() {
            return Err(CustomError::ValidationError {
                error: result.unwrap_err(),
            });
        }
    }
    Ok(players.into_inner())
}

async fn start_game(
    game_data: web::Data<Arc<Mutex<SecretSantaGame>>>,
) -> Result<HttpResponse, CustomError> {
    let mut game = game_data.lock().unwrap();
    let result = game.start_game();
    if result.is_err() {
        return Err(CustomError::ValidationError {
            error: result.unwrap_err(),
        });
    }
    Ok(HttpResponse::Ok().json("Game started"))
}

async fn reset_game(game_data: web::Data<Arc<Mutex<SecretSantaGame>>>) -> HttpResponse {
    let mut game = game_data.lock().unwrap();
    game.restart_game();
    HttpResponse::Ok().json("Game restarted")
}

async fn pick_players(
    player_name: web::Path<String>,
    game_data: web::Data<Arc<Mutex<SecretSantaGame>>>,
) -> Result<PickedResponse, CustomError> {
    let mut game = game_data.lock().unwrap();
    let result = game.player_pick(&player_name.into_inner());
    match result {
        Ok(player_name) => Ok(PickedResponse { name: player_name }),
        Err(e) => return Err(CustomError::ValidationError { error: e }),
    }
}

async fn remove_player(
    player_name: web::Path<String>,
    game_data: web::Data<Arc<Mutex<SecretSantaGame>>>,
) -> Result<HttpResponse, CustomError> {
    let mut game = game_data.lock().unwrap();
    let result = game.remove_player(&player_name.into_inner());
    if result.is_err() {
        return Err(CustomError::ValidationError {
            error: result.unwrap_err(),
        });
    }
    Ok(HttpResponse::Ok().json("Player removed"))
}

async fn show_players(game_data: web::Data<Arc<Mutex<SecretSantaGame>>>) -> Players {
    let game = game_data.lock().unwrap();
    let mut players = Vec::new();
    for player in game.players.iter() {
        players.push(player.name.clone());
    }
    Players { names: players }
}

pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("").route(web::get().to(index)))
        .service(web::resource("start-game").route(web::post().to(start_game)))
        .service(web::resource("reset-game").route(web::post().to(reset_game)))
        .service(web::resource("show-players").route(web::get().to(show_players)))
        .service(web::resource("player-pick/{player_name}").route(web::get().to(pick_players)))
        .service(web::resource("remove-player/{player_name}").route(web::post().to(remove_player)))
        .service(web::resource("add-players").route(web::post().to(add_players)));
}

#[cfg(test)]
mod tests {
    use actix_web::{test, App};
    use secret_santa::GameStatus;

    use super::*;

    #[actix_rt::test]
    async fn test_index() {
        let game = SecretSantaGame::new();
        let game_data = Arc::new(Mutex::new(game));
        let secret_santa_game = web::Data::new(game_data);

        let mut app = test::init_service(
            App::new()
                .app_data(secret_santa_game.clone())
                .service(web::scope("/secret-santa").configure(routes)),
        )
        .await;

        let req = test::TestRequest::get().uri("/secret-santa").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());

        let body = test::read_body(resp).await;
        let game_info: SantaGameInfo = serde_json::from_slice(&body).unwrap();
        assert_eq!(game_info.status, GameStatus::NotStarted);
        assert_eq!(game_info.players.len(), 0);
    }

    #[actix_rt::test]
    async fn test_add_players() {
        let game = SecretSantaGame::new();
        let game_data = Arc::new(Mutex::new(game));
        let secret_santa_game = web::Data::new(game_data);

        let mut app = test::init_service(
            App::new()
                .app_data(secret_santa_game.clone())
                .service(web::scope("/secret-santa").configure(routes)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/secret-santa/add-players")
            .set_json(&Players {
                names: vec!["Player1".to_string(), "Player2".to_string()],
            })
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());

        let body = test::read_body(resp).await;
        let players: Players = serde_json::from_slice(&body).unwrap();
        assert_eq!(players.names.len(), 2);
        assert_eq!(players.names[0], "Player1");
        assert_eq!(players.names[1], "Player2");
    }

    #[actix_rt::test]
    async fn test_start_game() {
        let game = SecretSantaGame::new();
        let game_data = Arc::new(Mutex::new(game));
        let secret_santa_game = web::Data::new(game_data);

        let mut app = test::init_service(
            App::new()
                .app_data(secret_santa_game.clone())
                .service(web::scope("/secret-santa").configure(routes)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/secret-santa/add-players")
            .set_json(&Players {
                names: vec!["Player1".to_string(), "Player2".to_string()],
            })
            .to_request();
        let _ = test::call_service(&mut app, req).await;

        let req = test::TestRequest::post()
            .uri("/secret-santa/start-game")
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());
    }

    #[actix_rt::test]
    async fn test_reset_game() {
        let game = SecretSantaGame::new();
        let game_data = Arc::new(Mutex::new(game));
        let secret_santa_game = web::Data::new(game_data);

        let mut app = test::init_service(
            App::new()
                .app_data(secret_santa_game.clone())
                .service(web::scope("/secret-santa").configure(routes)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/secret-santa/add-players")
            .set_json(&Players {
                names: vec!["Player1".to_string(), "Player2".to_string()],
            })
            .to_request();
        let _ = test::call_service(&mut app, req).await;

        let req = test::TestRequest::post()
            .uri("/secret-santa/start-game")
            .to_request();
        let _ = test::call_service(&mut app, req).await;

        let req = test::TestRequest::post()
            .uri("/secret-santa/reset-game")
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());

        let req = test::TestRequest::get().uri("/secret-santa").to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());

        let body = test::read_body(resp).await;
        let santa_game_response: SantaGameInfo = serde_json::from_slice(&body).unwrap();
        assert_eq!(santa_game_response.status, GameStatus::NotStarted);
        assert_eq!(santa_game_response.players.len(), 0);
    }

    #[actix_rt::test]
    async fn test_pick_players() {
        let game = SecretSantaGame::new();
        let game_data = Arc::new(Mutex::new(game));
        let secret_santa_game = web::Data::new(game_data);

        let mut app = test::init_service(
            App::new()
                .app_data(secret_santa_game.clone())
                .service(web::scope("/secret-santa").configure(routes)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/secret-santa/add-players")
            .set_json(&Players {
                names: vec!["Player1".to_string(), "Player2".to_string()],
            })
            .to_request();
        let _ = test::call_service(&mut app, req).await;

        let req = test::TestRequest::post()
            .uri("/secret-santa/start-game")
            .to_request();
        let _ = test::call_service(&mut app, req).await;

        let req = test::TestRequest::get()
            .uri("/secret-santa/player-pick/Player1")
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());

        let body = test::read_body(resp).await;
        let picked: PickedResponse = serde_json::from_slice(&body).unwrap();
        assert_eq!(picked.name, "Player2");
    }

    #[actix_rt::test]
    async fn test_show_players() {
        let game = SecretSantaGame::new();
        let game_data = Arc::new(Mutex::new(game));
        let secret_santa_game = web::Data::new(game_data);

        let mut app = test::init_service(
            App::new()
                .app_data(secret_santa_game.clone())
                .service(web::scope("/secret-santa").configure(routes)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/secret-santa/add-players")
            .set_json(&Players {
                names: vec!["Player1".to_string(), "Player2".to_string()],
            })
            .to_request();
        let _ = test::call_service(&mut app, req).await;

        let req = test::TestRequest::get()
            .uri("/secret-santa/show-players")
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_success());

        let body = test::read_body(resp).await;
        let players: Players = serde_json::from_slice(&body).unwrap();
        assert_eq!(players.names.len(), 2);
        assert_eq!(players.names[0], "Player1");
        assert_eq!(players.names[1], "Player2");
    }

    #[actix_rt::test]
    async fn test_add_players_with_invalid_name() {
        let game = SecretSantaGame::new();
        let game_data = Arc::new(Mutex::new(game));
        let secret_santa_game = web::Data::new(game_data);

        let mut app = test::init_service(
            App::new()
                .app_data(secret_santa_game.clone())
                .service(web::scope("/secret-santa").configure(routes)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/secret-santa/add-players")
            .set_json(&Players {
                names: vec![
                    "Player 1".to_string(),
                    "Player 2".to_string(),
                    "Player 1".to_string(),
                ],
            })
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_client_error());
    }

    #[actix_rt::test]
    async fn test_start_game_with_no_players() {
        let game = SecretSantaGame::new();
        let game_data = Arc::new(Mutex::new(game));
        let secret_santa_game = web::Data::new(game_data);

        let mut app = test::init_service(
            App::new()
                .app_data(secret_santa_game.clone())
                .service(web::scope("/secret-santa").configure(routes)),
        )
        .await;

        let req = test::TestRequest::post()
            .uri("/secret-santa/start-game")
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_client_error());
    }

    #[actix_rt::test]
    async fn test_pick_players_with_no_game_started() {
        let game = SecretSantaGame::new();
        let game_data = Arc::new(Mutex::new(game));
        let secret_santa_game = web::Data::new(game_data);

        let mut app = test::init_service(
            App::new()
                .app_data(secret_santa_game.clone())
                .service(web::scope("/secret-santa").configure(routes)),
        )
        .await;

        let req = test::TestRequest::get()
            .uri("/secret-santa/player-pick/Player1")
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_client_error());
    }

    #[actix_rt::test]
    async fn test_pick_players_with_invalid_player_name() {
        let game = SecretSantaGame::new();
        let game_data = Arc::new(Mutex::new(game));
        let secret_santa_game = web::Data::new(game_data);
        let mut app = test::init_service(
            App::new()
                .app_data(secret_santa_game.clone())
                .service(web::scope("/secret-santa").configure(routes)),
        )
        .await;
        let req = test::TestRequest::post()
            .uri("/secret-santa/add-players")
            .set_json(&Players {
                names: vec!["Player1".to_string(), "Player2".to_string()],
            })
            .to_request();
        let _ = test::call_service(&mut app, req).await;
        let req = test::TestRequest::post()
            .uri("/secret-santa/start-game")
            .to_request();
        let _ = test::call_service(&mut app, req).await;
        let req = test::TestRequest::get()
            .uri("/secret-santa/player-pick/Player3")
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_client_error());
    }

    #[actix_rt::test]
    async fn test_pick_players_with_player_already_picked() {
        let game = SecretSantaGame::new();
        let game_data = Arc::new(Mutex::new(game));
        let secret_santa_game = web::Data::new(game_data);
        let mut app = test::init_service(
            App::new()
                .app_data(secret_santa_game.clone())
                .service(web::scope("/secret-santa").configure(routes)),
        )
        .await;
        let req = test::TestRequest::post()
            .uri("/secret-santa/add-players")
            .set_json(&Players {
                names: vec!["Player1".to_string(), "Player2".to_string()],
            })
            .to_request();
        let _ = test::call_service(&mut app, req).await;
        let req = test::TestRequest::post()
            .uri("/secret-santa/start-game")
            .to_request();
        let _ = test::call_service(&mut app, req).await;
        let req = test::TestRequest::get()
            .uri("/secret-santa/player-pick/Player1")
            .to_request();
        let _ = test::call_service(&mut app, req).await;
        let req = test::TestRequest::get()
            .uri("/secret-santa/player-pick/Player1")
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_client_error());
    }

    #[actix_rt::test]
    async fn test_pick_players_with_game_already_finished() {
        let mut game = SecretSantaGame::new();
        game.add_player(Player::new("Player1")).unwrap();
        game.add_player(Player::new("Player2")).unwrap();
        game.start_game().unwrap();
        game.player_pick("Player1").unwrap();
        game.player_pick("Player2").unwrap();
        let game_data = Arc::new(Mutex::new(game));
        let secret_santa_game = web::Data::new(game_data);
        let mut app = test::init_service(
            App::new()
                .app_data(secret_santa_game.clone())
                .service(web::scope("/secret-santa").configure(routes)),
        )
        .await;
        let req = test::TestRequest::get()
            .uri("/secret-santa/player-pick/Player1")
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_client_error());
    }

    #[actix_rt::test]
    async fn test_pick_players_with_game_not_started() {
        let mut game = SecretSantaGame::new();
        game.add_player(Player::new("Player1")).unwrap();
        game.add_player(Player::new("Player2")).unwrap();
        let game_data = Arc::new(Mutex::new(game));
        let secret_santa_game = web::Data::new(game_data);
        let mut app = test::init_service(
            App::new()
                .app_data(secret_santa_game.clone())
                .service(web::scope("/secret-santa").configure(routes)),
        )
        .await;
        let req = test::TestRequest::get()
            .uri("/secret-santa/player-pick/Player1")
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_client_error());
    }

    #[actix_rt::test]
    async fn test_pick_players_with_game_already_restarted() {
        let mut game = SecretSantaGame::new();
        game.add_player(Player::new("Player1")).unwrap();
        game.add_player(Player::new("Player2")).unwrap();
        game.start_game().unwrap();
        game.restart_game();
        let game_data = Arc::new(Mutex::new(game));
        let secret_santa_game = web::Data::new(game_data);
        let mut app = test::init_service(
            App::new()
                .app_data(secret_santa_game.clone())
                .service(web::scope("/secret-santa").configure(routes)),
        )
        .await;
        let req = test::TestRequest::get()
            .uri("/secret-santa/player-pick/Player1")
            .to_request();
        let resp = test::call_service(&mut app, req).await;
        assert!(resp.status().is_client_error());
    }
}
