from functools import partial
from pathlib import Path

here = Path(__file__).absolute().parent


def a_fully_contained_in_b(a, b):
    return a[0] >= b[0] and a[1] <= b[1]


def a_partly_contained_in_b(a, b):
    return b[0] <= a[0] <= b[1] or b[0] <= a[1] <= b[1]


def section_contained(contained_fun, section: str) -> bool:
    fst, snd = (list(map(int, x.split("-"))) for x in section.split(","))
    return contained_fun(fst, snd) or contained_fun(snd, fst)


def compute_total(text, contained_func) -> int:
    sections = map(str.strip, text.strip().split("\n"))
    return sum(map(partial(section_contained, contained_func), sections))


def main() -> int:

    with open(here / "input.txt") as f:
        text = f.read()

    print(f"Part 1:\n{compute_total(text, a_fully_contained_in_b)}")
    print(f"Part 2:\n{compute_total(text, a_partly_contained_in_b)}")

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
else:
    import pytest

    example_input = """
    2-4,6-8
    2-3,4-5
    5-7,7-9
    2-8,3-7
    6-6,4-6
    2-6,4-8
    """

    def test_compute_total_part1() -> None:
        assert compute_total(example_input, a_fully_contained_in_b) == 2

    @pytest.mark.parametrize(
        ("text", "expected"),
        (
            ("2-4,6-8", False),
            ("2-3,4-5", False),
            ("5-7,7-9", False),
            ("2-8,3-7", True),
            ("6-6,4-6", True),
            ("2-6,4-8", False),
        ),
    )
    def test_section_fully_contained(text, expected):
        assert section_contained(a_fully_contained_in_b, text) is expected

    @pytest.mark.parametrize(
        ("text", "expected"),
        (
            ("2-4,6-8", False),
            ("2-3,4-5", False),
            ("5-7,7-9", True),
            ("2-8,3-7", True),
            ("6-6,4-6", True),
            ("2-6,4-8", True),
        ),
    )
    def test_section_partly_contained(text, expected):
        assert section_contained(a_partly_contained_in_b, text) is expected

    def test_compute_total_part2() -> None:
        assert compute_total(example_input, a_partly_contained_in_b) == 4
