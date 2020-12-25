import sys


def transform(value, subject_value):
    return (value * subject_value) % 20201227


def find_loop_size(public_key, subject_value=7):
    value = 1
    loop_size = 0
    while value != public_key:
        value = transform(value, subject_value)
        loop_size += 1
    return loop_size


def find_encryption_key(key, loop_size):
    value = 1
    for _ in range(loop_size):
        value = transform(value, key)
    return value


public_keys = [int(line) for line in sys.stdin]
loop_sizes = [find_loop_size(key) for key in public_keys]
print(find_encryption_key(public_keys[0], loop_sizes[1]))
