fn main() {
    let input = include_str!("../input.txt");
    let data: Vec<&str> = input.split("\n\n").collect();
    let (stackdata, instructions) = (data[0], data[1]);

    let stackdata: Vec<Vec<char>> = stackdata
        .lines()
        .map(|l| {
            l.chars()
                .map(|c| if c.is_alphabetic() { c } else { ' ' })
                .collect()
        })
        .collect();
    let mut stacks: Vec<Vec<char>> = vec![vec![' '; stackdata.len()]; stackdata[0].len()];
    for i in 0..stackdata.len() {
        for j in 0..stackdata[0].len() {
            stacks[j][i] = stackdata[i][j].clone();
        }
    }
    let mut stacks: Vec<Vec<char>> = stacks
        .into_iter()
        .map(|s| s.into_iter().filter(|c| !c.is_whitespace()).collect())
        .collect();

    for s in stacks.iter_mut() {
        s.reverse();
    }

    let mut stacks: Vec<Vec<char>> = stacks.into_iter().filter(|s| !s.is_empty()).collect();

    let instructions: Vec<Vec<usize>> = instructions
        .lines()
        .map(|l| {
            l.split_whitespace()
                .filter_map(|s| s.parse::<usize>().ok())
                .collect()
        })
        .collect();

    // stacks.iter().for_each(|l| println!("{:?}", l));
    // println!();
    for ins in instructions.into_iter() {
        let mut tomove: Vec<char> = Vec::new();
        for i in 0..ins[0] {
            tomove.push(stacks[ins[1]-1].pop().unwrap());
        }
        tomove.reverse();
        stacks[ins[2]-1].append(&mut tomove);
    }

    stacks.iter().for_each(|s| print!("{}", s[s.len()-1]));
    println!();

    
}
