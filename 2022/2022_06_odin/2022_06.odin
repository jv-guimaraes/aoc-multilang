package main

import "core:fmt"

input :: #load("input.txt", string)
set_bottom, set_top: u8 : 97, 122
Char_Set :: bit_set[set_bottom..=set_top]

has_repeated_char :: proc(buffer: string) -> bool {
    bs: Char_Set
    for _, i in buffer {
        if buffer[i] in bs do return true;
        bs += {buffer[i]}
    }
    return false
}

start_of_unique_n_slice :: proc(buffer: string, n: int) -> int {
    for i in n+1..<len(buffer) {
        if !has_repeated_char(buffer[i-n+1:i+1]) do return i + 1
    }
    panic("Didn't find a marker")
}

main :: proc() {
    fmt.println("Part 1:", start_of_unique_n_slice(input, 4))
    fmt.println("Part 2:", start_of_unique_n_slice(input, 14))
}