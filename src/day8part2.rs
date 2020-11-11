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

    let mut finished_layer = layers.pop().unwrap();

    for cur_layer in layers.iter().rev() {
        for (idx, val) in cur_layer.iter().enumerate() {
            match val {
                0 => finished_layer[idx] = 0,
                1 => finished_layer[idx] = 1,
                _ => continue
            }
        }
    }

    let mut start = 0;

    while start < total_pixels {
        let line = finished_layer.iter()
            .skip(start)
            .take(layer_width);
        for i in line {
            print!("{}", i);
        }
        println!("");
        start += layer_width;
    }


    Ok(())
}
