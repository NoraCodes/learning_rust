fn take_ownership_sum(v: Vec<i32>) -> i32 {
    let mut sum = 0;
    for value in v {
        sum += value;
    }
    return sum;
}

fn borrow_sum(v: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for value in v {
        sum += *value;
    }
    return sum;
}

fn main() {
    let values: Vec<i32> = vec![1, 2, 3, 4, 5];
    // let sum = borrow_sum(values);
    // ^^ fails due to not borrowed
    let sum = borrow_sum(&values);
    println!("{}: {}", values.len(), sum);
}

