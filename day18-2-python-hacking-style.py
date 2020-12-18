import re
import sys


class Num:
    def __init__(self, num):
        self.num = num

    def __add__(self, other):
        return Num(self.num * other.num)

    def __mul__(self, other):
        return Num(self.num + other.num)


def preprocess(line):
    line = line.replace("*", "#").replace("+", "*").replace("#", "+")
    return re.sub(r"(\d+)", r"Num(\1)", line)


print(sum(eval(preprocess(line)).num for line in sys.stdin.readlines()))
