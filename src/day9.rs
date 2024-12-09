use std::fs::read_to_string;

enum DiskItem {
    File { length: usize, id: usize },
    FreeSpace { length: usize },
}

fn display_disk(disk: &Vec<DiskItem>) {
    for item in disk {
        match item {
            DiskItem::File { length, id } => {
                let c = id.to_string().chars().last().unwrap();
                for _ in 0..*length {
                    print!("{}", c);
                }
            }
            DiskItem::FreeSpace { length } => {
                for _ in 0..*length {
                    print!(".");
                }
            }
        }
    }
    println!();
}

pub fn part1() {
    let disk_map = read_to_string("input/09/example.txt").unwrap();
    let mut file_next = true;
    let mut id = 0;
    let mut disk = Vec::new();

    for c in disk_map.trim().chars() {
        let length = c.to_digit(10).unwrap() as usize;
        if file_next {
            disk.push(DiskItem::File { length, id });
            id += 1;
        } else {
            disk.push(DiskItem::FreeSpace { length });
        }
        file_next ^= true;
    }

    display_disk(&disk);

    let mut next_free_space = 1;
    let mut last_file = disk.len() - (disk.len() % 2);

    while next_free_space < last_file {
        match disk[next_free_space] {
            DiskItem::File { length, id } => next_free_space += 1,
            DiskItem::FreeSpace { length } => {}
        }
    }

    println!("day 9 part 1: {}", 0);
}

pub fn part2() {
    println!("day 9 part 1: {}", 0);
}
