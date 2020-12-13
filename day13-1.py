import sys

earliest = int(sys.stdin.readline())
bus_ids = [int(bus_id) for bus_id in sys.stdin.readline().split(",") if bus_id != "x"]
wait = min((bus_id - (earliest % bus_id), bus_id) for bus_id in bus_ids)
print(wait[0] * wait[1])
