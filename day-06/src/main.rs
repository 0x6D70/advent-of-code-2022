fn all_different(chars: &[char]) -> bool {
    for (i, ch1) in chars.iter().enumerate() {
        for (k, ch2) in chars.iter().enumerate() {
            if i != k && ch1 == ch2 {
                return false;
            }
        }
    }

    true
}

fn main() {
    let content = std::fs::read_to_string("./input.txt").unwrap();
    let chars = content.chars().collect::<Vec<_>>();

    let mut solution1 = 3;

    while solution1 < chars.len() {
        let marker = &chars[solution1 - 3..solution1 + 1];

        if all_different(marker) {
            break;
        }

        solution1 += 1;
    }

    println!("solution 1: {}", solution1 + 1);

    let mut solution2 = 13;

    while solution2 < chars.len() {
        let marker = &chars[solution2 - 13..solution2 + 1];

        if all_different(marker) {
            break;
        }

        solution2 += 1;
    }

    println!("solution 2: {}", solution2 + 1);
}
