use std::{collections::BinaryHeap, fs};

fn str2int(cal: &str) -> u32 {
    cal.parse::<u32>().unwrap()
}

fn elf2total_calories(elf: &str) -> u32 {
    elf.split("\n").map(str2int).sum()
}

fn get_sorted_calories(s: String) -> BinaryHeap<u32> {
    let elves = s.trim_end().split("\n\n");
    let total_calories = elves.map(elf2total_calories);
    let sorted_calories: BinaryHeap<u32> = total_calories.collect();
    sorted_calories
}

fn main() {
    let file_path = "input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut sorted_calories = get_sorted_calories(contents);

    let mut total = sorted_calories.pop().unwrap();
    println!("Part 1:\n{total}");

    for _n in 1..3 {
        total += sorted_calories.pop().unwrap();
    }
    println!("Part 2:\n{total}");
}
