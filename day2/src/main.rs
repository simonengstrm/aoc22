// Rock: A X
// Paper: B Y
// Scissors: C Z
// A beats C
// B beats A
// C beats B

fn main() {
    day1();
    day2();
}

fn day2() {
    let input = include_str!("../input.txt");

    let data: Vec<&str> = input.lines().collect();
    let data: Vec<Vec<&str>> = data.iter().map(|l| l.split_whitespace().collect()).collect();
    let result = data.iter().fold(0, |acc, l| {
        acc + play2(l)
    });
    
    println!("{}", result)
}

fn play2(plays: &Vec<&str>) -> usize {
    return match plays[1] {
        "Y" => {
            // Draw
            let mut score = 0;
            score += 3; //Draw score
            score += move_to_score(move_to_move(plays[0]));
            score
        },
        "X" => {
            // Lose
            let mut score = 0;
            score += move_to_score(move_to_lose(plays[0]));
            score
        },
        "Z" => {
            // Win 
            let mut score = 0;
            score += 6; //Draw score
            score += move_to_score(move_to_win(plays[0]));
            score
        },
        _ => 0
    } 
}

fn move_to_win(m: &str) -> &str {
    match m {
        "A" => "Y",
        "B" => "Z",
        "C" => "X",
        _ => ""
    }
}

fn move_to_lose(m: &str) -> &str {
    match m {
        "A" => "Z",
        "B" => "X",
        "C" => "Y",
        _ => "" 
    }
}

fn move_to_move(m: &str) -> &str {
    match m {
        "A" => "X",
        "B" => "Y",
        "C" => "Z",
        _ => ""
    }
}

fn day1() {
    let input = include_str!("../input.txt");

    let data: Vec<&str> = input.lines().collect();
    let data: Vec<Vec<&str>> = data.iter().map(|l| l.split_whitespace().collect()).collect();
    let result = data.iter().fold(0, |acc, l| {
        acc + play1(l)
    });

    println!("{}", result)
}

fn play1(plays: &Vec<&str>) -> usize {
    return match plays[0] {
        "A" => {
            let mut score = 0;
            if plays[1] == "Y" {
                score += 6;
            } else if plays[1] == "X" {
                score += 3;
            }
            score += move_to_score(plays[1]);
            return score;
        },
        "B" => {
            let mut score = 0;
            if plays[1] == "Z" {
                score += 6;
            } else if plays[1] == "Y" {
                score += 3;
            }
            score += move_to_score(plays[1]);
            return score;
        },
        "C" => {
            let mut score = 0;
            if plays[1] == "X" {
                score += 6;
            } else if plays[1] == "Z" {
                score += 3;
            }
            score += move_to_score(plays[1]);
            return score;
        },
        _ => 0
    } 
}

fn move_to_score(m: &str) -> usize {
    match m {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => 0
    }
}
