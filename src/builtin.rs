use chrono;

// make them a trait with
// call
// arity
// name

pub fn get_time() -> i64 {
    chrono::Utc::now().timestamp_micros()
}

pub fn print(value: &str) {
    println!("{}", value)
}
