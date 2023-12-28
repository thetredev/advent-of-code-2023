import sys


def lex(path: str) -> int:
    result: int = 0

    with open(path) as fp:
        for line in fp:
            digits = [x for x in line if x.isdigit()]
            result += int(f"{digits[0]}{digits[-1]}")

    return result


if __name__ == "__main__":
    print(lex(sys.argv[1]))
