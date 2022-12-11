use std::fs;

fn a_contained_in_b(a_low: &str, a_high: &str, b_low: &str, b_high: &str) -> bool {
    let low = a_low.parse::<i32>().unwrap() >= b_low.parse::<i32>().unwrap();
    let high = a_high.parse::<i32>().unwrap() <= b_high.parse::<i32>().unwrap();
    high & low
}

fn section_fully_contained(section: &str) -> u32 {
    let (fst, snd) = section.split_once(",").unwrap();
    let (fst_low, fst_high) = fst.split_once("-").unwrap();
    let (snd_low, snd_high) = snd.split_once("-").unwrap();

    (a_contained_in_b(fst_low, fst_high, snd_low, snd_high)
        | a_contained_in_b(snd_low, snd_high, fst_low, fst_high)) as u32
}

fn compute_total_part1(text: &str) -> u32 {
    text.trim()
        .split("\n")
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .map(section_fully_contained)
        .sum()
}
fn main() {
    let file_path = "input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let score_part_1 = compute_total_part1(&contents);
    println!("Part1: {score_part_1}");
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = "
    2-4,6-8
    2-3,4-5
    5-7,7-9
    2-8,3-7
    6-6,4-6
    2-6,4-8
    ";

    #[test]
    fn test_compute_total_part1() {
        let total = compute_total_part1(EXAMPLE_INPUT);
        assert_eq!(total, 2);
    }
}
