from dataclasses import dataclass
import re
import sys
from typing import Dict, Set
from weakref import WeakSet


@dataclass
class BagNode:
    color: str
    may_be_contained_in: WeakSet  # Prevent cyclic strong references to allow GC by ref count
    needs_to_contain: dict

    def __hash__(self):
        return hash(self.color)


def transitive_containment_hull(root: BagNode) -> Set[BagNode]:
    return (
        set(root.may_be_contained_in).union(
        *(
            transitive_containment_hull(node)
            for node in root.may_be_contained_in
        ))
    )


def total_bags_inside(root: BagNode) -> int:
    return sum(
        count * (1 + total_bags_inside(node))
        for node, count in root.needs_to_contain.items()
    )


def get_node(name: str, graph: Dict[str, BagNode]) -> BagNode:
    m = re.match(r"\s*(.*)\s+bags?\s*.?", name)
    color = m.group(1)
    if color not in graph_of_bags:
        graph_of_bags[color] = BagNode(
            color=color, may_be_contained_in=WeakSet(), needs_to_contain={}
        )
    return graph_of_bags[color]


graph_of_bags = {}

for line in sys.stdin.readlines():
    outer, inner_list = line.split("contain")

    outer_node = get_node(outer, graph_of_bags)

    for inner_with_count in inner_list.split(","):
        m = re.match(r"\s*(\d+)\s+(.*)", inner_with_count)
        if m:
            count = int(m.group(1))
            inner = m.group(2)
            inner_node = get_node(inner, graph_of_bags)
            inner_node.may_be_contained_in.add(outer_node)
            outer_node.needs_to_contain[inner_node] = count


root = graph_of_bags["shiny gold"]
print(len(transitive_containment_hull(root)))
print(total_bags_inside(root))
