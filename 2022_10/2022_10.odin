package main

import "core:fmt"
import "core:strings"

INPUT :: #load("input.txt", string)
CYCLES :: [?]int{20, 60, 100, 140, 180, 220}

InstructionType :: enum {noop, add}
Instruction :: struct {type: InstructionType, value: int}

string_to_int :: proc(str: string) -> (res: int) {
    place_value := 1
    is_negative := (str[0] == '-')
    #reverse for v in str {
        if v == '-' do break
        res += (cast(int) v - 48) * place_value
        place_value *= 10
    }
    if is_negative do return res * -1
    else do return res
}

parse :: proc(input: string) -> []Instruction {
    res := [dynamic]Instruction{}
    for line in strings.split_lines(input) {
        if line[0] == 'n' do append(&res, Instruction{.noop, 0})
        else {
            split := strings.split(line, " ")
            value := string_to_int(split[1])
            append(&res, Instruction{.add, value})
        }
    }
    return res[:]
}

compute_signal :: proc(instructions: []Instruction) -> ([]int) {
    x := 1
    cycle := 0
    res := [dynamic]int{}
    
    for instruction in instructions {
        if instruction.type == .noop {
            cycle += 1
            append(&res, x)
        }
        else if instruction.type == .add {
            for i in 0..<2 {
                cycle += 1
                append(&res, x)
            }
            x += instruction.value
        }
    }
    // append(&res, x)
    return res[:]
}

render_signals :: proc(signals: []int) {
    for i in 0..<6 {
        for j in 0..<40 {
            index := j + (i * 40)
            x := signals[index]
            pixel := j
            if (pixel == x - 1) || (pixel == x) || (pixel == x + 1) {
                fmt.print("#")
            } else do fmt.print(".")
        }
        fmt.println()
        // return
    }
}

main :: proc() {
    parsed_input := parse(INPUT)
    computed_signals := compute_signal(parsed_input)
    
    // Part 1
    total := 0
    for c in CYCLES do total += computed_signals[c - 1] * c
    fmt.println("Part 1:", total)

    // Part 2
    fmt.println("\nPart 2:")
    render_signals(computed_signals)
}