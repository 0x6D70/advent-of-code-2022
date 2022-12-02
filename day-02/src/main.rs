#[derive(Debug, Clone)]
enum Rps {
    Rock,
    Paper,
    Scissors,
}

impl From<&str> for Rps {
    fn from(s: &str) -> Self {
        match s {
            "A" | "X" => Rps::Rock,
            "B" | "Y" => Rps::Paper,
            "C" | "Z" => Rps::Scissors,
            _ => panic!("Invalid input"),
        }
    }
}

impl Rps {
    fn to_number(&self) -> usize {
        match self {
            Rps::Rock => 1,
            Rps::Paper => 2,
            Rps::Scissors => 3,
        }
    }
}

fn main() {
    let content = std::fs::read_to_string("input.txt").unwrap();

    let plays = content
        .lines()
        .map(|line| {
            let parts = line.split(' ').collect::<Vec<_>>();

            (Rps::from(parts[0]), Rps::from(parts[1]))
        })
        .collect::<Vec<_>>();

    let mut points = 0;

    for (opponent, elf) in &plays {
        let elf_points: usize = elf.to_number();

        points += match (opponent, elf) {
            (Rps::Paper, Rps::Paper) => 3,
            (Rps::Rock, Rps::Rock) => 3,
            (Rps::Scissors, Rps::Scissors) => 3,
            (Rps::Rock, Rps::Paper) => 6,
            (Rps::Rock, Rps::Scissors) => 0,
            (Rps::Paper, Rps::Rock) => 0,
            (Rps::Paper, Rps::Scissors) => 6,
            (Rps::Scissors, Rps::Rock) => 6,
            (Rps::Scissors, Rps::Paper) => 0,
        };

        points += elf_points;
    }

    println!("solution 1: {}", points);

    let mut points2 = 0;

    for (opponent, elf) in &plays {
        let elf_points: usize = elf.to_number();

        let move_selected = match (opponent, elf_points) {
            (Rps::Rock, 1) => Rps::Scissors,
            (Rps::Rock, 2) => Rps::Rock,
            (Rps::Rock, 3) => Rps::Paper,
            (Rps::Paper, 1) => Rps::Rock,
            (Rps::Paper, 2) => Rps::Paper,
            (Rps::Paper, 3) => Rps::Scissors,
            (Rps::Scissors, 1) => Rps::Paper,
            (Rps::Scissors, 2) => Rps::Scissors,
            (Rps::Scissors, 3) => Rps::Rock,
            _ => unreachable!(),
        };

        points2 += (elf_points - 1) * 3;
        points2 += move_selected.to_number();
    }

    println!("solution 2: {}", points2);
}
