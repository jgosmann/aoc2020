import numpy as np
import sys

adapter_chain = sorted(int(line) for line in sys.stdin.readlines())
device_joltage = adapter_chain[-1] + 3
joltage_diffs = np.diff([0] + adapter_chain + [device_joltage])
joltage_diff_dist = np.bincount(joltage_diffs)

print("Answer part 1:", joltage_diff_dist[1] * joltage_diff_dist[3])

num_arrangements = np.zeros_like(joltage_diffs)
num_arrangements[0] = 1

for i in range(1, len(num_arrangements)):
    diff = 0
    j = i
    while j >= 0:
        diff += joltage_diffs[j]
        if diff > 3:
            break

        num_arrangements[i] += num_arrangements[j - 1] if j > 0 else 1

        j -= 1

print("Num. arrangements:", num_arrangements[-1])
