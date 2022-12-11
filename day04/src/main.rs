use std::fs;

fn a_fully_contained_in_b(a_low: &str, a_high: &str, b_low: &str, b_high: &str) -> bool {
    let low = a_low.parse::<i32>().unwrap() >= b_low.parse::<i32>().unwrap();
    let high = a_high.parse::<i32>().unwrap() <= b_high.parse::<i32>().unwrap();
    high && low
}

fn a_partly_contained_in_b(a_low: &str, a_high: &str, b_low: &str, b_high: &str) -> bool {
    let b0 = b_low.parse::<i32>().unwrap();
    let b1 = b_high.parse::<i32>().unwrap();
    let a0 = a_low.parse::<i32>().unwrap();
    let a1 = a_high.parse::<i32>().unwrap();

    let low = b0 <= a0 && a0 <= b1;
    let high = b0 <= a1 && a1 <= b1;
    high || low
}

fn section_contained(contained_func: fn(&str, &str, &str, &str) -> bool, section: &str) -> u32 {
    let (fst, snd) = section.split_once(",").unwrap();
    let (fst_low, fst_high) = fst.split_once("-").unwrap();
    let (snd_low, snd_high) = snd.split_once("-").unwrap();

    (contained_func(fst_low, fst_high, snd_low, snd_high)
        | contained_func(snd_low, snd_high, fst_low, fst_high)) as u32
}

fn compute_total(text: &str, contained_func: fn(&str, &str, &str, &str) -> bool) -> u32 {
    let func = |x| section_contained(contained_func, x);
    text.trim()
        .split("\n")
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .map(func)
        .sum()
}
fn main() {
    let file_path = "input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let score_part_1 = compute_total(&contents, a_fully_contained_in_b);
    println!("Part1: {score_part_1}");
    let score_part_2 = compute_total(&contents, a_partly_contained_in_b);
    println!("Part2: {score_part_2}");
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
        let total = compute_total(EXAMPLE_INPUT, a_fully_contained_in_b);
        assert_eq!(total, 2);
    }

    #[test]
    fn test_compute_total_part2() {
        let total = compute_total(EXAMPLE_INPUT, a_partly_contained_in_b);
        assert_eq!(total, 4);
    }
}
