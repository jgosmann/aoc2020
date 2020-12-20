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
        yield self.get_top_border()
        yield self.get_bottom_border()
        yield self.get_left_border()
        yield self.get_right_border()

    def get_left_border(self):
        return "".join(row[0] for row in self.tile)

    def get_right_border(self):
        return "".join(row[-1] for row in self.tile)

    def get_top_border(self):
        return self.tile[0]

    def get_bottom_border(self):
        return self.tile[-1]

    def rotate(self):
        self.tile = [
            "".join(row[len(row) - i - 1] for row in self.tile)
            for i in range(len(self.tile[0]))
        ]

    def flip(self):
        self.tile = self.tile[::-1]

    def __str__(self):
        return "\n".join(self.tile)

    def core_region(self):
        return [row[1:-1] for row in self.tile[1:-1]]


def read_tiles(f):
    try:
        while True:
            yield Tile.read_from_file(f)
    except ParseError:
        pass


tiles = {tile.tile_id: tile for tile in read_tiles(sys.stdin)}


border2tile = defaultdict(lambda: set())
for tile in tiles.values():
    for border in tile.get_borders():
        border2tile[border].add(tile)
        border2tile[border[::-1]].add(tile)


def is_corner_tile(tile):
    num_not_matched_borders = 0
    for border in tile.get_borders():
        if len(border2tile[border]) < 2 and len(border2tile[border[::-1]]) < 2:
            num_not_matched_borders += 1
    return num_not_matched_borders == 2


corner_tiles = [tile.tile_id for tile in tiles.values() if is_corner_tile(tile)]
print("Corner tiles:", corner_tiles)
print("Corner tile product:", reduce(operator.mul, corner_tiles, 1))

op_order_for_all_orientations = ["rotate"] * 4 + ["flip"] + ["rotate"] * 4

top_left = tiles[corner_tiles[0]]
ops = iter(op_order_for_all_orientations)
while (
    len(border2tile[top_left.get_right_border()])
    + len(border2tile[top_left.get_bottom_border()])
    != 4
):
    getattr(top_left, next(ops))()


placed_tiles = [[top_left]]
for row in range(12):
    for col in range(1, 12):
        left_tile = placed_tiles[row][col - 1]
        join_border = left_tile.get_right_border()
        right_tile = next(
            tile
            for tile in border2tile[join_border] | border2tile[join_border[::-1]]
            if tile is not left_tile
        )

        ops = iter(op_order_for_all_orientations)
        while (
            join_border != right_tile.get_left_border()
            or (
                row > 0
                and placed_tiles[row - 1][col].get_bottom_border()
                != right_tile.get_top_border()
            )
            or (row == 0 and len(border2tile[right_tile.get_top_border()]) > 1)
        ):
            op = next(ops)
            getattr(right_tile, op)()
        placed_tiles[row].append(right_tile)

    if row < 11:
        top_tile = placed_tiles[row][0]
        join_border = top_tile.get_bottom_border()
        bottom_tile = next(
            tile
            for tile in border2tile[join_border] | border2tile[join_border]
            if tile is not top_tile
        )
        ops = iter(op_order_for_all_orientations)
        while (
            join_border != bottom_tile.get_top_border()
            or len(border2tile[bottom_tile.get_right_border()]) != 2
        ):
            op = next(ops)
            getattr(bottom_tile, op)()

        placed_tiles.append([bottom_tile])

picture = Tile(
    None,
    [
        "".join(col.core_region()[i] for col in row)
        for row in placed_tiles
        for i in range(8)
    ],
)

sea_monster = re.compile(
    "(..................#.)(?:.|\n){77}(#....##....##....###)(?:.|\n){77}(.#..#..#..#..#..#...)"
)
ops = iter(op_order_for_all_orientations)
while not sea_monster.search(str(picture)):
    op = next(ops)
    getattr(picture, op)()

count = 0
x = sea_monster.search(str(picture))
while x is not None:
    count += 1
    x = sea_monster.search(str(picture), x.end(1) + 1)

print("Water roughness:", str(picture).count("#") - 15 * count)
