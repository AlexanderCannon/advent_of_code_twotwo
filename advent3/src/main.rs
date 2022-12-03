use std::collections::HashSet;
use std::fs;

fn main() {
    println!("{}", scores())
}

fn scores() -> u32 {
    let mut file = fs::read_to_string("./src/input.txt").expect("Less code!");
    let mismatched_scores = file.lines().map(|str| str.into()).map(find_mismatch).sum();
    return mismatched_scores;
}

fn item_score(item: char) -> u32 {
    match item {
        'a'..='z' => item as u32 - 'a' as u32 + 1,
        'A'..='Z' => item as u32 - 'A' as u32 + 27,
        _ => unreachable!(),
    }
}

fn find_mismatch(list: &str) -> u32 {
    let (first, second) = list.split_at(list.len() / 2);
    let first_as_chars: HashSet<char> = first.chars().collect();
    let second_as_chars: HashSet<char> = second.chars().collect();
    let matches = *first_as_chars
        .intersection(&second_as_chars)
        .next()
        .expect("no matches");
    println!("{}: {}", matches, item_score(matches));
    return item_score(matches);
}

fn sum_of_badges() {
    let file = fs::read_to_string("./src/input.txt").expect("Less code!");
    let file_vec: <Vec<str>> = file.lines().map(|str| str.into());
    let mut result = 0;

    println!("{}", file_vec.len());
    // let elves = file.lines().map(|str| str.into()).collect();
    // let mut f_struct = file.lines().map(|str| str.into());
    // while f_struct.len() > 0 {
    //     let (elf_1, elf_2, elf_3, ..) = f_struct;
    // f_struct = rest
    for [elv_1, elv_2, elv_3] in file.lines() {

    for i in elf_1.chars() {
        if elf_2.contains(i) && elf_3.contains(i) {
            result += item_score(i);
            break;
        }
    }

    }
    }
    println!("Result: {}", result);
}
