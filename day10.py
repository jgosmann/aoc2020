import numpy as np
import sys

adapter_chain = sorted(int(line) for line in sys.stdin.readlines())
joltage_diff_dist = np.bincount(np.diff([0] + adapter_chain + [adapter_chain[-1] + 3]))
print(joltage_diff_dist[1] * joltage_diff_dist[3])
