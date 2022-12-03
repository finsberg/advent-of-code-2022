from pathlib import Path

here = Path(__file__).absolute().parent


def get_total_calories(s: str) -> list[int]:

    elves = s.strip().split("\n\n")
    total_calories = [sum(map(int, elf.split("\n"))) for elf in elves]
    total_calories.sort()
    return total_calories


def main() -> int:

    with open(here / "input.txt") as f:
        text = f.read()

    total_calories = get_total_calories(text)
    print("Part 1:")
    print(total_calories[-1])
    print("Part 2:")
    print(sum(total_calories[-3:]))

    return 0


if __name__ == "__main__":
    raise SystemExit(main())
else:
    import pytest

    @pytest.mark.parametrize(
        ("input_s", "expected"),
        (("""1\n2\n\n4\n""", [3, 4]), ("""10\n2\n\n4\n""", [4, 12])),
    )
    def test_get_total_calories(input_s: str, expected: int) -> None:
        assert get_total_calories(input_s) == expected
