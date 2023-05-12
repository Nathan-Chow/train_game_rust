use std::{thread, sync::mpsc::channel};

use itertools::Itertools;

fn main() {
    let characters: Vec<char> = "+-*/"
        .chars()
        .collect(); // Characters to consider

    let combination_length = 3;

    let operations = generate_combinations(&characters, combination_length, Vec::new());

    // println!("{:?} | {}", operations, operations.len());

    let digits_operators: Vec<(Vec<i32>, Vec<char>)> = std::env::args()
        .nth(1)
        .unwrap()
        .chars()
        .map(|x| x.to_digit(10).unwrap() as i32)
        .permutations(4)
        .cartesian_product(operations.into_iter())
        .collect();

    // println!("{:?}", digits_operators);

    let length = digits_operators.len();

    let chunk_size = length / 6;
    let chunk_vec = digits_operators.chunks(chunk_size);
    
    let (tx, rx) = channel();

    thread::scope(|s| {
        let mut vec_threads = vec![];
        for v in chunk_vec {
            let tx = tx.clone();
            let join_handler = s.spawn(move || {
                let mut count = 0;
                for (digit, operation) in v {
                    if let Ok(true) = calculate(digit.to_owned(), operation.to_owned()) {
                        count += 1;
                    }
                }
                tx.send(count).unwrap();
            });
            vec_threads.push(join_handler);
        }
        for handler in vec_threads {
            handler.join().unwrap();
        }
    });

    let mut total = 0;
    for _ in 0..6 {
        total += rx.recv().unwrap();
    }
    println!("There are {total} total combinations.");
    
}

fn generate_combinations(characters: &[char], combination_length: usize, prefix: Vec<char>) -> Vec<Vec<char>> {
    if prefix.len() == combination_length {
        return vec![prefix.into_iter().collect::<Vec<char>>()]; // Return the combination as a vector
    }

    let mut combinations = Vec::new();

    for &character in characters {
        let mut new_prefix = prefix.clone();
        new_prefix.push(character);
        combinations.append(&mut generate_combinations(characters, combination_length, new_prefix));
    }

    combinations
}

fn calculate(digits: Vec<i32>, operators: Vec<char>) -> Result<bool, ()> {
    let num1 = digits[0];
    let num2 = digits[1];
    let num3 = digits[2];
    let num4 = digits[3];

    let op1 = operators[0];
    let op2 = operators[1];
    let op3 = operators[2];

    let result = operate(num1, num2, op1)?;
    let result = operate(result, num3, op2)?;
    let result = operate(result, num4, op3)?;

    if result == 10 {
        println!(
            "{} {} {} {} {} {} {} = 10",
            num1, op1, num2, op2, num3, op3, num4
        );
        return Ok(true);
    }

    Ok(false)
}

fn operate(num1: i32, num2: i32, op: char) -> Result<i32, ()> {
    match op {
        '+' => Ok(num1 + num2),
        '-' => Ok(num1 - num2),
        '*' => Ok(num1 * num2),
        '/' => num1.checked_div(num2).ok_or(()),
        _ => panic!("Invalid operation"),
    }
}