use crate::multithread::Operations;

pub fn generate_combinations(
    characters: &[Operations],
    combination_length: usize,
    prefix: Vec<Operations>,
) -> Vec<Vec<Operations>> {
    if prefix.len() == combination_length {
        return vec![prefix.into_iter().collect::<Vec<Operations>>()];
    }

    let mut combinations = Vec::new();

    for &character in characters {
        let mut new_prefix = prefix.clone();
        new_prefix.push(character);
        combinations.append(&mut generate_combinations(
            characters,
            combination_length,
            new_prefix,
        ));
    }

    combinations
}

pub fn calculate(digits: Vec<i32>, operators: Vec<Operations>) -> Result<Option<String>, ()> {
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
        return Ok(Some(format!(
            "{} {} {} {} {} {} {} = 10",
            num1, op1, num2, op2, num3, op3, num4
        )));
    }

    Ok(None)
}

fn operate(num1: i32, num2: i32, op: Operations) -> Result<i32, ()> {
    match op {
        Operations::Add => Ok(num1 + num2),
        Operations::Sub => Ok(num1 - num2),
        Operations::Mul => Ok(num1 * num2),
        Operations::Div => num1.checked_div(num2).ok_or(()),
    }
}
