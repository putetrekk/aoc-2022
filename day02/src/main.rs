use std::{fs::File, io::Read, ptr::null};

fn main() {
    let input = read_string_from_file("day02/input.txt");

    let rounds: Vec<Vec<&str>> = input
        .lines()
        .map(|l| l.split_whitespace().collect())
        .collect();

    let mut score: i32 = 0;

    for round in &rounds {
        let opponent_action = round[0];
        let player_action = round[1];

        if player_action == "X" {
            score += 1;
        } else if player_action == "Y" {
            score += 2;
        } else if player_action == "Z" {
            score += 3;
        }

        let outcome = get_outcome(player_action, opponent_action);
        let outcome_str = outcome.as_str();

        match outcome_str {
            "WIN" => {
                score += 6;
            }
            "LOSS" => {
                score += 0;
            }
            "DRAW" => {
                score += 3;
            }
            _ => panic!(),
        }
    }

    println!("score: {}", score);

    score = 0;

    for round in &rounds {
        let opponent_action = round[0];
        let mut player_action: &str = "X";
        let desired_outcome = round[1];
        let desired_outcome_word: String = outcome_letter_as_word(desired_outcome);
        let desired_outcome_word_str = desired_outcome_word.as_str();

        for action in ["X", "Y", "Z"] {
            player_action = action;
            let predicted_outcome = get_outcome(player_action, opponent_action);
            let predicted_outcome_str = predicted_outcome.as_str();
            if predicted_outcome_str == desired_outcome_word_str {
                break;
            }
        }

        if player_action == "X" {
            score += 1;
        } else if player_action == "Y" {
            score += 2;
        } else if player_action == "Z" {
            score += 3;
        }

        let outcome = get_outcome(player_action, opponent_action);
        let outcome_str = outcome.as_str();

        match outcome_str {
            "WIN" => {
                score += 6;
            }
            "LOSS" => {
                score += 0;
            }
            "DRAW" => {
                score += 3;
            }
            _ => panic!(),
        }
    }

    println!("score: {}", score);
}

fn outcome_letter_as_word(desired_outcome: &str) -> String {
    match desired_outcome {
        "X" => {
            return String::from("LOSS");
        }
        "Y" => {
            return String::from("DRAW");
        }
        "Z" => {
            return String::from("WIN");
        }
        _ => panic!(),
    }
}

fn get_outcome(player_action: &str, opponent_action: &str) -> String {
    if player_action == "X" {
        if opponent_action == "A" {
            return String::from("DRAW");
        }
        if opponent_action == "B" {
            return String::from("LOSS");
        }
        if opponent_action == "C" {
            return String::from("WIN");
        }
    }
    if player_action == "Y" {
        if opponent_action == "A" {
            return String::from("WIN");
        }
        if opponent_action == "B" {
            return String::from("DRAW");
        }
        if opponent_action == "C" {
            return String::from("LOSS");
        }
    }
    if player_action == "Z" {
        if opponent_action == "A" {
            return String::from("LOSS");
        }
        if opponent_action == "B" {
            return String::from("WIN");
        }
        if opponent_action == "C" {
            return String::from("DRAW");
        }
    }
    println!(
        "could not determine outcome for {} {}",
        opponent_action, player_action
    );
    panic!()
}

fn read_string_from_file(filename: &str) -> String {
    let mut file: File = File::open(filename).expect("Can't open file!");

    let mut contents = String::new();
    file.read_to_string(&mut contents)
        .expect("Can't read file contents...");

    return contents;
}
