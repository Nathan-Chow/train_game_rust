pub enum TrainGameError {
    Size,
    Invalid,
}

impl std::fmt::Display for TrainGameError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TrainGameError::Size => {
                write!(f, "Invalid input. Input number must be a 4 digit number.")
            }
            TrainGameError::Invalid => {
                write!(f, "Invalid input. Input number must only contain numbers.")
            }
        }
    }
}
