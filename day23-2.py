class Node:
    def __init__(self, value):
        self.value = value
        self.next = None


max_node = 1_000_000
nodes = {i: Node(i) for i in range(1, max_node + 1)}

initial_arrangement = [1, 5, 6, 7, 9, 4, 8, 2, 3, 10]
current = nodes[initial_arrangement[0]]
for i in range(1, len(initial_arrangement)):
    prev = initial_arrangement[i - 1]
    nodes[prev].next = nodes[initial_arrangement[i]]
for i in range(len(initial_arrangement) + 1, max_node + 1):
    nodes[i - 1].next = nodes[i]
nodes[max_node].next = current


def take_n(start, n):
    cur = start
    for _ in range(n):
        cur = cur.next
    snippet_start = start.next
    start.next = cur.next
    cur.next = None
    return snippet_start


def insert(insertion_point, nodes):
    end = insertion_point.next
    insertion_point.next = nodes
    cur = nodes
    while cur.next is not None:
        cur = cur.next
    cur.next = end


def decrement_label(l):
    if l <= 1:
        return max_node
    else:
        return l - 1


for i in range(10_000_000):
    picked_up = take_n(current, 3)
    picked_values = set(
        [picked_up.value, picked_up.next.value, picked_up.next.next.value]
    )
    destination = decrement_label(current.value)
    while destination in picked_values:
        destination = decrement_label(destination)
    insert(nodes[destination], picked_up)
    current = current.next

print(nodes[1].next.value * nodes[1].next.next.value)
