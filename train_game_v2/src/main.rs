pub mod multithread;
pub mod calcs;
pub mod errors;

use std::collections::HashSet;

use crate::multithread::solve;

use actix_web::{post, web::{ServiceConfig, self}, HttpResponse, http::StatusCode};
use shuttle_actix_web::ShuttleActixWeb;
use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
struct TrainPayload {
    numbers: String
}

#[derive(Serialize)]
struct ResponseBody {
    all_solutions: HashSet<String>,
    num_solutions: i32,
}

#[post("/train_game")]
async fn train_game(payload: web::Json<TrainPayload>) -> HttpResponse {
    let number = payload.numbers.clone();

    let all_solutions = match solve(number) {
        Ok(set) => set,
        Err(_) => return HttpResponse::build(StatusCode::BAD_REQUEST).finish()
    };

    let num_solutions = all_solutions.len() as i32;
    let response_body = ResponseBody {
        all_solutions,
        num_solutions
    };
    
    HttpResponse::Ok().json(response_body)
}

#[shuttle_runtime::main]
async fn actix_web(
) -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(train_game);
    };

    Ok(config.into())
}