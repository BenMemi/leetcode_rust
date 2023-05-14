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

pub fn average(salary: Vec<i32>) -> f64 {
    let mut min = salary[0];
    let mut max = 0;
    let mut sum = 0;
    for val in salary.iter() {
        if val > &max {
            max = *val;
            sum = sum + val;
            continue;
        }
        if val < &min {
            min = *val;
            sum = sum + val;
            continue;
        }
        sum = sum + val;
    } 
    ((sum - (min + max)) as f64) / ((salary.len() - 2) as f64)
}
