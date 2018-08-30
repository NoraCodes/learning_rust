// fn cap_values_takes_ownership(max: i32, v: Vec<i32>) -> Vec<i32> {
fn cap_values_takes_ownership(max: i32, mut v: Vec<i32>) -> Vec<i32> {
    for index in 0..v.len() {
        if v[index] > max {
            v[index] = max
        }
    }

    return v;
}


fn main() {
    let mut values = vec![1, 2, 5, 7, 1024, 6];

    let values = cap_values_takes_ownership(10, values);

    for v in values {
        println!("{}", v);
    }
}

