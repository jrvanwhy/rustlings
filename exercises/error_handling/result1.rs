// result1.rs
// Make this test pass! Execute `rustlings hint result1` for hints :)

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        // Alternative solution using match instead of if.
        // match value {
        //     0 => Err(CreationError::Zero),
        //     value if value > 0 => Ok(PositiveNonzeroInteger(value as u64)),
        //     value => Err(CreationError::Negative),
        // }
        if value <= 0 {
            match value {
                0 => return Err(CreationError::Zero),
                _ => return Err(CreationError::Negative),
            }
        }
        Ok(PositiveNonzeroInteger(value as u64))
    }
}

#[test]
fn test_creation() {
    assert!(PositiveNonzeroInteger::new(10).is_ok());
    assert_eq!(
        Err(CreationError::Negative),
        PositiveNonzeroInteger::new(-10)
    );
    assert_eq!(Err(CreationError::Zero), PositiveNonzeroInteger::new(0));
}
