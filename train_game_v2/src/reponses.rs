use std::collections::HashSet;

use serde::{Deserialize, Serialize};

#[derive(Deserialize)]
pub struct TrainPayload {
    pub numbers: String,
}

#[derive(Serialize)]
pub struct ResponseBody {
    pub all_solutions: HashSet<String>,
    pub num_solutions: i32,
}

#[derive(Serialize)]
pub struct ErrorBody {
    pub error_message: String,
}
