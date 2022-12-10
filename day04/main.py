from pathlib import Path

here = Path(__file__).absolute().parent


def a_contained_in_b(a, b):
    return a[0] >= b[0] and a[1] <= b[1]


def section_fully_contained(section: str) -> bool:
    fst, snd = (list(map(int, x.split("-"))) for x in section.split(","))
    return a_contained_in_b(fst, snd) or a_contained_in_b(snd, fst)


def compute_total_part1(text) -> int:
    sections = map(str.strip, text.strip().split("\n"))
    return sum(map(section_fully_contained, sections))


def main() -> int:

    with open(here / "input.txt") as f:
        text = f.read()

    print(f"Part 1:\n{compute_total_part1(text)}")

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
        assert compute_total_part1(example_input) == 2

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
        assert section_fully_contained(text) is expected
