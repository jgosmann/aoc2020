from collections import deque
import re
import sys

decks = (deque(), deque())

player = 0
for line in sys.stdin:
    line = line.strip()
    if line != "":
        m = re.match(r"Player (\d+):", line)
        if m:
            player = int(m.group(1)) - 1
        else:
            decks[player].append(int(line))

while all(len(d) > 0 for d in decks):
    cards = [d.popleft() for d in decks]
    if cards[0] > cards[1]:
        decks[0].extend(cards)
    elif cards[0] < cards[1]:
        decks[1].extend(reversed(cards))
    else:
        raise AssertionError("Duplicate card.")

winning_deck = next(d for d in decks if len(d) > 0)
score = sum(
    a * b for a, b in zip(reversed(winning_deck), range(1, len(winning_deck) + 1))
)
print(score)
