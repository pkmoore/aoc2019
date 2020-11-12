use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;

use std::collections::HashSet;
use std::f64::consts::PI;

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

    let mut ang_dist = asteroids.iter()
        .filter(|&x| x != best.0)
        .map(|x| (*x, get_angle_from_neg_x_axis(*best.0, *x), get_distance(*best.0, *x)))
        .collect::<Vec<((i64, i64), f64, f64)>>();
    //Sort by angle then by distance
    ang_dist.sort_by(|x, y| x.1.partial_cmp(&y.1).unwrap()
                    .then(x.2.partial_cmp(&y.2).unwrap()));

    let closest_to_up_idx = ang_dist.iter().position(|x| x.1 >= 90.0).unwrap();

    // rotate vector until the elements at the front are closest to 90 degrees
    ang_dist.rotate_left(closest_to_up_idx);

    let mut vaporized_order: Vec<((i64, i64), f64, f64)> = Vec::new();
    let mut index;
    let mut delete_list:Vec<usize> = Vec::new();

    while vaporized_order.len() < ang_dist.len() {
        index = 0;
        while index < ang_dist.len()-1 {
            // vaporize current asteroid
            let prev = ang_dist[index];
            let mut next = ang_dist[index+1];
            vaporized_order.push(prev);

            // note that we must actually remove it after this pass
            delete_list.push(index);

            // if the next asteroid has the same angle as the current one,
            // keep advancing until we find the first asteroid with a differing
            // angle.
            // This is to handle cases where you have a line of asteroids at
            // a particular angle (e.g. 90 deg) and you can only get the closest
            // one this pass
            if next.1 == prev.1 {
                while next.1 == prev.1 {
                    index += 1;
                    next = ang_dist[index];
                }
            // otherwise, we just advance one asteroid
            } else {
                index += 1;
            }
        }
        // Delete the elements we moved to the vaporize list from back to front
        // in order to clear things up before we start the next pass
        for i in delete_list.iter().rev() {
            ang_dist.remove(*i);
        }
        delete_list.clear();
    }

    println!("{:?}", (vaporized_order[199].0.0 * 100) + vaporized_order[199].0.1);


    Ok(())
}

fn get_angle_from_neg_x_axis(start: (i64, i64), dest: (i64, i64)) -> f64 {
    // Convert coordinates so our starting point is the origin
    let input = ((dest.0 - start.0) as f64, (start.1 - dest.1) as f64);
    // Get the angle going up from the positive x axis to the destination
    let radians = input.1.atan2(input.0);
    let mut degrees = radians * 180.0 / PI;
    // Make negative angles positive
    if degrees < 0.0 {
        degrees += 360.0;
    }
    // convert from angle between positive X and our line its supplement
    // We want to order by supplement rather than the angle itself because
    // that will give us our clockwise sweep rather than a counterclockwise
    // sweep.
    let mut flipped = 180.0 - degrees;
    if flipped < 0.0 {
        flipped += 360.0;
    }
    flipped
}

fn get_distance(start: (i64, i64), dest: (i64, i64)) -> f64 {
    // Standard distance formula
    ((((dest.0 - start.0).pow(2)) + ((dest.1 - start.1).pow(2))) as f64).sqrt()
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
