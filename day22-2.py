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


def recursive_combat(decks):
    decks = [deque(d) for d in decks]
    previous_decks = set()
    while all(len(d) > 0 for d in decks):
        decks_copy = tuple(tuple(d) for d in decks)
        if decks_copy in previous_decks:
            return 0, decks[0]
        previous_decks.add(decks_copy)
        cards = [d.popleft() for d in decks]
        if all(len(d) >= c for c, d in zip(cards, decks)):
            winner, _ = recursive_combat([list(d)[:c] for c, d in zip(cards, decks)])
        elif cards[0] > cards[1]:
            winner = 0
        elif cards[0] < cards[1]:
            winner = 1
        else:
            raise AssertionError("Duplicate card.")
        if winner == 0:
            decks[0].extend(cards)
        else:
            decks[1].extend(reversed(cards))
    winner, winning_deck = next((i, d) for i, d in enumerate(decks) if len(d) > 0)
    return winner, winning_deck


_, winning_deck = recursive_combat(decks)
score = sum(
    a * b for a, b in zip(reversed(winning_deck), range(1, len(winning_deck) + 1))
)
print(score)
