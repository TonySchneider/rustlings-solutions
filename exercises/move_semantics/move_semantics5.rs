// move_semantics5.rs
//
// Make me compile only by reordering the lines in `main()`, but without adding,
// changing or removing any of them.
//
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand
// for a hint.

#[test]
fn main() {
    let mut x = 100;
    let y = &mut x;
    *y += 100;
    // We can't define two mutable variables by using references because we want to avoid changing the value of X simultaneously.
    // Therefore, we must use 'y' before we borrow 'x' a second time.
    let z = &mut x;
    *z += 1000;
    assert_eq!(x, 1200);
}
