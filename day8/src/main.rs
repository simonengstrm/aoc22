fn main() {
    let input = include_str!("../example.txt");
    let data: Vec<Vec<u32>> = input.lines().map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect()).collect();
    println!("{:?}", data);
}
