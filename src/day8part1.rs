use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

fn main() -> std::io::Result<()> {
    let file = File::open("./data/day8.txt")?;
    let mut reader = BufReader::new(file);
    let mut data = String::new();

    reader.read_line(&mut data)?;
    data.pop();

    let layer_width: usize = 25;
    let layer_height: usize = 6;
    let total_pixels: usize = layer_width * layer_height;
    assert!(data.len() % total_pixels == 0);

    let mut nums = data
        .chars()
        .map(|x| x.to_digit(10).unwrap())
        .collect::<Vec<u32>>();

    let mut layers: Vec<Vec<u32>> = Vec::new();
    while nums.len() > 0 {
        layers.insert(0, nums.split_off(nums.len() - total_pixels as usize));
    }

    let fewest_zeroes = layers
        .iter()
        .min_by(|x, y| {
            x.iter()
                .filter(|x| **x == 0)
                .count()
                .cmp(&y.iter().filter(|x| **x == 0).count())
        })
        .unwrap();


    let one_digits = fewest_zeroes.iter().filter(|x| **x == 1).count();
    let two_digits = fewest_zeroes.iter().filter(|x| **x == 2).count();
    println!("Answer: {}", one_digits * two_digits);

    Ok(())
}
