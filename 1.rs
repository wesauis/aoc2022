use std::fs::File;
use std::path::Path;
use std::io;
use std::io::BufRead;
use std::collections::HashMap;
use std::iter::FromIterator;

fn main() {
    let calories_by_elf = get_calories_by_elf(read_lines("1.input").unwrap());

    let mut elves_ranked = Vec::from_iter(calories_by_elf);
    elves_ranked.sort_by(|&(_, a), &(_, b)| b.cmp(&a));

    println!("--- Day 1: Calorie Counting ---");

    let top_1 = &elves_ranked[0];
    println!("The Elf number {} carries the most calories: {}", top_1.0, top_1.1);

    println!("\n--- Part Two ---");

    let top_3 = &elves_ranked[0..3];
    println!("The top 3 Elves, by calories, are:");
    for elf in top_3 {
        println!("~> {: >3} carries {} cal", elf.0, elf.1);
    }

    let total = top_3.iter().map(|&(_, calories)| calories).sum::<u32>();
    println!("The 3 combined carry {} cal", total);
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}

fn get_calories_by_elf(lines: io::Lines<io::BufReader<File>>) -> HashMap<u32, u32> {
    let mut elf_num = 1u32;
    let mut calories_by_elf: HashMap<u32, u32> = HashMap::new();

    for line_or_error in lines {
        let calories_or_blank = line_or_error.unwrap().trim().to_owned();

        if calories_or_blank.is_empty() {
            elf_num += 1;
            continue;
        }

        let calories = calories_or_blank.parse::<u32>().unwrap();
        *calories_by_elf.entry(elf_num).or_insert(0) += calories;
    }

    calories_by_elf
}
