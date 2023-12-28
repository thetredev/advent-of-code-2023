import sys


max_cubes: dict[str, int] = {
    "red": 12,
    "green": 13,
    "blue": 14
}


def is_possible(groups: list[str]) -> bool:
    for group in groups:
        slices: list[str] = group.split(", ")

        for value in slices:
            cubes, color = value.split(" ")

            if color.endswith("\n"):
                color = color[:-1]

            if int(cubes) > max_cubes[color]:
                return False

    return True


def lex(path: str) -> int:
    with open(path) as fp:
        for index, line in enumerate(fp):
            game_id: int = index + 1

            # : + 2 = start of data
            data: str = line[line.index(":") + 2:]
            groups: list[str] = data.split("; ")

            if is_possible(groups):
                yield game_id


if __name__ == "__main__":
    print(sum(lex(sys.argv[1])))
