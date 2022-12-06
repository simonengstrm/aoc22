fn main() {
    let input = include_str!("../input.txt");
    let segsize: usize = 14;
    
    let mut res = 0;
    for i in segsize..input.len()-segsize {
        println!("{}", &input[i-segsize..i]);
        if !has_dupes(&input[i-segsize..i]) {
            res = i; 
            break;
        }
    }

    println!("{}", res);
}

fn has_dupes(s: &str) -> bool {
    for a in s.chars() {
        let mut count = 0;
        for b in s.chars() {
            if a == b {
                count += 1;
            }
        } 
        if count > 1 {
            return true;
        }
    }
    return false;
}
