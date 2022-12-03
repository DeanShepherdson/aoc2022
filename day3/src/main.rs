extern crate array_tool;
use std::fs;
use std::collections::HashSet;

use array_tool::vec::*;

fn get_common_char(a: &str, b: &str) -> char {
    let seta: HashSet<char> = a.chars().collect();
    let setb: HashSet<char> = b.chars().collect();
    let mut intersection = seta.intersection(&setb);

    match intersection.next() {
        Some(x) => *x,
        None => 'a'
    }
}

fn get_common_badge(a: &str, b: &str, c: &str) -> char {
    let va: Vec<char> = a.chars().collect();
    let vb: Vec<char> = b.chars().collect();
    let vc: Vec<char> = c.chars().collect();

    let vab = va.intersect(vb);
    let vac = va.intersect(vc);

    let intersection = vac.intersect(vab);

    intersection[0]
}

fn get_char_value(x: &char) -> u32 {
        if x.is_ascii_uppercase() {
            (x.to_ascii_lowercase() as u32) - ('a' as u32) + 27
        } else {
            (*x as u32) - ('a' as u32) + 1
        }
}

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file.");

    let mut sum = 0;
    for line in contents.lines() {
        let strs = line.split_at(line.len() / 2);
        let x = get_common_char(strs.0, strs.1);
        println!("Common char is: {}", x);

        sum += get_char_value(&x);
    }

    println!("Part 1: the sum is {}", sum);

    // Part 2
    sum = 0;
    let mut counter = 0;
    let mut strs = ["", "", ""];
    for line in contents.lines() {
        strs[counter % 3] = line;
        counter += 1;
        if counter > 0 && counter % 3 == 0 {
            println!("common: {}", get_common_badge(strs[0], strs[1], strs[2]));
            sum += get_char_value(&get_common_badge(strs[0], strs[1], strs[2]))
        }
    }

    println!("Part 2: the sum is {}", sum);
}
