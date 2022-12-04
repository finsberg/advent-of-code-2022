from pathlib import Path

here = Path(__file__).absolute().parent


def compute_priority(item: str) -> int:
    if item.isupper():
        return ord(item) - 65 + 27
    else:
        return ord(item) - 96


def rucksack2priority(rucksack: str) -> int:
    fst = set(rucksack[: len(rucksack) // 2])
    snd = set(rucksack[len(rucksack) // 2 :])
    common_item = (fst & snd).pop()
    return compute_priority(common_item)


def compute_total_part1(text: str) -> int:
    clean_text = filter(lambda x: x != "", map(str.strip, text.strip().split("\n")))
    return sum(map(rucksack2priority, clean_text))


def compute_total_part2(text: str) -> int:
    rucksacks = filter(lambda x: x != "", map(str.strip, text.strip().split("\n")))

    common = set(next(rucksacks))
    total = 0
    for i, rucksack in enumerate(rucksacks, start=1):
        if i % 3 == 0:
            assert len(common) == 1
            total += compute_priority(common.pop())
            common = set(rucksack)
        else:
            common &= set(rucksack)

    assert (i + 1) % 3 == 0
    total += compute_priority(common.pop())
    return total


def main() -> int:

    with open(here / "input.txt") as f:
        text = f.read()

    print(f"Part 1:\n{compute_total_part1(text)}")
    print(f"Part 1:\n{compute_total_part2(text)}")

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
else:
    import pytest

    example_input = """
    vJrwpWtwJgWrhcsFMMfFFhFp\n
    jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\n
    PmmdzqPrVvPwwTWBwg\n
    wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\n
    ttgJtRGJQctTZtZT\n
    CrZsJsPPZsGzwwsLwLmpwMDw
    """

    @pytest.mark.parametrize(("text", "expected_total"), ((example_input, 157),))
    def test_compute_total_part1(text, expected_total) -> None:
        assert compute_total_part1(text) == expected_total

    @pytest.mark.parametrize(("text", "expected_total"), ((example_input, 70),))
    def test_compute_total_part2(text, expected_total) -> None:
        assert compute_total_part2(text) == expected_total
