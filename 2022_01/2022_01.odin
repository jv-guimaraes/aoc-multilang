package main

import "core:fmt"
import "core:strings"
import "core:slice"

char_to_int :: proc(char: rune) -> int {
    return cast(int) char - 48
}

string_to_int :: proc(str: string) -> (res: int) {
    place_value := 1
    #reverse for v in str {
        res += char_to_int(v) * place_value
        place_value *= 10
    }
    return
}

main :: proc() {
    input := #load("input.txt", string)
    elfs_calories: [dynamic]int
    
    current_elf := 0
    for line in strings.split_lines(input) {
        if len(line) == 0 {
            append(&elfs_calories, current_elf)
            current_elf = 0
        }
        current_elf += string_to_int(line)
    }

    slice.reverse_sort(elfs_calories[:])
    fmt.println("Top elf:", elfs_calories[0])
    top_three := elfs_calories[0] + elfs_calories[1] + elfs_calories[2]
    fmt.println("Top three elfs:", top_three)
}