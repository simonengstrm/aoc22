// Approach 1:
// Recursively determine the size of folders by recursing down every folder until a folder with
// only files is found, sum it and return. When unwinding the recursion, every folders sum is
// calculated from the return value, and all > 100000 is stored in an accumulator.
// Every folder to recurse down into is found by searching the input for "ls {dirname}" after the
// "dir {dirname}" is found in the input file
//
// Approach 2:
// Same as before, but the input is parsed as a tree

use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");
    let data: Vec<&str> = input.lines().collect();
    let mut bigfolders: Vec<usize> = Vec::new();
    let mut visited: HashSet<usize> = HashSet::new();
    let total = rec_folder_size("/", &data, &mut bigfolders, &mut visited);

    // Part 1
    println!(
        "Sum of dirs with size <= 100000: {}",
        bigfolders.iter().filter(|s| s <= &&100000).sum::<usize>()
    );

    // Part 2
    let unused: usize = 70000000 - total;
    println!("Unused space: {}", unused);
    let needed: usize = 30000000 - unused;
    println!("Needed space: {}", needed);
    let folders_big_enough: Vec<usize> = bigfolders.into_iter().filter(|s| s >= &&needed).collect();
    println!("Size of minimal folder to remove: {}", folders_big_enough.iter().min().unwrap());
}

fn rec_folder_size(
    c_folder: &str,
    data: &Vec<&str>,
    big_folders: &mut Vec<usize>,
    visited: &mut HashSet<usize>,
) -> usize {
    let (mut start, mut end) = folder_indexes(c_folder, data, 0);
    while visited.contains(&start) {
        (start, end) = folder_indexes(c_folder, data, end);
    }
    visited.insert(start);
    let sum: usize = data[start..end]
        .iter()
        .map(|l| {
            let s: Vec<&str> = l.split_whitespace().collect();
            match s[0] {
                "dir" => rec_folder_size(s[1], data, big_folders, visited),
                _ => s[0].parse::<usize>().unwrap(),
            }
        })
        .sum();

    big_folders.push(sum);
    return sum;
}

fn folder_indexes(c_folder: &str, data: &Vec<&str>, look_from: usize) -> (usize, usize) {
    let start_index: usize = data
        .iter()
        .skip(look_from)
        .position(|&l| l == format!("$ cd {}", c_folder))
        .unwrap()
        + 2
        + look_from;
    let end_index = match data[start_index..].iter().position(|l| l.starts_with("$")) {
        Some(index) => index + start_index,
        None => data.len(),
    };
    (start_index, end_index)
}
