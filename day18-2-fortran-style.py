import sys


def preprocess(line):
    return (
        "(("
        + line.replace("(", "(((")
        .replace(")", ")))")
        .replace("*", "))*((")
        .replace("+", ")+(")
        + "))"
    )


print(sum(eval(preprocess(line)) for line in sys.stdin.readlines()))
