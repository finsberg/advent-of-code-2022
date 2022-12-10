use std::collections::HashSet;
use std::fs;

fn compute_priority(item: &char) -> u32 {
    if item.is_ascii_uppercase() {
        *item as u32 - 65 + 27
    } else {
        *item as u32 - 96
    }
}

fn rucksack2priority(rucksack: &str) -> u32 {
    let (fst, snd) = rucksack.split_at(rucksack.len() / 2);
    let fst_set: HashSet<char> = fst.chars().collect();
    let snd_set: HashSet<char> = snd.chars().collect();
    let common_item = fst_set.intersection(&snd_set).into_iter().next().unwrap();
    compute_priority(common_item)
}

fn compute_total_part1(text: &str) -> u32 {
    text.trim()
        .split("\n")
        .map(|x| x.trim())
        .filter(|x| !x.is_empty())
        .map(rucksack2priority)
        .sum()
}

fn priorities_part2<'a, I>(lst: I) -> Vec<u32>
where
    I: Iterator<Item = &'a str>,
{
    // Create a new iterable where we can check if we have
    // exhausted the iterator
    let mut iter = lst.into_iter().peekable();
    // Store the values in a vector
    // FIXME: No need to use a vector here since the
    // size should be one third of the iterator
    let mut ret = Vec::new();

    // While there is still a next element
    while iter.peek().is_some() {
        let fst_set: HashSet<char> = iter.next().unwrap().chars().collect();
        let snd_set: HashSet<char> = iter.next().unwrap().chars().collect();
        let thrd_set: HashSet<char> = iter.next().unwrap().chars().collect();
        let common_fst_snd: HashSet<char> = fst_set.intersection(&snd_set).cloned().collect();
        let common_item = common_fst_snd
            .intersection(&thrd_set)
            .into_iter()
            .next()
            .unwrap();
        ret.push(compute_priority(common_item));
    }
    ret
}

fn compute_total_part2(text: &str) -> u32 {
    let rucksacks = text
        .trim()
        .split("\n")
        .map(|x| x.trim())
        .filter(|x| !x.is_empty());
    let priorities = priorities_part2(rucksacks);
    priorities.into_iter().sum()
}

fn main() {
    let file_path = "input.txt";
    println!("In file {}", file_path);

    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let score_part_1 = compute_total_part1(&contents);
    println!("Part1: {score_part_1}");
    let score_part_2 = compute_total_part2(&contents);
    println!("Part2: {score_part_2}");
}

#[cfg(test)]
mod tests {
    use super::*;

    static EXAMPLE_INPUT: &str = "
    vJrwpWtwJgWrhcsFMMfFFhFp\n
    jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n
    PmmdzqPrVvPwwTWBwg\n
    wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n
    ttgJtRGJQctTZtZT\n
    CrZsJsPPZsGzwwsLwLmpwMDw
    ";

    #[test]
    fn test_compute_total_part1() {
        let total = compute_total_part1(EXAMPLE_INPUT);
        assert_eq!(total, 157);
    }

    #[test]
    fn test_compute_total_part2() {
        let total = compute_total_part2(EXAMPLE_INPUT);
        assert_eq!(total, 70);
    }
}
