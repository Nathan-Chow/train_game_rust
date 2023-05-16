use std::{thread, sync::mpsc::channel, collections::HashSet};
use itertools::Itertools;

use crate::{calcs::{generate_combinations, calculate}, errors::TrainGameError};

#[derive(Debug, Clone, Copy)]
pub enum Operations {
    Add,
    Sub,
    Mul,
    Div
}

impl std::fmt::Display for Operations {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Operations::Add => write!(f, "+"),
            Operations::Sub => write!(f, "-"),
            Operations::Mul => write!(f, "*"),
            Operations::Div => write!(f, "/"),
        }
    }
}

pub fn solve(digit_string: String) -> Result<HashSet<String>, TrainGameError>{
    let characters = vec![Operations::Add, Operations::Sub, Operations::Mul, Operations::Div];

    let combination_length = 3;

    let operations = generate_combinations(&characters, combination_length, Vec::new());

    let digit_values = match string_to_digit(digit_string) {
        Ok(digit_vec) => digit_vec,
        Err(e) => return Err(e)
    };
        
    let digits_operators: Vec<(Vec<i32>, Vec<Operations>)> = digit_values.into_iter()
        .permutations(4)
        .cartesian_product(operations.into_iter())
        .collect();

    let length = digits_operators.len();

    let chunk_size = length / 6;
    let chunk_vec = digits_operators.chunks(chunk_size);
    
    let (tx_solutions, rx_solutions) = channel();

    thread::scope(|s| {
        let mut vec_threads = vec![];
        for v in chunk_vec {
            let tx_solutions = tx_solutions.clone();
            let join_handler = s.spawn(move || {
                for (digit, operation) in v {
                    if let Ok(Some(solution)) = calculate(digit.to_owned(), operation.to_owned()) {
                        tx_solutions.send(solution).unwrap();
                    }
                }
            });
            vec_threads.push(join_handler);
        }
        for handler in vec_threads {
            handler.join().unwrap();
        }
    });
    drop(tx_solutions);
    
    let mut solutions_set = HashSet::new();
    while let Ok(solution) = rx_solutions.recv() {
        solutions_set.insert(solution);
    }

    Ok(solutions_set)
}

fn string_to_digit(digit_string: String) -> Result<Vec<i32>, TrainGameError> {
    let digit_chars = digit_string.chars();
    let mut digit_vec = vec![];
    for num in digit_chars {
        if let Some(digit) = num.to_digit(10) {
            digit_vec.push(digit as i32);
        }
        else {
            return Err(TrainGameError::Invalid);
        }
    }
    Ok(digit_vec)
}