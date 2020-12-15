import pytest

from day15 import play_memory


@pytest.mark.parametrize(
    "starting_numbers,expected",
    [
        ([1, 3, 2], 1),
        ([2, 1, 3], 10),
        ([1, 2, 3], 27),
        ([2, 3, 1], 78),
        ([3, 2, 1], 438),
        ([3, 1, 2], 1836),
    ],
)
def test_play_memory(starting_numbers, expected):
    assert play_memory(starting_numbers, 2020) == expected
