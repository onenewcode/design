
fn main() {
    let mut numbers = vec![1, 2, 3, 4, 5, 6, 8, 9, 11, 13, 14, 15];
    
    numbers.drain_filter(|x| *x % 2 == 0);
    // let odds = numbers;
    
    assert_eq!(evens, vec![2, 4, 6, 8, 14]);
    assert_eq!(odds, vec![1, 3, 5, 9, 11, 13, 15]);
}