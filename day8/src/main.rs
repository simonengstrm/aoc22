
fn main() {
    let input = include_str!("../input.txt");
    let data: Vec<Vec<u32>> = input.lines().map(|l| l.chars().map(|c| c.to_digit(10).unwrap()).collect()).collect();
    // data.iter().for_each(|l| println!("{:?}", l));

    let mut visible_trees = usize::try_from(data.len()*4-4).unwrap();

    visible_trees += count_visible_trees(&data);
    println!("Visible trees: {}", visible_trees);
 
    println!("Max scenic score: {}", max_scenic_score(&data));
}

fn count_visible_trees(trees: &Vec<Vec<u32>>) -> usize {
    let mut visible_trees = 0;

    for i in 1..trees.len()-1 {
        for j in 1..trees.len()-1 {
            if is_visible(i,j,&trees) {
                visible_trees += 1;
            }
        }
    }

    return visible_trees;
}

fn max_scenic_score(trees: &Vec<Vec<u32>>) -> usize {
    let mut max_score: usize = 0;

    for i in 0..trees.len() {
        for j in 0..trees.len() {
            let score = scenic_score(i, j, &trees);
            if score > max_score {
                max_score = score;
            }
        }
    }

    max_score
}

fn is_visible(i: usize, j: usize, trees: &Vec<Vec<u32>>) -> bool {
    let height = trees[i][j];
    // Left
    let left = trees[i][..j].iter().all(|&t| t < height);
    // Right
    let right = trees[i][j+1..].iter().all(|&t| t < height);
    // Up
    let mut up = true;
    for r in 0..i {
        if trees[r][j] >= height {
            up = false;
        }
    }
    // Down
    let mut down = true;
    for r in i+1..trees.len() {
        if trees[r][j] >= height {
            down = false;
        }
    }
    left || right || up || down
}

fn scenic_score(i: usize, j: usize, trees: &Vec<Vec<u32>>) -> usize {
    let height = trees[i][j];
    
    let mut left_trees = 0;
    for c in (0..j).rev() {
        if trees[i][c] >= height {
            left_trees += 1;
            break;
        }
        left_trees += 1;
    }

    let mut right_trees = 0;
    for c in j+1..trees.len() {
        if trees[i][c] >= height {
            right_trees += 1;
            break;
        }
        right_trees += 1;
    }

    let mut up_trees = 0;
    for r in (0..i).rev() {
        if trees[r][j] >= height {
            up_trees += 1;
            break;
        }
        up_trees += 1;
    }

    let mut down_trees = 0;
    for r in i+1..trees.len() {
        if trees[r][j] >= height {
            down_trees += 1;
            break;
        }
        down_trees += 1;
    }

    return left_trees * right_trees * up_trees * down_trees;
}

