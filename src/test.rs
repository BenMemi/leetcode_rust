#[test]
fn test_p1523() {
    assert_eq!(crate::count_odds(1, 3), 2);
    assert_eq!(crate::count_odds(5, 10), 3);
}

#[test]
fn test_average() {
    assert_eq!(crate::average(vec![1, 2, 3]), 2 as f64);
    assert_eq!(crate::average(vec![1i32, 2, 3, 4, 5]), 3 as f64); 
}
