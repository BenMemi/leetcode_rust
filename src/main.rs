mod test;

fn main() {
    print!("Hello World!")
}

//p1523:Count Odd Numbers in an Interval Range
pub fn count_odds(low: i32, high: i32) -> i32 {
    let d = (high - low) + 1;
    if d % 2 == 0 {
        return d / 2;
    }
    if low % 2 == 0 {
        return d / 2;
    }
    (d / 2) + 1
}
