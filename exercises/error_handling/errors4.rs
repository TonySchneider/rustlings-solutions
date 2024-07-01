// errors4.rs
//
// Execute `rustlings hint errors4` or use the `hint` watch subcommand for a
// hint.

#[derive(PartialEq, Debug)]
struct PositiveNonzeroInteger(u64);

#[derive(PartialEq, Debug)]
enum CreationError {
    Negative,
    Zero,
}

impl PositiveNonzeroInteger {
    fn new(value: i64) -> Result<PositiveNonzeroInteger, CreationError> {
        u64::try_from(value)
            .map_err(|_| CreationError::Negative) // custom the error object. this returns Err but we want to define our custom Error.
            .and_then(|num| {
                // "and_then" handles the Ok value
                if num == 0 {
                    Err(CreationError::Zero)
                } else {
                    Ok(PositiveNonzeroInteger(num))
                }
            })
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
