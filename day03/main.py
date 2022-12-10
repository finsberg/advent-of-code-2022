import typing
from pathlib import Path

T = typing.TypeVar("T")
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


def create_groups_of_three(
    lst: typing.Iterable[T],
) -> typing.Iterable[
    typing.Tuple[typing.Any, ...]
]:  # actually typing.Iterable[typing.Tuple[T, T, T]]:
    return zip(*[iter(lst)] * 3)


def compute_total_part1(text: str) -> int:
    clean_text = filter(lambda x: x != "", map(str.strip, text.strip().split("\n")))
    return sum(map(rucksack2priority, clean_text))


def compute_total_part2(text: str) -> int:
    rucksacks = filter(lambda x: x != "", map(str.strip, text.strip().split("\n")))

    total = 0
    for (fst, snd, thrd) in create_groups_of_three(rucksacks):
        common = set(fst) & set(snd) & set(thrd)
        total += compute_priority(common.pop())

    return total


def main() -> int:

    with open(here / "input.txt") as f:
        text = f.read()

    print(f"Part 1:\n{compute_total_part1(text)}")
    print(f"Part 2:\n{compute_total_part2(text)}")

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

    def test_create_groups_of_three():
        groups = create_groups_of_three(range(9))
        assert list(groups) == [(0, 1, 2), (3, 4, 5), (6, 7, 8)]
