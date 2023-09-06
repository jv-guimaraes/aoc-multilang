package main

import "core:fmt"
import "core:strings"

input :: #load("input.txt", string)

Hand :: enum {
    rock = 1, paper, scissors
}

loses_to :: proc(hand: Hand) -> (opp: Hand) {
    switch hand {
        case .rock: opp = .scissors
        case .paper: opp = .rock
        case .scissors: opp = .paper
    }
    return
}

wins_against :: proc(hand: Hand) -> (opp: Hand) {
    switch hand {
        case .rock: opp = .paper
        case .paper: opp = .scissors
        case .scissors: opp = .rock
    }
    return
}

char_to_hand :: proc(char: u8) -> (hand: Hand) {
    switch char {
        case 'A', 'X': hand = .rock
        case 'B', 'Y': hand = .paper
        case 'C', 'Z': hand = .scissors
        case: panic("Invalid char.")
    }
    return
}

game_score :: proc(line: string) -> int {
    if len(line) == 0 do return 0
    enemy, player := char_to_hand(line[0]), char_to_hand(line[2])
    // Draw
    if player == enemy do return int(player) + 3
    // Victory
    if enemy == loses_to(player) do return int(player) + 6
    // Defeat
    return int(player) + 0
}

game_score2 :: proc(line: string) -> int {
    if len(line) == 0 do return 0
    enemy := char_to_hand(line[0])
    outcome := line[2]
    switch outcome {
        case 'X': return int(loses_to(enemy)) + 0
        case 'Y': return int(enemy) + 3
        case 'Z': return int(wins_against(enemy)) + 6
        case: panic("Invalid outcome char. Must be X, Y or Z")
    }
}

main :: proc() {
    total_score := 0
    for line in strings.split_lines(input) do total_score += game_score(line)
    fmt.println("Part 1:", total_score)

    total_score = 0
    for line in strings.split_lines(input) do total_score += game_score2(line)
    fmt.println("Part 2:", total_score)
}