pub fn checked_int_div(a: i64, b: i64) -> Option<i64> {
    if b == 0 || a % b != 0 {
        None
    } else {
        Some(a / b)
    }
}
