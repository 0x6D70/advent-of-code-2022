use std::collections::HashSet;

fn main() {
    let content = std::fs::read_to_string("./input.txt").unwrap();
    let moves = content
        .lines()
        .map(|x| {
            let s = x.split_ascii_whitespace().collect::<Vec<_>>();

            (s[0].chars().next().unwrap(), s[1].parse::<i32>().unwrap())
        })
        .collect::<Vec<_>>();

    let mut head_pos = (0, 0);
    let mut tail_pos = (0, 0);

    let mut tail_positions: HashSet<(i32, i32)> = HashSet::new();
    tail_positions.insert(tail_pos);

    for (direction, count) in &moves {
        for _ in 0..*count {
            match direction {
                'U' => head_pos.1 += 1,
                'D' => head_pos.1 -= 1,
                'L' => head_pos.0 -= 1,
                'R' => head_pos.0 += 1,
                _ => unreachable!(),
            }

            if head_pos.0 == tail_pos.0 {
                if head_pos.1 == tail_pos.1 + 2 {
                    tail_pos.1 += 1;
                } else if head_pos.1 == tail_pos.1 - 2 {
                    tail_pos.1 -= 1;
                }
            } else if head_pos.1 == tail_pos.1 {
                if head_pos.0 == tail_pos.0 + 2 {
                    tail_pos.0 += 1;
                } else if head_pos.0 == tail_pos.0 - 2 {
                    tail_pos.0 -= 1;
                }
            } else if head_pos.0.abs_diff(tail_pos.0) == 2 {
                tail_pos.1 = head_pos.1;

                if head_pos.0 > tail_pos.0 {
                    tail_pos.0 += 1;
                } else {
                    tail_pos.0 -= 1;
                }
            } else if head_pos.1.abs_diff(tail_pos.1) == 2 {
                tail_pos.0 = head_pos.0;

                if head_pos.1 > tail_pos.1 {
                    tail_pos.1 += 1;
                } else {
                    tail_pos.1 -= 1;
                }
            }

            tail_positions.insert(tail_pos);
        }
    }

    println!("solution 1: {}", tail_positions.len());

    // part 2
    let mut positions = vec![(0, 0); 10];

    let mut tail_positions2: HashSet<(i32, i32)> = HashSet::new();
    tail_positions2.insert(positions[9]);

    for (direction, count) in &moves {
        for _ in 0..*count {
            match direction {
                'U' => positions[0].1 += 1,
                'D' => positions[0].1 -= 1,
                'L' => positions[0].0 -= 1,
                'R' => positions[0].0 += 1,
                _ => unreachable!(),
            }

            for i in 0..9 {
                let head_pos = positions[i];
                let mut tail_pos = positions[i + 1];

                if head_pos.0 == tail_pos.0 {
                    if head_pos.1 == tail_pos.1 + 2 {
                        tail_pos.1 += 1;
                    } else if head_pos.1 == tail_pos.1 - 2 {
                        tail_pos.1 -= 1;
                    }
                } else if head_pos.1 == tail_pos.1 {
                    if head_pos.0 == tail_pos.0 + 2 {
                        tail_pos.0 += 1;
                    } else if head_pos.0 == tail_pos.0 - 2 {
                        tail_pos.0 -= 1;
                    }
                } else if head_pos.0.abs_diff(tail_pos.0) == 2
                    || head_pos.1.abs_diff(tail_pos.1) == 2
                {
                    // handle that edge case
                    if head_pos.0 > tail_pos.0 {
                        tail_pos.0 += 1;
                    } else {
                        tail_pos.0 -= 1;
                    }

                    if head_pos.1 > tail_pos.1 {
                        tail_pos.1 += 1;
                    } else {
                        tail_pos.1 -= 1;
                    }
                } else if head_pos.0.abs_diff(tail_pos.0) == 2 {
                    tail_pos.1 = head_pos.1;

                    if head_pos.0 > tail_pos.0 {
                        tail_pos.0 += 1;
                    } else {
                        tail_pos.0 -= 1;
                    }
                } else if head_pos.1.abs_diff(tail_pos.1) == 2 {
                    tail_pos.0 = head_pos.0;

                    if head_pos.1 > tail_pos.1 {
                        tail_pos.1 += 1;
                    } else {
                        tail_pos.1 -= 1;
                    }
                }

                positions[i] = head_pos;
                positions[i + 1] = tail_pos;
            }

            tail_positions2.insert(positions[9]);

            /*
            let mut map = vec![vec!['.'; 6]; 6];

            for (i, (p_x, p_y)) in positions.iter().enumerate() {
                let ch = match i {
                    0 => 'H',
                    _ => i.to_string().chars().next().unwrap()
                };

                map[5 - *p_y as usize][*p_x as usize] = ch;
            }

            for row in &map {
                for c in row {
                    print!("{}", c);
                }

                println!();
            }
            */
        }
    }

    println!("solution 2: {}", tail_positions2.len());
}
