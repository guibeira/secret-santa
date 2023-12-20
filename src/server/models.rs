use actix_web::Responder;
use secret_santa::GameStatus;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct SantaGameInfo {
    pub status: GameStatus,
    pub players: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Players {
    pub names: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct PickedResponse {
    pub name: String,
}

impl Responder for PickedResponse {
    type Body = actix_web::body::BoxBody;
    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse {
        let body = serde_json::to_string(&self).unwrap();
        actix_web::HttpResponse::Ok()
            .content_type("application/json")
            .body(body)
    }
}

impl Responder for SantaGameInfo {
    type Body = actix_web::body::BoxBody;
    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse {
        let body = serde_json::to_string(&self).unwrap();
        actix_web::HttpResponse::Ok()
            .content_type("application/json")
            .body(body)
    }
}
impl Responder for Players {
    type Body = actix_web::body::BoxBody;
    fn respond_to(self, _: &actix_web::HttpRequest) -> actix_web::HttpResponse {
        let body = serde_json::to_string(&self).unwrap();
        actix_web::HttpResponse::Ok()
            .content_type("application/json")
            .body(body)
    }
}
