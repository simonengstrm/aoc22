use std::collections::HashSet;

fn main() {
    let input = include_str!("../input.txt");
    let instructions: Vec<Vec<&str>> = input.lines().map(|l| l.split_whitespace().collect()).collect();
    let mut visited: HashSet<(i32, i32)> = HashSet::new();

    // let mut head: (i32, i32) = (0,0);
    // let mut tail: (i32, i32) = (0,0);

    let mut rope = vec![(0,0); 10];

    for i in instructions.iter() {
        for c in 0..i[1].parse::<usize>().unwrap() {
            rope[0] = step(rope[0], i[0]);

            for k in 1..rope.len() {
                if !touching(rope[k-1], rope[k]) {
                    rope[k] = move_tail(rope[k-1], rope[k]);
                }
            }
        
            visited.insert(rope[rope.len()-1]);

            // If not touching, figure out where to move
            // if !touching(head, tail) {
            //     tail = move_tail(head, tail);
            //     visited.insert(tail);
            // }
            // visited.insert(tail);
        }
    }

    println!("Visisted: {:?}", visited.len());
}

fn move_tail(head: (i32, i32), tail: (i32, i32)) -> (i32, i32) {
    let mut new_tail: (i32, i32) = tail.clone();
    // On the same row -> Move towards head
    if head.1 == tail.1 {
        if head.0 > tail.0 {
            new_tail.0 += 1;
        } else {
            new_tail.0 -= 1;
        }
    }

    // On the same col -> Move towards head
    else if head.0 == tail.0 {
        if head.1 > tail.1 {
            new_tail.1 += 1;
        } else {
            new_tail.1 -= 1;
        }
    }
    
    // Different row and col -> Move diagonally towards
    else {
        // Figure out direction
        if head.0 > tail.0 { // Right
            new_tail.0 += 1;
        } else { // Left
            new_tail.0 -= 1;
        }

        if head.1 > tail.1 { // Up
            new_tail.1 += 1;
        } else { // Down
            new_tail.1 -= 1;
        }
    }

    new_tail
}

fn step((x,y): (i32, i32), dir: &str) -> (i32, i32) {
    match dir {
        "R" => (x+1, y),
        "L" => (x-1, y),
        "U" => (x, y+1),
        "D" => (x, y-1),
        _ => (x,y)
    }
}

fn touching(head: (i32, i32), tail: (i32, i32)) -> bool {
    head.0.abs_diff(tail.0) <= 1 && head.1.abs_diff(tail.1) <= 1
}
