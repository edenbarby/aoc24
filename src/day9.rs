use std::{cmp::Ordering, fs::read_to_string};

#[derive(Clone, Copy)]
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
    let disk_map = read_to_string("input/09/input.txt").unwrap();
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

    // display_disk(&disk);

    let mut next_free_space = 1;
    let mut last_file = disk.len() - (disk.len() % 2);

    while next_free_space < last_file {
        match disk[next_free_space] {
            DiskItem::File { length: _, id: _ } => next_free_space += 1,
            DiskItem::FreeSpace {
                length: free_space_length,
            } => match disk[last_file] {
                DiskItem::File {
                    length: file_length,
                    id,
                } => match free_space_length.cmp(&file_length) {
                    Ordering::Less => {
                        if let DiskItem::File { length, id: _ } = &mut disk[last_file] {
                            *length -= free_space_length;
                        }

                        disk[next_free_space] = DiskItem::File {
                            length: free_space_length,
                            id,
                        };
                        next_free_space += 2;
                    }
                    Ordering::Equal => {
                        let item = disk.remove(last_file);
                        last_file -= 2;

                        // Completely consume the free space.
                        disk[next_free_space] = item;
                        next_free_space += 2;
                    }
                    Ordering::Greater => {
                        // Move the file into location.
                        let item = disk.remove(last_file);
                        last_file -= 2; // Point to the next last file (2 since they alternate).

                        // We inset before the free space because it will have some space left over.
                        disk.insert(next_free_space, item);
                        next_free_space += 1; // Shfit everything right by 1.
                        last_file += 1;

                        // Make sure the next freespace has it's length update to reflect the newly moved file.
                        if let DiskItem::FreeSpace { length } = &mut disk[next_free_space] {
                            *length -= file_length;
                        }
                    }
                },
                DiskItem::FreeSpace { length: _ } => last_file -= 1,
            },
        }

        // display_disk(&disk);
    }

    let mut block_pos = 0;
    let mut checksum = 0;
    for item in disk {
        if let DiskItem::File { length, id } = item {
            let first_block_pos = block_pos;
            let last_block_pos = block_pos + length - 1;
            block_pos += length;
            checksum += id * length * (first_block_pos + last_block_pos) / 2;
        }
    }

    println!("day 9 part 1: {}", checksum);
}

pub fn part2() {
    let disk_map = read_to_string("input/09/input.txt").unwrap();
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

    for target_file_id in (0..id).rev() {
        let target_file = disk
            .iter()
            .find(|i| match i {
                DiskItem::File { length: _, id } => *id == target_file_id,
                DiskItem::FreeSpace { length: _ } => false,
            })
            .unwrap();

        let target_file_length = match target_file {
            DiskItem::File { length, id: _ } => length,
            DiskItem::FreeSpace { length: _ } => panic!(),
        };

        let large_enough_space = disk.iter().find(|i| match i {
            DiskItem::File { length: _, id: _ } => false,
            DiskItem::FreeSpace { length } => length >= target_file_length,
        });
    }

    // // display_disk(&disk);

    // let mut next_free_space = 1;
    // let mut last_file = disk.len() - (disk.len() % 2);

    // while next_free_space < last_file {
    //     match disk[next_free_space] {
    //         DiskItem::File { length: _, id: _ } => next_free_space += 1,
    //         DiskItem::FreeSpace {
    //             length: free_space_length,
    //         } => match disk[last_file] {
    //             DiskItem::File {
    //                 length: file_length,
    //                 id,
    //             } => match free_space_length.cmp(&file_length) {
    //                 Ordering::Less => {
    //                     if let DiskItem::File { length, id: _ } = &mut disk[last_file] {
    //                         *length -= free_space_length;
    //                     }

    //                     disk[next_free_space] = DiskItem::File {
    //                         length: free_space_length,
    //                         id,
    //                     };
    //                     next_free_space += 2;
    //                 }
    //                 Ordering::Equal => {
    //                     let item = disk.remove(last_file);
    //                     last_file -= 2;

    //                     // Completely consume the free space.
    //                     disk[next_free_space] = item;
    //                     next_free_space += 2;
    //                 }
    //                 Ordering::Greater => {
    //                     // Move the file into location.
    //                     let item = disk.remove(last_file);
    //                     last_file -= 2; // Point to the next last file (2 since they alternate).

    //                     // We inset before the free space because it will have some space left over.
    //                     disk.insert(next_free_space, item);
    //                     next_free_space += 1; // Shfit everything right by 1.
    //                     last_file += 1;

    //                     // Make sure the next freespace has it's length update to reflect the newly moved file.
    //                     if let DiskItem::FreeSpace { length } = &mut disk[next_free_space] {
    //                         *length -= file_length;
    //                     }
    //                 }
    //             },
    //             DiskItem::FreeSpace { length: _ } => last_file -= 1,
    //         },
    //     }

    //     // display_disk(&disk);
    // }

    let mut block_pos = 0;
    let mut checksum = 0;
    for item in disk {
        if let DiskItem::File { length, id } = item {
            let first_block_pos = block_pos;
            let last_block_pos = block_pos + length - 1;
            block_pos += length;
            checksum += id * length * (first_block_pos + last_block_pos) / 2;
        }
    }

    println!("day 9 part 2: {}", checksum);
}
