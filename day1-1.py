import sys

expenses = sorted(int(line) for line in sys.stdin.readlines())

left = 0
right = len(expenses) - 1

while (summed := expenses[left] + expenses[right]) != 2020:
    assert left < right

    if summed < 2020:
        left += 1
    else:
        right -= 1

print(expenses[left] * expenses[right])
