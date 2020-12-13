import math
import sys


def lcm(a, b):
    return a * b // math.gcd(a, b)


_ = int(sys.stdin.readline())
schedule = [
    (int(bus_id), dt)
    for dt, bus_id in enumerate(sys.stdin.readline().split(","))
    if bus_id != "x"
]
print(schedule)

t = 0
increment = 1
for bus_id, dt in schedule:
    cycle_length = lcm(increment, bus_id)
    while (t + dt) % bus_id != 0:
        t += increment
    increment = cycle_length

print(t)
