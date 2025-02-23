mod fen;
mod pieces;
mod position;
mod bitboard;
mod utils;
mod evaluator;
mod playground;
mod board_navigator;

#[macro_use] extern crate rocket;
use rocket::serde::{Serialize, Deserialize, json::Json};
use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

use crate::evaluator::evaluate;

pub struct CORS;

#[rocket::async_trait]
impl Fairing for CORS {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new("Access-Control-Allow-Methods", "POST, GET, PATCH, OPTIONS"));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
}

#[get("/play")]
fn play() -> &'static str {
    playground::play();
    "OK"
}

#[derive(Deserialize)]
#[serde(crate = "rocket::serde")]
struct RequestPayload<'r> {
    fen: &'r str,
}

#[derive(Serialize)]
#[serde(crate = "rocket::serde")]
struct ResponsePayload {
    evaluation: f32,
}

#[post("/evaluate", data = "<fen>")]
fn evaluate_post(fen: Json<RequestPayload>) -> Json<ResponsePayload> {
    Json(ResponsePayload {
        evaluation: evaluate(fen.fen, 1)
    })
}

#[options("/evaluate")]
fn evaluate_options() { }

#[launch]
fn rocket() -> _ {
    rocket::build().attach(CORS).mount("/", routes![hello, play, evaluate_post, evaluate_options])
}
