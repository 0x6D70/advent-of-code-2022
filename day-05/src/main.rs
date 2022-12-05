fn main() {
    let content = std::fs::read_to_string("./input.txt").unwrap();
    let lines = content.lines().collect::<Vec<_>>();

    let mut i = 0;

    let idx = loop {
        i += 1;

        if lines[i].starts_with(" 1") {
            break i;
        }
    };

    let mut stacks: Vec<(usize, Vec<char>)> = vec![];

    for (i, c) in lines[idx].chars().enumerate() {
        if c.is_ascii_digit() {
            let stack_no = c.to_digit(10).unwrap() as usize;

            let mut v: Vec<char> = vec![];

            for k in (0..idx).rev() {
                let line = lines[k].chars().collect::<Vec<_>>();
                let c = line.get(i);

                if let Some(ch) = c {
                    if *ch != ' ' {
                        v.push(*ch);
                    }
                }
            }

            stacks.push((stack_no, v));
        }
    }

    let mut stacks2 = stacks.clone();

    let moves = content.split("\n\n").collect::<Vec<_>>()[1];

    for move_line in moves.lines() {
        let move_vec = move_line
            .split_ascii_whitespace()
            .filter_map(|val| {
                if val.chars().all(|x| x.is_ascii_digit()) {
                    Some(val.parse::<usize>().unwrap())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        let from = move_vec[1];
        let to = move_vec[2];

        for _ in 0..move_vec[0] {
            let ch = stacks[from - 1].1.pop().unwrap();
            stacks[to - 1].1.push(ch);
        }
    }

    let mut result1 = String::new();

    for (_, stack) in stacks {
        if !stack.is_empty() {
            result1.push(stack[stack.len() - 1]);
        }
    }

    println!("solution 1: {}", result1);

    for move_line in moves.lines() {
        let move_vec = move_line
            .split_ascii_whitespace()
            .filter_map(|val| {
                if val.chars().all(|x| x.is_ascii_digit()) {
                    Some(val.parse::<usize>().unwrap())
                } else {
                    None
                }
            })
            .collect::<Vec<_>>();

        let from = move_vec[1];
        let to = move_vec[2];

        let mut v: Vec<char> = vec![];

        for _ in 0..move_vec[0] {
            let ch = stacks2[from - 1].1.pop().unwrap();
            v.push(ch);
        }

        for ch in v.iter().rev() {
            stacks2[to - 1].1.push(*ch);
        }
    }

    let mut result2 = String::new();

    for (_, stack) in stacks2 {
        if !stack.is_empty() {
            result2.push(stack[stack.len() - 1]);
        }
    }

    println!("solution 1: {}", result2);
}
