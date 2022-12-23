use std::cmp::Reverse;

use priority_queue::PriorityQueue;

fn main() {
    let mut map: Vec<Vec<char>> = include_str!("../input.txt")
        .lines()
        .map(|l| l.chars().collect())
        .collect();
    let (mut x, mut y) = (0, 0);
    let (mut ex, mut ey) = (0, 0);
    for r in 0..map.len() {
        for c in 0..map[r].len() {
            if map[r][c] == 'S' {
                x = c;
                y = r;
                map[r][c] = 'a';
            } else if map[r][c] == 'E' {
                ex = c;
                ey = r;
                map[r][c] = 'z';
            }
        }
    }
        
    // Part 1
    let (prev,dist) = dijkstra((y, x),(ey,ex), &map);
    println!("{}", dist[ey][ex]);

    // Part 2
    let mut shortest = usize::MAX;
    for row in 0..map.len() {
        for col in 0..map[0].len() {
            if map[row][col] == 'a' {
                let length = dijkstra((row, col), (ey,ex), &map).1[ey][ex];
                if length < shortest {
                    shortest = length;
                }
            }
        }
    }
    println!("{}", shortest);
}

fn dijkstra(source: (usize, usize), target: (usize, usize), map: &Vec<Vec<char>>) -> (Vec<Vec<(usize, usize)>>, Vec<Vec<usize>>) {
    let mut prev = vec![vec![(usize::MAX, usize::MAX); map[0].len()]; map.len()];
    let mut dist = vec![vec![100000; map[0].len()]; map.len()];
    let mut q: PriorityQueue<(usize, usize), Reverse<usize>> = PriorityQueue::new();
    dist[source.0][source.1] = 0;

    for row in 0..map.len() {
        for col in 0..map[0].len() {
            q.push((row,col), Reverse(dist[row][col]));
        }
    }

    while !q.is_empty() {
        let u = q.pop().unwrap();
        for v in get_candidates(u.0, map) {
            let alt = dist[u.0.0][u.0.1] + 1;
            if alt < dist[v.0][v.1] {
                dist[v.0][v.1] = alt;
                prev[v.0][v.1] = u.0;
                q.push_decrease((v.0, v.1), Reverse(alt));
                if u.0 == target {break;}
            }
        }
    }

    (prev, dist)
}

fn get_candidates((y, x): (usize, usize), map: &Vec<Vec<char>>) -> Vec<(usize, usize)> {
    let mut res: Vec<(usize, usize)> = vec![];

    if y + 1 < map.len() {
        res.push((y + 1, x));
    }
    if y as i32 - 1 >= 0 {
        res.push((y - 1, x));
    }
    if x + 1 < map[0].len() {
        res.push((y, x + 1));
    }
    if x as i32 - 1 >= 0 {
        res.push((y, x - 1));
    }

    res = res
        .into_iter()
        .filter(|(cy, cx)| {
            if map[*cy][*cx] as i32 <= map[y][x] as i32 + 1 {
                return true;
            } else {
                return false;
            }
        })
        .collect();

    res
}
