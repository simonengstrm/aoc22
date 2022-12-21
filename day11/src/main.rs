#[derive(Debug, Clone)]
struct Monkey {
    id: usize,
    items: Vec<u128>,
    op: Operator,
    test_nr: u128,
    true_monkey: usize,
    false_monkey: usize,
    inspects: usize,
}

#[derive(Debug, Clone)]
enum Operator {
    Add(u128),
    Mul(u128),
    Square,
}

impl Operator {
    fn do_op(&self, a: u128) -> u128 {
        match self {
            Operator::Add(b) => a + b,
            Operator::Mul(b) => a * b,
            Operator::Square => a * a,
        }
    }
}

fn main() {
    let input: Vec<Vec<Vec<&str>>> = include_str!("../input.txt")
        .split("\n\n")
        .map(|s| s.lines().map(|l| l.split_whitespace().collect()).collect())
        .collect();

    let mut monkeys: Vec<Monkey> = input.iter().map(|seg| to_monkey(seg)).collect();
    let N = monkeys.iter().fold(1, |acc, m| acc * m.test_nr);

    for _ in 0..10000 {
        for i in 0..monkeys.len() {
            while !monkeys[i].items.is_empty() {
                let mut item = monkeys[i].items.pop().unwrap();
                item = monkeys[i].op.do_op(item) % N;
                monkeys[i].inspects += 1;
                let to_monkey = if item % monkeys[i].test_nr == 0 {
                    monkeys[i].true_monkey
                } else {
                    monkeys[i].false_monkey
                };
                monkeys[to_monkey].items.push(item);
            }
        }
    }
    for m in monkeys.iter() {
        println!("Monkey {}, inspects: {}", m.id, m.inspects);
    }
    monkeys.sort_by_key(|m| m.inspects);
    monkeys.reverse();
    let part1: usize = monkeys[0..2].iter().fold(1, |acc, m| acc * m.inspects);
    println!("{}", part1);
}

fn to_monkey(seg: &Vec<Vec<&str>>) -> Monkey {
    let id: usize = seg[0][1][0..1].parse::<usize>().unwrap();
    let mut items: Vec<u128> = seg[1][2..]
        .iter()
        .map(|n| n.trim_end_matches(',').parse().unwrap())
        .collect();
    let op = match seg[2][4] {
        "*" => match seg[2][5].parse() {
            Ok(num) => Operator::Mul(num),
            Err(_) => Operator::Square,
        },
        "+" => Operator::Add(seg[2][5].parse().unwrap()),
        _ => Operator::Square,
    };
    let test_nr = seg[3][3].parse().unwrap();
    let true_monkey = seg[4][5].parse::<usize>().unwrap();
    let false_monkey = seg[5][5].parse::<usize>().unwrap();
    items.reverse();

    Monkey {
        id,
        items,
        op,
        test_nr,
        true_monkey,
        false_monkey,
        inspects: 0,
    }
}
