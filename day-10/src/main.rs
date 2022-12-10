
fn main() {
    let content = std::fs::read_to_string("./input.txt").unwrap();
    
    let mut signal: Vec<isize> = Vec::new();
    let mut x = 1;

    signal.push(x);

    for instruction in content.lines() {
        let parts = instruction.split_ascii_whitespace().collect::<Vec<_>>();

        if parts[0] == "noop" {
            signal.push(x);
        } else if parts[0] == "addx" {
            let offset = parts[1].parse::<isize>().unwrap();

            signal.push(x);
            signal.push(x);

            x += offset;
        } else {
            unreachable!()
        }
    }

    println!("solution 1: {}", 20 * signal[20] + 60 * signal[60] + 100 * signal[100] + 140 * signal[140] + 180 * signal[180] + 220 * signal[220]);

    // part 2

    for y in 0..6 {
        for x in 0..40 {
            let cycle = y * 40 + x;
            let offset = cycle % 40;


            if (signal[cycle + 1] as i32).abs_diff(offset as i32) <= 1 {
                print!("#");
            } else {
                print!(".");
            }
        }

        println!();
    }
}
