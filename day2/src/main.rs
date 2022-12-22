use std::fs::File;
use std::io::Read;

#[derive(Copy, Clone)]
enum Shoot {
    Rock,
    Paper,
    Scissor
}

enum Result {
    Lose,
    Tie,
    Win,
}

fn score(opp_hand: Shoot, your_hand: Shoot) -> u16{
    let result_val = match (&opp_hand, &your_hand) {
        (Shoot::Rock, Shoot::Paper) => 6,
        (Shoot::Paper, Shoot::Rock) => 0,
        (Shoot::Scissor, Shoot::Rock) => 6,
        (Shoot::Rock, Shoot::Scissor) => 0,
        (Shoot::Paper, Shoot::Scissor) => 6,
        (Shoot::Scissor, Shoot::Paper) => 0,
        (_, _) => 3
    };
    let your_val = match &your_hand  {
        Shoot::Rock => 1,
        Shoot::Paper => 2,
        Shoot::Scissor => 3,
    };

    result_val + your_val
}

fn find_your_move(opp_hand: &Shoot, result_val: &Result) -> Shoot{
    match (opp_hand, result_val) {
        (Shoot::Rock, Result::Win) => Shoot::Paper,
        (Shoot::Rock, Result::Lose) => Shoot::Scissor,

        (Shoot::Paper, Result::Win) => Shoot::Scissor,
        (Shoot::Paper, Result::Lose) => Shoot::Rock,

        (Shoot::Scissor, Result::Win) => Shoot::Rock,
        (Shoot::Scissor, Result::Lose) => Shoot::Paper,

        (_, Result::Tie) => (*opp_hand).clone()
    }
}

fn main() {
    let mut file = File::open("day2.input").unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let mut total_score = 0;

    for line in contents.lines() {
        let opp_hand = match line.chars().nth(0).unwrap() {
            'A' => Shoot::Rock,
            'B' => Shoot::Paper,
            'C' => Shoot::Scissor,
            _ => Shoot::Scissor
        };

        let your_hand = match line.chars().nth(2).unwrap() {
            'X' => Shoot::Rock,
            'Y' => Shoot::Paper,
            'Z' => Shoot::Scissor,
            _ => Shoot::Scissor
        };
        let temp_score = score(opp_hand, your_hand);
        println!("Add: {} \t to Total: {}" ,temp_score, total_score);
        total_score += temp_score;
    }

    println!("The total score from the guide is: {}", total_score);

    // PART 2
    // X = need to lose
    // Y = need to draw
    // Z = need to win
    let mut total_2 = 0;
    for line in contents.lines() {
        let opp_hand = match line.chars().nth(0).unwrap() {
            'A' => Shoot::Rock,
            'B' => Shoot::Paper,
            'C' => Shoot::Scissor,
            _ => Shoot::Scissor
        };

        let ideal_result = match line.chars().nth(2).unwrap() {
            'X' => Result::Lose,
            'Y' => Result::Tie,
            'Z' => Result::Win,
            _ => Result::Tie
        };

        let temp_score = score(opp_hand, find_your_move(&opp_hand, &ideal_result));
        println!("Add: {} \t to Total: {}" ,temp_score, total_2);
        total_2 += temp_score;
    }
    println!("The total score from the guide is: {}", total_2);


}
