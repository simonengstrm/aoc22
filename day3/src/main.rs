use std::time::{Instant, self};

fn main() {
    let start = time::Instant::now();
    let prio = "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ";
    let input = include_str!("../input.txt");
    let data: Vec<&str> = input.lines().collect();
    
    let mut prios: Vec<String> = Vec::new();

    for i in (0..data.len()).step_by(3) {
        for a in data[i].chars() {
            if data[i+1].contains(a) && data[i+2].contains(a) {
                prios.push(a.to_string());
                break;
            }
        }
    }
    let sum : usize = prios.iter().map(|c| prio.find(c).unwrap()+1).sum();
    println!("{:?}", start.elapsed());
    println!("{}", sum);
}



    // data.iter().for_each(|l| println!("{:?}", l));
    // 
    // let mut prios: Vec<String> = Vec::new();
    // data.iter().for_each(|l| {
    //     'outer: 
    //     for a in l[0..l.len()/2].chars() {
    //         for b in l[l.len()/2..l.len()].chars() {
    //             if a == b {
    //                 prios.push(a.to_string());
    //                 break 'outer;
    //             }
    //         }
    //     }
    // });
    // let sum : usize = prios.iter().map(|c| prio.find(c).unwrap()+1).sum();
    // println!("{:?}", sum);
