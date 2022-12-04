fn main() {
    let input = include_str!("../input.txt");
    let data: Vec<Vec<Vec<usize>>> = input.lines().map(|l| l.split(',').map(|s| s.split('-').map(|n| n.parse().unwrap()).collect()).collect()).collect();

    let res: usize = data.iter().map(|l| if (overlaps(l)) {1} else {0}).sum();
    println!("{}", res);
}

fn overlaps(line: &Vec<Vec<usize>>) -> bool {
    let start1 = line[0][0];
    let start2 = line[1][0];
    let end1 = line[0][1];
    let end2 = line[1][1];

    if (start1..=end1).contains(&start2) || (start1..=end1).contains(&end2) {
        return true;
    } else if (start2..=end2).contains(&start1) || (start2..=end2).contains(&end1) {
        return true;
    }
    return false;
}
