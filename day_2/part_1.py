filename = "final.txt"

with open(f"day_2/{filename}", "r") as file:
    lines = file.read().splitlines()


limits = {
    "red": 12,
    "green": 13,
    "blue": 14,
}

def round_is_valid(round: str) -> bool:
    parts = round.split(", ")

    for part in parts:
        count, colour = part.strip().split(" ")
        count = int(count)

        if limits[colour] < count:
            return False

    return True


def game_is_valid(game_info: str) -> bool:
    rounds = game_info.split("; ")

    # Whoops overriding keyword
    for round in rounds:
        if not round_is_valid(round):
            return False

    return True

# Whoops overriding keyword
sum = 0

for game in lines:
    name, game_info = game.split(": ")

    game_id = int(name.split(" ")[1])

    if game_is_valid(game_info):
        sum += game_id


print(sum)
