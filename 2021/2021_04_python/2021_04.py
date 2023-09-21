from copy import deepcopy
from board import Board

def parse_random_numbers(numbers: str) -> list[list[int]]:
    splitted = numbers.split(",")
    return [int(n) for n in splitted]


def parse_boards(boards: list[str]) -> list[Board]:
    return [Board.from_str_board(b) for b in boards]

def part1(random_numbers, boards):
    for num in random_numbers:
        for board in boards:
            board.mark_number(num)
            winner, score = board.is_winner()
            if winner:
                print("Part 1:", score * num)
                return
            
def  part2(random_numbers: list[int], boards: list[Board]):
    winners = set()
    last_winner_score = 0
    for num in random_numbers:
        for i, board in enumerate(boards):
            board.mark_number(num)
            winner, score = board.is_winner()
            if winner and i not in winners:
                winners.add(i)
                last_winner_score, winner = score * num, board

    print("Part 2:", last_winner_score)


split_input = open("input.txt", "r").read().split("\n\n")
random_numbers = parse_random_numbers(split_input[0])
boards = parse_boards(split_input[1:])

part1(deepcopy(random_numbers), deepcopy(boards))
part2(random_numbers, boards)
    
