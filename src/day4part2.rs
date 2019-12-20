fn main() -> std::io::Result<()> {
    let mut results = Vec::new();
    // This range is puzzle input
    for i in 273025..767253 {
        let str_i = i.to_string();
        if str_i.len() == 6
            && digits_always_increasing(&str_i)
            && adjacent_equal_digits_exist(&str_i)
        {
            results.push(i);
        }
    }
    println!("Day 4 part 2 numbers meeting rules: {}", results.len());
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
    let digits: Vec<u32> = i.chars().map(|x| x.to_digit(10).unwrap()).collect();
    let mut i = 0;
    let mut found;
    while i < digits.len() {
        if (i + 1 < digits.len()) && digits.get(i) == digits.get(i + 1) {
            // We might have found a double
            found = true;
            if (i + 2 < digits.len()) && digits.get(i) == digits.get(i + 2) {
                // This double is bad because it is at least a triple
                found = false;
            }
            if (i > 0) && digits.get(i) == digits.get(i - 1) {
                // Make sure we aren't a triple looking back as well
                found = false;
            }
            if found {
                return true;
            }
        }
        i += 1;
    }
    // We reached the end of the number without satisfying the criteria
    false
}
