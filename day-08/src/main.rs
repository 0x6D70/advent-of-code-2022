fn is_visible(field: &Vec<Vec<u32>>, x: usize, y: usize) -> bool {
    // TODO: check if at the edge

    let tree_height = field[y][x];
    let mut visible = false;

    // down
    let mut all_smaller = true;
    for y_idx in 0..y {
        if field[y_idx][x] >= tree_height {
            all_smaller = false;
        }
    }

    if all_smaller {
        visible = true;
    }

    // up
    let mut all_smaller = true;
    for y_idx in y + 1..field.len() {
        if field[y_idx][x] >= tree_height {
            all_smaller = false;
        }
    }

    if all_smaller {
        visible = true;
    }

    // left
    let mut all_smaller = true;
    for x_idx in 0..x {
        if field[y][x_idx] >= tree_height {
            all_smaller = false;
        }
    }

    if all_smaller {
        visible = true;
    }

    // right
    let mut all_smaller = true;
    for x_idx in x + 1..field[0].len() {
        if field[y][x_idx] >= tree_height {
            all_smaller = false;
        }
    }

    if all_smaller {
        visible = true;
    }

    visible
}

fn viewing_distance(field: &Vec<Vec<u32>>, x: usize, y: usize) -> usize {
    let tree_height = field[y][x];
    let mut score = 1;

    // down
    let mut count = 0;
    for y_idx in (0..y).rev() {
        if field[y_idx][x] >= tree_height {
            count += 1;
            break;
        }

        count += 1;
    }
    score *= count;

    // up
    let mut count = 0;
    for y_idx in y + 1..field.len() {
        if field[y_idx][x] >= tree_height {
            count += 1;
            break;
        }

        count += 1;
    }
    score *= count;

    // left
    let mut count = 0;
    for x_idx in (0..x).rev() {
        if field[y][x_idx] >= tree_height {
            count += 1;
            break;
        }

        count += 1;
    }
    score *= count;

    // right
    let mut count = 0;
    for x_idx in x + 1..field[0].len() {
        if field[y][x_idx] >= tree_height {
            count += 1;
            break;
        }

        count += 1;
    }
    score *= count;

    score
}

fn main() {
    let content = std::fs::read_to_string("./input.txt").unwrap();
    let field = content
        .lines()
        .map(|x| {
            x.chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let x_len = field[0].len();
    let y_len = field.len();

    let mut solution1 = 0;

    for y in 0..y_len {
        for x in 0..x_len {
            if is_visible(&field, x, y) {
                solution1 += 1;
            }
        }
    }

    println!("solution 1: {}", solution1);

    let mut solution2 = 0;

    for y in 0..y_len {
        for x in 0..x_len {
            let a = viewing_distance(&field, x, y);

            if a > solution2 {
                solution2 = a;
            }
        }
    }

    println!("solution 2: {}", solution2);
}
