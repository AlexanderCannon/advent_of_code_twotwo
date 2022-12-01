use std::fs::File;
use std::io::prelude::*;

fn main() {
    println!("Top Elf total: {}", top_elf());
    println!("Top Three total: {}", top_three_elves());
}

fn top_elf() -> u32 {
    let mut file = File::open("./src/input.txt").expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Error while reading file");
    let highest = data
        .split("\n\n")
        .map(|e| e.lines().map(|c| c.parse::<u32>().unwrap()).sum::<u32>())
        .max()
        .unwrap();
    return highest;
}

fn top_three_elves() -> u32 {
    let mut file = File::open("./src/input.txt").expect("File not found");
    let mut data = String::new();
    file.read_to_string(&mut data)
        .expect("Error while reading file");
    let mut elves = data
        .split("\n\n")
        .map(|e| e.lines().map(|c| c.parse::<u32>().unwrap()).sum::<u32>())
        .collect::<Vec<u32>>();

    elves.sort_unstable_by(|a, b| b.cmp(a));

    return elves[0] + elves[1] + elves[2];
}
