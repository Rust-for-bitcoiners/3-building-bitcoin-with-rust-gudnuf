#![allow(unused)]
use std::{convert::Infallible, fmt::Debug};

pub enum MResult<T, E: Debug> {
    Ok(T),
    Err(E),
}

impl<T, E: Debug> MResult<T, E> {
    pub fn ok(value: T) -> MResult<T, E> {
        MResult::Ok(value)
    }
    // Function to create an Err variant
    pub fn err(error: E) -> MResult<T, E> {
        MResult::Err(error)
    }

    // Method to check if it's an Ok variant
    pub fn is_ok(&self) -> bool {
        match self {
            Self::Ok(_) => true,
            _ => false,
        }
    }

    // Method to check if it's an Err variant
    pub fn is_err(&self) -> bool {
        match self {
            Self::Err(_) => true,
            _ => false,
        }
    }

    // Method to unwrap the Ok value, panics if it's an Err
    pub fn unwrap(self) -> T {
        match self {
            Self::Ok(value) => value,
            Self::Err(e) => panic!("Called unwrap on an Err value: {:?}", e),
        }
    }

    // Method to unwrap the Err value, panics if it's an Ok
    pub fn unwrap_err(self) -> E {
        match self {
            Self::Err(error) => error,
            _ => panic!("Called unwrap_err on an Ok value"),
        }
    }
}

// Add unit tests below
mod tests {
    use super::*;

    #[test]
    fn test_ok() {
        let result: MResult<i32, String> = MResult::ok(10);
        assert!(result.is_ok());
        assert!(!result.is_err());
        assert_eq!(result.unwrap(), 10);
    }

    #[test]
    fn test_err() {
        let err: MResult<i32, String> = MResult::err(String::from("oops"));
        assert!(err.is_err());
        assert!(!err.is_ok());
        assert_eq!(err.unwrap_err(), "oops");
    }
}
