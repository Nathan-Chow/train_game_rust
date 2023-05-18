use itertools::Itertools;
use std::{collections::HashSet, sync::mpsc::channel, thread};

use crate::{
    calcs::{calculate, generate_combinations},
    errors::TrainGameError, operations::Operations,
};

pub fn solve(digit_string: String) -> Result<HashSet<String>, TrainGameError> {
    if digit_string.len() != 4 {
        return Err(TrainGameError::Length)
    }

    let operators = vec![
        Operations::Add,
        Operations::Sub,
        Operations::Mul,
        Operations::Div,
    ];

    let combination_length = 3;

    let operations = generate_combinations(&operators, combination_length, Vec::new());

    let digit_values = match string_to_digit(digit_string) {
        Ok(digit_vec) => digit_vec,
        Err(e) => return Err(e),
    };

    let chunk_vec = chunking_data(digit_values, operations);

    let (tx_solutions, rx_solutions) = channel();

    thread::scope(|s| {
        for v in chunk_vec {
            let tx_solutions = tx_solutions.clone();
            s.spawn(move || {
                for (digit, operation) in v {
                    if let Ok(Some(solution)) = calculate(digit.to_owned(), operation.to_owned()) {
                        let _ = tx_solutions.send(solution);
                    }
                }
            });
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
        } else {
            return Err(TrainGameError::Invalid);
        }
    }
    Ok(digit_vec)
}

fn chunking_data(digit_values: Vec<i32>, operations: Vec<Vec<Operations>>) -> Vec<Vec<(Vec<i32>, Vec<Operations>)>> {
    let digits_operators: Vec<(Vec<i32>, Vec<Operations>)> = digit_values
        .into_iter()
        .permutations(4)
        .cartesian_product(operations.into_iter())
        .collect();
    
    let length = digits_operators.len();
    let chunk_size = length / 6;
    let chunk_vec: Vec<Vec<(Vec<i32>, Vec<Operations>)>> = digits_operators.into_iter()
        .chunks(chunk_size)
        .into_iter()
        .map(|chunk| chunk.collect())
        .collect();
    chunk_vec
}