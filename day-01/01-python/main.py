import sys


def lex(path: str) -> None:
    with open(path) as fp:
        for line in fp:
            digits = [x for x in line if x.isdigit()]
            yield int(f"{digits[0]}{digits[-1]}")


if __name__ == "__main__":
    print(sum(lex(sys.argv[1])))
