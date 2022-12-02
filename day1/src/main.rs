use std::time;

fn main() {
    day1();
}

fn day1() {
    let input = include_str!("../data.txt");
    let data: Vec<usize> = input.split("\n\n").map(|g| g.lines().map(|c| c.parse::<usize>().unwrap()).sum());
}

fn day1_2() {
    let before = time::Instant::now();
    let input = include_str!("../data.txt");

    let data:Vec<&str> = input.lines().collect();
    let mut calories: Vec<usize> = Vec::new();
    let mut acc = 0;
    for i in data.iter() {
        if i.is_empty() {
            calories.push(acc);
            acc = 0;
            continue;
        }
        let current: usize = i.parse().expect("Couldn't parse line");
        acc += current;
    }
    
    calories.sort();
    calories.reverse();
    let sum: usize = calories[0..3].iter().sum();
    println!("{:?}", before.elapsed());
    println!("{:?}", sum);
}
