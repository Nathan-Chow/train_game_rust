#[cfg(test)]
mod tests {
    use crate::{multithread::solve, errors::TrainGameError};

    #[test]
    fn test_correct() {
        let solutions = solve(String::from("1234"));
        assert!(solutions.is_ok());
        if let Ok(set) = solutions {
            assert_eq!(set.len(), 57);
        }
    }
    
    #[test]
    fn test_duplicates() {
        let solutions = solve(String::from("2222"));
        assert!(solutions.is_ok());
        if let Ok(set) = solutions {
            assert_eq!(set.len(), 2);
        }
    }

    #[test]
    fn test_short_len() {
        let solutions = solve(String::from("1"));
        assert!(solutions.is_err());
        let err = solutions.err().unwrap();
        assert!(err == TrainGameError::Length);
    }

    #[test]
    fn test_long_len() {
        let solutions = solve(String::from("12345"));
        assert!(solutions.is_err());       
        let err = solutions.err().unwrap();
        assert!(err == TrainGameError::Length);
    }

    #[test]
    fn test_bad_char() {
        let solutions = solve(String::from("123*"));
        assert!(solutions.is_err());
        let err = solutions.err().unwrap();
        assert!(err == TrainGameError::Invalid);
    }
}