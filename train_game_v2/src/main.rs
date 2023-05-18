pub mod calcs;
pub mod errors;
pub mod multithread;
pub mod operations;
pub mod reponses;
mod test;

use crate::{multithread::solve, reponses::{TrainPayload, ErrorBody, ResponseBody}};

use actix_web::{
    http::StatusCode,
    post,
    web::{self, ServiceConfig},
    HttpResponse,
};
use shuttle_actix_web::ShuttleActixWeb;

#[post("/train_game")]
async fn train_game(payload: web::Json<TrainPayload>) -> HttpResponse {
    let number = payload.numbers.clone();

    let all_solutions = match solve(number) {
        Ok(set) => set,
        Err(e) => {
            let error_repsonse = ErrorBody {
                error_message: e.to_string(),
            };
            return HttpResponse::build(StatusCode::BAD_REQUEST).json(error_repsonse);
        }
    };

    let num_solutions = all_solutions.len() as i32;
    let response_body = ResponseBody {
        all_solutions,
        num_solutions,
    };

    HttpResponse::Ok().json(response_body)
}

#[shuttle_runtime::main]
async fn actix_web() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(train_game);
    };

    Ok(config.into())
}
