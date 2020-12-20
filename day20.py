from collections import defaultdict
from functools import reduce
import operator
import re
import sys


class ParseError(RuntimeError):
    pass


class Tile:
    def __init__(self, tile_id, tile):
        self.tile_id = tile_id
        self.tile = tile

    @classmethod
    def read_from_file(cls, f):
        header_match = re.match(r"Tile\s+(\d+):", f.readline().strip())
        if not header_match:
            raise ParseError()

        tile = cls(
            int(header_match.group(1)), [f.readline().strip() for _ in range(10)],
        )
        f.readline()
        return tile

    def get_borders(self):
        yield self.tile[0]
        yield self.tile[-1]
        yield "".join(row[0] for row in self.tile)
        yield "".join(row[-1] for row in self.tile)


def read_tiles(f):
    try:
        while True:
            yield Tile.read_from_file(f)
    except ParseError:
        pass


tiles = list(read_tiles(sys.stdin))

border_counts = defaultdict(lambda: 0)
for tile in tiles:
    for border in tile.get_borders():
        border_counts[border] += 1
        border_counts[border[::-1]] += 1


def is_corner_tile(tile):
    num_not_matched_borders = 0
    for border in tile.get_borders():
        if border_counts[border] < 2 and border_counts[border[::-1]] < 2:
            num_not_matched_borders += 1
    return num_not_matched_borders == 2


corner_tiles = [tile.tile_id for tile in tiles if is_corner_tile(tile)]
print("Corner tiles:", corner_tiles)
print("Corner tile product:", reduce(operator.mul, corner_tiles, 1))
