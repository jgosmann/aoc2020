def play_memory(starting_numbers, n_rounds):
    spoken_numbers = {
        num: round_index for round_index, num in enumerate(starting_numbers[:-1])
    }
    previous_number = starting_numbers[-1]
    for round_index in range(len(starting_numbers) - 1, n_rounds - 1):
        if previous_number in spoken_numbers:
            current_number = round_index - spoken_numbers[previous_number]
        else:
            current_number = 0
        spoken_numbers[previous_number] = round_index
        previous_number = current_number
    return previous_number


if __name__ == "__main__":
    print(play_memory([10, 16, 6, 0, 1, 17], 2020))
    print(play_memory([10, 16, 6, 0, 1, 17], 30_000_000))
