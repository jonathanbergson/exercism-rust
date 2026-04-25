use time::{PrimitiveDateTime as DateTime, Duration};

// Returns a DateTime one billion seconds after start.
pub fn after(start: DateTime) -> DateTime {
    let one_billion = 1000000000;
    start + Duration::seconds(one_billion)
}
