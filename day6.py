import string
import sys

def count_distinct_answers(group):
    return len(set(answer for answer in group if answer in string.ascii_lowercase))

def count_unanimous_answers(group):
    return len(set.intersection(*(set(individual) for individual in group.split('\n'))))

groups = sys.stdin.read().split("\n\n")
print(
    "Count for anyone answered yes:",
    sum(count_distinct_answers(group) for group in groups)
)
print(
    "Count for everyone answered yes:",
    sum(count_unanimous_answers(group) for group in groups)
)
