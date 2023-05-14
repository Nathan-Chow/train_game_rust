use std::{thread, sync::mpsc::channel};
use itertools::Itertools;

use crate::calcs::{generate_combinations, calculate};

pub fn solve(digits: String) -> Vec<String> {
    let characters: Vec<char> = vec!['+', '-', '*', '/'];

    let combination_length = 3;

    let operations = generate_combinations(&characters, combination_length, Vec::new());

    let digits_operators: Vec<(Vec<i32>, Vec<char>)> = digits
        .chars()
        .map(|x| x.to_digit(10).unwrap() as i32)
        .permutations(4)
        .cartesian_product(operations.into_iter())
        .collect();

    let length = digits_operators.len();

    let chunk_size = length / 6;
    let chunk_vec = digits_operators.chunks(chunk_size);
    
    let (tx_solutions, rx_solutions) = channel();
    let (tx_count, rx_count) = channel();

    thread::scope(|s| {
        let mut vec_threads = vec![];
        for v in chunk_vec {
            let tx_count = tx_count.clone();
            let tx_solutions = tx_solutions.clone();
            let join_handler = s.spawn(move || {
                let mut count = 0;
                for (digit, operation) in v {
                    if let Ok(valid) = calculate(digit.to_owned(), operation.to_owned()) {
                        if let Some(solution) = valid {
                            tx_solutions.send(solution).unwrap();
                            count += 1;
                        }
                    }
                }
                tx_count.send(count).unwrap();
            });
            vec_threads.push(join_handler);
        }
        for handler in vec_threads {
            handler.join().unwrap();
        }
    });
    drop(tx_count);
    drop(tx_solutions);
    
    let mut total = 0;
    while let Ok(count) = rx_count.recv() {
        total += count;
    }

    let mut solutions_vec = vec![];
    while let Ok(solution) = rx_solutions.recv() {
        solutions_vec.push(solution);
    }

    println!("There are {total} total combinations.");

    solutions_vec
}

