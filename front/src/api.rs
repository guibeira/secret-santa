use reqwasm::http::Request;
use gloo::console::log;
use web_sys::window;

use crate::app::PlayersCreate;

#[derive(Debug, Clone)]
pub struct Api {
    url: String,
}

impl Api {
    pub fn new() -> Self {
        let mut url = window()
            .unwrap()
            .location()
            .href()
            .unwrap_or_else(|_| "unknown".to_string());

        if cfg!(debug_assertions) {
            log!("Running on debug mode");
            url = "http://localhost:8080/".to_string();
        } else {
            log!("Running on release mode");
        }
        log!(format!("using url: {}", url));
        let url = url + "secret-santa";
        Api { url }
    }
    pub async fn remove_player(
        &self,
        name: &String,
    ) -> Result<reqwasm::http::Response, reqwasm::Error> {
        let url = format!("{}/remove-player/{}", self.url, name);
        Request::post(&url)
            .header("Content-Type", "application/json")
            .send()
            .await
    }

    pub async fn start_game(&self) -> Result<reqwasm::http::Response, reqwasm::Error> {
        let url = format!("{}/start-game", self.url);
        Request::post(&url).send().await
    }

    pub async fn info(&self) -> Result<reqwasm::http::Response, reqwasm::Error> {
        Request::get(&self.url).send().await
    }

    pub async fn reset_game(&self) -> Result<reqwasm::http::Response, reqwasm::Error> {
        let url = format!("{}/reset-game", self.url);
        Request::post(&url).send().await
    }

    pub async fn pick_player(
        &self,
        name: &String,
    ) -> Result<reqwasm::http::Response, reqwasm::Error> {
        let url = format!("{}/player-pick/{}", self.url, name);
        Request::get(&url)
            .header("Content-Type", "application/json")
            .send()
            .await
    }

    pub async fn add_player(
        &self,
        name: &String,
    ) -> Result<reqwasm::http::Response, reqwasm::Error> {
        let players = Vec::from([name.clone()]);
        let players_create = PlayersCreate { names: players };
        let players_create = serde_json::to_string(&players_create).unwrap();
        let url = format!("{}/add-players", self.url);
        Request::post(&url)
            .header("Content-Type", "application/json")
            .body(players_create)
            .send()
            .await
    }
}
