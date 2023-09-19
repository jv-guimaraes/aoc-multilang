package main

import "core:fmt"
import "core:strings"

input :: #load("input.txt", string)

Range :: struct {
    bottom, top: int
}

str_to_int :: proc(str: string) -> (res: int) {
    place_value := 1
    #reverse for v in str {
        res += (cast(int) v - 48) * place_value
        place_value *= 10
    }
    return
}

str_to_range :: proc(str: string) -> Range {
    split := strings.split(str, "-")
    return Range {str_to_int(split[0]), str_to_int(split[1])}
}

contains :: proc(r1, r2: Range) -> bool {
    if r2.bottom >= r1.bottom && r2.top <= r1.top {
        return true
    }
    return false
}

overlap :: proc(r1, r2: Range) -> bool {
    if r1.bottom >= r2.bottom && r1.bottom <= r2.top do return true
    if r2.bottom >= r1.bottom && r2.bottom <= r1.top do return true
    if r1.top <= r2.top && r1.top >= r2.bottom do return true
    if r2.top <= r1.top && r2.top >= r1.bottom do return true
    return false
}

main :: proc() {
    lines := strings.split_lines(input)
    
    total := 0
    for line in lines {
        split := strings.split(line, ",")
        r1, r2 := str_to_range(split[0]), str_to_range(split[1])
        if contains(r1, r2) || contains(r2, r1) do total += 1
    }
    fmt.println("Part 1:", total)

    total = 0
    for line in lines {
        split := strings.split(line, ",")
        r1, r2 := str_to_range(split[0]), str_to_range(split[1])
        if overlap(r1, r2) do total += 1
    }
    fmt.println("Part 2:", total)
}