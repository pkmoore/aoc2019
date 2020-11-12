use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use std::collections::HashSet;

fn main() -> std::io::Result<()> {
    let file = File::open("./data/day10.txt")?;
    let mut reader = BufReader::new(file);
    let mut data = String::new();

    let mut grid: Vec<Vec<char>> = Vec::new();
    while reader.read_line(&mut data)? > 0 {
        data.pop();
        grid.push(data.chars().collect::<Vec<char>>());
        data.clear();
    }
    let asteroids = find_asteroids(&grid);
    let best = asteroids
        .iter()
        .map(|x| (x, count_visible(*x, &asteroids)))
        .max_by(|x, y| x.1.cmp(&y.1))
        .unwrap();
    println!("Best: {:?}", best);

    Ok(())
}

fn count_visible(one: (i64, i64), asteroids: &HashSet<(i64, i64)>) -> usize {
    asteroids
        .iter()
        .map(|&x| can_see(one, x, asteroids))
        .filter(|&x| x == true)
        .count()
}

fn can_see(one: (i64, i64), two: (i64, i64), asteroids: &HashSet<(i64, i64)>) -> bool {
    // Asteroid can't see itself
    if one == two {
        return false;
    }
    let slope = slope_between_asteroids(one, two);
    // starting point x + slope run
    // starting point y + slope rise
    let mut next_point = ((one.0 + slope.1), (one.1 + slope.0));
    //While we haven't reached our destination point
    while next_point != two {
        // if the point we're checking is in the set of those with asteroids
        // we return false
        if asteroids.contains(&next_point) {
            return false;
        }
        next_point = ((next_point.0 + slope.1), (next_point.1 + slope.0));
    }
    // We've reached our destination point without hitting any asteroids
    // so we return true
    true
}

// Euclid's two-thousand-year-old algorithm for finding the greatest common
// divisor. From Rust docs.  Can return negative GCDs
fn gcd(x: i64, y: i64) -> i64 {
    let mut x = x;
    let mut y = y;
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    x
}

fn slope_between_asteroids(one: (i64, i64), two: (i64, i64)) -> (i64, i64) {
    let rise = two.1 - one.1;
    let run = two.0 - one.0;
    // Make sure we don't have a negative GCD so we can have negative slopes
    // where appropriate
    let gcd = gcd(rise, run).abs();

    (rise / gcd as i64, run / gcd as i64)
}

fn find_asteroids(grid: &Vec<Vec<char>>) -> HashSet<(i64, i64)> {
    let mut asteroids: HashSet<(i64, i64)> = HashSet::new();
    for (y_coord, row) in grid.iter().enumerate() {
        for (x_coord, item) in row.iter().enumerate() {
            if *item == '#' {
                asteroids.insert((x_coord as i64, y_coord as i64));
            }
        }
    }

    asteroids
}
