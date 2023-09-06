package main

import "core:fmt"
import "core:strings"

input :: #load("input.txt", string)

bottom, top : u8 : 65, 122
Char_Set :: bit_set[bottom..=top]

repeated_item :: proc(rucksack: string) -> u8 {
    half_len := len(rucksack) / 2
    side_a := charset_from_rucksack(rucksack[:half_len])
    
    for i in half_len..<len(rucksack) {
        if rucksack[i] in side_a do return rucksack[i]
    }
    panic("Didn't find a repeated item")
}

item_priority :: proc(item: u8) -> int {
    switch item {
        case 'A'..='Z':
            return int(item) - 38
        case 'a'..='z':
            return int(item) - 96
        case: panic("Invalid item!")
    }
}

charset_from_rucksack :: proc(rucksack: string) -> (char_set: Char_Set) {
    for _, i in rucksack do char_set += {rucksack[i]}
    return
}

main :: proc() {
    total := 0
    lines := strings.split_lines(input)

    for line in lines {
        total += item_priority(repeated_item(line))
    }
    fmt.println("Part 1:", total)

    // Part 2
    total = 0
    for i := 0; i < len(lines); i += 3 {
        set_a := charset_from_rucksack(lines[i])
        set_b := charset_from_rucksack(lines[i+1])
        set_c := charset_from_rucksack(lines[i+2])
        intersect := set_a & set_b & set_c
        for item in bottom..=top {
            if item in intersect do total += item_priority(item)
        }
    }
    fmt.println("Part 2:", total)
}