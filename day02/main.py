from pathlib import Path
from typing import Callable

here = Path(__file__).absolute().parent

SELECTION_SCORE_PART1 = {
    "X": 1,  # Rock
    "Y": 2,  # Paper
    "Z": 3,  # Scissor
}

OUTCOME_SCORE_PART1 = {
    "A": {"X": 3, "Y": 6, "Z": 0},
    "B": {"X": 0, "Y": 3, "Z": 6},
    "C": {"X": 6, "Y": 0, "Z": 3},
}

SELECTION_SCORE_PART2 = {
    "A": 1,  # Rock
    "B": 2,  # Paper
    "C": 3,  # Scissor
}

OUTCOME_SCORE_PART2 = {
    "A": {"A": 3, "B": 6, "C": 0},
    "B": {"A": 0, "B": 3, "C": 6},
    "C": {"A": 6, "B": 0, "C": 3},
}

CHOICE_PART2 = {
    "A": {"X": "C", "Y": "A", "Z": "B"},
    "B": {"X": "A", "Y": "B", "Z": "C"},
    "C": {"X": "B", "Y": "C", "Z": "A"},
}


def compute_score_part1(opponent: str, you: str) -> int:
    return SELECTION_SCORE_PART1[you] + OUTCOME_SCORE_PART1[opponent][you]


def compute_score_part2(opponent: str, choice: str) -> int:
    you = CHOICE_PART2[opponent][choice]
    return SELECTION_SCORE_PART2[you] + OUTCOME_SCORE_PART2[opponent][you]


def compute_total(text: str, compute_score: Callable[[str, str], int]) -> int:
    clean_text = filter(lambda x: x != "", map(str.strip, text.strip().split("\n")))
    return sum(map(lambda s: compute_score(*s.split(" ")), clean_text))


def main() -> int:

    with open(here / "input.txt") as f:
        text = f.read()

    print(f"Part 1:\n{compute_total(text, compute_score_part1)}")
    print(f"Part 2:\n{compute_total(text, compute_score_part2)}")

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
else:
    import pytest

    example_input = """
    A Y\n
    B X\n
    C Z\n
    """

    @pytest.mark.parametrize(("text", "expected_total"), ((example_input, 15),))
    def test_compute_total_part1(text, expected_total) -> None:
        assert compute_total(text, compute_score_part1) == expected_total

    @pytest.mark.parametrize(
        ("opponent", "you", "score"),
        (
            ("A", "Y", 8),
            ("B", "X", 1),
            ("C", "Z", 6),
        ),
    )
    def test_compute_score_part1(opponent, you, score):
        assert compute_score_part1(opponent, you) == score

    @pytest.mark.parametrize(
        ("opponent", "you", "score"),
        (
            ("A", "Y", 4),
            ("B", "X", 1),
            ("C", "Z", 7),
        ),
    )
    def test_compute_score_part2(opponent, you, score):
        assert compute_score_part2(opponent, you) == score

    @pytest.mark.parametrize(("text", "expected_total"), ((example_input, 12),))
    def test_compute_total_part2(text, expected_total) -> None:
        assert compute_total(text, compute_score_part2) == expected_total
