fn main() -> std::io::Result<()> {
    let mut results = Vec::new();
    // This range is puzzle input
    for i in 273025..767254 {
        let str_i = i.to_string();
        if digits_always_increasing(&str_i)
            && adjacent_equal_digits_exist(&str_i)
            && str_i.len() == 6
        {
            results.push(i);
        }
    }
    println!("Day 4 part 1 numbers meeting rules: {}", results.len());
    Ok(())
}

fn digits_always_increasing(i: &String) -> bool {
    let mut left_chars: Vec<u32> = i.chars().map(|x| x.to_digit(10).unwrap()).collect();
    let mut right_chars: Vec<u32> = i.chars().map(|x| x.to_digit(10).unwrap()).collect();
    left_chars.pop();
    right_chars.remove(0);
    let mut left_iter = left_chars.iter();
    let mut right_iter = right_chars.iter();
    !left_iter.any(|x| x > right_iter.next().unwrap())
}

fn adjacent_equal_digits_exist(i: &String) -> bool {
    let mut left_chars: Vec<u32> = i.chars().map(|x| x.to_digit(10).unwrap()).collect();
    let mut right_chars: Vec<u32> = i.chars().map(|x| x.to_digit(10).unwrap()).collect();
    left_chars.pop();
    right_chars.remove(0);
    let mut left_iter = left_chars.iter();
    let mut right_iter = right_chars.iter();
    left_iter.any(|x| x == right_iter.next().unwrap())
}
