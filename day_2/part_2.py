from functools import reduce

filename = "final.txt"

with open(f"day_2/{filename}", "r") as file:
    lines = file.read().splitlines()


def game_to_minima(game_info: str) -> dict[str, int]:
    rounds = game_info.split("; ")

    minima = {
        "red": 0,
        "green": 0,
        "blue": 0,
    }

    for round in rounds:
        parts = round.split(", ")

        for part in parts:
            count, colour = part.split(" ")
            count = int(count)

            minima[colour] = max(minima[colour], count)

    return minima


def product_of_numbers(numbers):
    return reduce(lambda count, acc: count * acc, numbers, 1)


def minima_to_power(minima: dict[str, int]) -> int:
    return product_of_numbers(minima.values())


sum_of_powers = 0
for game in lines:
    name, game_info = game.split(": ")

    game_id = int(name.split(" ")[1])


    minima = game_to_minima(game_info)

    power = minima_to_power(minima)

    sum_of_powers += power

print(sum_of_powers)

