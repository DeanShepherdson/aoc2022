use std::fs;
use std::collections::HashMap;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Should have been able to read the file.");

    let mut score_map = HashMap::new();
    score_map.insert(String::from("X"), 1);
    score_map.insert(String::from("Y"), 2);
    score_map.insert(String::from("Z"), 3);

    let mut draw_map = HashMap::new();
    draw_map.insert(String::from("A"), String::from("X"));
    draw_map.insert(String::from("B"), String::from("Y"));
    draw_map.insert(String::from("C"), String::from("Z"));

    let mut win_map = HashMap::new();
    win_map.insert(String::from("A"), String::from("Y"));
    win_map.insert(String::from("B"), String::from("Z"));
    win_map.insert(String::from("C"), String::from("X"));

    let mut total_score = 0;
    for line in contents.lines(){
        let result = line.split(" ").collect::<Vec<&str>>();
        let opp_play = result[0];
        let me_play = result[1];
        let option_score = score_map[me_play];

        let win_result = if draw_map[opp_play].eq(me_play) {
            3
        } else {
            if win_map[opp_play].eq(me_play) {
                6
            } else {
                0
            }
        };

        total_score += option_score + win_result;
    }

    println!("Part 1: your total score is {}", total_score);
    total_score = 0;

    // Part 2
    let mut outcome_map = HashMap::new();
    outcome_map.insert(String::from("X"), 0);
    outcome_map.insert(String::from("Y"), 3);
    outcome_map.insert(String::from("Z"), 6);

    let mut lose_map = HashMap::new();
    lose_map.insert(String::from("A"), String::from("Z"));
    lose_map.insert(String::from("B"), String::from("X"));
    lose_map.insert(String::from("C"), String::from("Y"));

    let mut reverse_option_map = HashMap::new();
    reverse_option_map.insert(String::from("X"), lose_map);
    reverse_option_map.insert(String::from("Y"), draw_map);
    reverse_option_map.insert(String::from("Z"), win_map);

    for line in contents.lines(){
        let result = line.split(" ").collect::<Vec<&str>>();
        let opp_play = result[0];
        let outcome = result[1];

        let win_result = outcome_map[outcome];
        let option_score = score_map[&reverse_option_map[outcome][opp_play]];
        total_score += option_score + win_result;
    }

    println!("Part 2: your total score is {}", total_score);

}

