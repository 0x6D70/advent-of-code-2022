use std::collections::HashSet;

fn main() {
    let content = std::fs::read_to_string("./input.txt").unwrap();
    let commands = content
        .split('$')
        .map(|x| x.trim())
        .flat_map(|x| if x.is_empty() { None } else { Some(x) })
        .collect::<Vec<_>>();

    let mut path: Vec<String> = Vec::new();
    let mut files: Vec<(String, usize)> = Vec::new();

    for command in commands {
        let mut lines = command.lines().collect::<Vec<_>>();
        let cmd = lines.remove(0);
        let cmd_parts = cmd.split_ascii_whitespace().collect::<Vec<_>>();

        if cmd_parts[0] == "cd" {
            if cmd_parts[1] == "/" {
                path.clear();
            } else if cmd_parts[1] == ".." {
                path.pop();
            } else {
                path.push(cmd_parts[1].to_string());
            }
        } else if cmd_parts[0] == "ls" {
            for line in lines {
                let l = line.split_ascii_whitespace().collect::<Vec<_>>();

                if l[0] != "dir" {
                    let size = l[0].parse::<usize>().unwrap();
                    let p = path.join("/") + "/" + l[1];

                    files.push((p, size));
                }
            }
        } else {
            unreachable!()
        }
    }

    let mut directories: HashSet<String> = HashSet::new();

    for (file_path, _) in &files {
        let mut parts = file_path.split('/').collect::<Vec<_>>();
        parts.pop(); // remove file name

        while !parts.is_empty() {
            directories.insert(parts.join("/"));
            parts.pop();
        }
    }

    let mut solution1 = 0;

    for dir in &directories {
        let mut size = 0;

        for (name, file_size) in &files {
            if name.starts_with(dir) {
                size += file_size;
            }
        }

        if size < 100000 {
            solution1 += size;
        }
    }

    println!("solution 1: {}", solution1);

    let mut sum_all_files = 0;

    for (_, file_size) in &files {
        sum_all_files += file_size;
    }

    let space_to_delete = sum_all_files - (70000000 - 30000000);
    let mut directory_sizes: Vec<usize> = Vec::new();

    for dir in &directories {
        let mut size = 0;

        for (name, file_size) in &files {
            if name.starts_with(dir) {
                size += file_size;
            }
        }

        directory_sizes.push(size);
    }

    directory_sizes.sort();

    let mut i = 0;

    let solution2 = loop {
        if directory_sizes[i] > space_to_delete {
            break directory_sizes[i];
        }

        i += 1;
    };

    println!("solution 2: {}", solution2);
}
