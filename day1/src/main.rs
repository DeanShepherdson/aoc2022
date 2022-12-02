extern crate sort;
use std::fs;

use sort::insertion_sort;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file.");

    let mut max = vec![0, 0, 0];
    let mut cals = 0;
    for line in contents.lines() {
        if line.is_empty() {
                if cals > max[0] {
                    max[0] = cals;
                }
                insertion_sort(&mut max);
            cals = 0;
        } else {
            let current_cals: i32 = line.parse().unwrap();
            cals = cals + current_cals
        }
    }

    let sum: i32 = max.iter().sum();
    println!("Max calories on top 3 elves: {}", sum);
}
