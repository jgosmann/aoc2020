import sys

expenses = sorted(int(line) for line in sys.stdin.readlines())

for i, base in enumerate(expenses):
    target = 2020 - base
    left = i + 1
    right = len(expenses) - 1

    while left < right:
        summed = expenses[left] + expenses[right]
        if summed == target:
            print(base * expenses[left] * expenses[right])
            sys.exit(0)
        if summed < target:
            left += 1
        else:
            right -= 1

