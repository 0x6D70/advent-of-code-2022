fn char_to_priority(c: &char) -> u32 {
    if c.is_uppercase() {
        *c as u32 - 'A' as u32 + 27
    } else {
        *c as u32 - 'a' as u32 + 1
    }
}

fn main() {
    let content = std::fs::read_to_string("./input.txt").unwrap();
    let mut result: Vec<char> = Vec::new();

    content.lines().for_each(|line| {
        let (part1, part2) = line.split_at(line.len() / 2);

        for c in part1.chars() {
            if part2.contains(c) {
                result.push(c);
                break;
            }
        }
    });

    let priority_sum = result.iter().map(char_to_priority).sum::<u32>();

    println!("solution 1: {}", priority_sum);

    let mut result2: Vec<char> = Vec::new();
    let lines = content.lines().collect::<Vec<_>>();

    for idx in (0..lines.len()).step_by(3) {
        let part1 = lines[idx];
        let part2 = lines[idx + 1];
        let part3 = lines[idx + 2];

        for c in part1.chars() {
            if part2.contains(c) && part3.contains(c) {
                result2.push(c);
                break;
            }
        }
    }

    let priority_sum2 = result2.iter().map(char_to_priority).sum::<u32>();

    println!("solution 2: {}", priority_sum2);
}
