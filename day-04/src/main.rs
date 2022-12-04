fn sections_contains_other(elf1: (usize, usize), elf2: (usize, usize)) -> bool {
    elf1.0 >= elf2.0 && elf1.1 <= elf2.1 || elf1.0 <= elf2.0 && elf1.1 >= elf2.1
}

fn sections_overlap(elf1: (usize, usize), elf2: (usize, usize)) -> bool {
    elf1.0 <= elf2.0 && elf1.1 >= elf2.0
        || elf1.0 <= elf2.1 && elf1.1 >= elf2.1
        || elf2.0 <= elf1.0 && elf2.1 >= elf1.0
        || elf2.0 <= elf1.1 && elf2.1 >= elf1.1
}

fn main() {
    let content = std::fs::read_to_string("./input.txt").unwrap();
    let mut result = 0;

    content.lines().for_each(|line| {
        let mut line_split = line.split(',');
        let elf1 = line_split.next().unwrap();
        let elf2 = line_split.next().unwrap();

        let elf1_sections = elf1
            .split('-')
            .collect::<Vec<_>>()
            .iter()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let elf2_sections = elf2
            .split('-')
            .collect::<Vec<_>>()
            .iter()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        if sections_contains_other(
            (elf1_sections[0], elf1_sections[1]),
            (elf2_sections[0], elf2_sections[1]),
        ) {
            result += 1;
        }
    });

    println!("solution 1: {}", result);

    let mut result2 = 0;

    content.lines().for_each(|line| {
        let mut line_split = line.split(',');
        let elf1 = line_split.next().unwrap();
        let elf2 = line_split.next().unwrap();

        let elf1_sections = elf1
            .split('-')
            .collect::<Vec<_>>()
            .iter()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();
        let elf2_sections = elf2
            .split('-')
            .collect::<Vec<_>>()
            .iter()
            .map(|x| x.parse::<usize>().unwrap())
            .collect::<Vec<_>>();

        if sections_overlap(
            (elf1_sections[0], elf1_sections[1]),
            (elf2_sections[0], elf2_sections[1]),
        ) {
            result2 += 1;
        }
    });

    println!("solution 2: {}", result2);
}
