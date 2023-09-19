package main

import "core:fmt"
import "core:strings"
import "core:slice"

input :: #load("input.txt", string)
Stacks :: [dynamic][dynamic]u8

clone_stacks :: proc(stacks: Stacks) -> (res: Stacks) {
    for row, i in stacks {
        append(&res, [dynamic]u8{})
        for col in row do append(&res[i], col)
    }
    return res
}

str_to_int :: proc(str: string) -> (res: int) {
    place_value := 1
    #reverse for v in str {
        res += (cast(int) v - 48) * place_value
        place_value *= 10
    }
    return
}

parse_input:: proc(input: string) -> (stacks: Stacks, moves: [dynamic]string) {
    lines := strings.split_lines(input)

    #reverse for line in lines {
        if len(line) == 0 do continue
        if line[0] == 'm' do append(&moves, line)
        else if strings.contains_rune(line, '[') {
            for i, j := 1, 0; i < len(line); i, j = i+4, j+1 {
                box := line[i]
                if len(stacks)-1 < j do append(&stacks, make([dynamic]u8))
                if box != 32 do append(&stacks[j], box)
            }
        }
    }
    slice.reverse(moves[:])
    return stacks, moves
}

print_stacks :: proc(stacks: Stacks) {
    for stack in stacks {
        for box in stack {
            fmt.print(rune(box), "")
        }
        fmt.println()
    }
    fmt.println()
}

apply_move :: proc(stacks: ^Stacks, move: string) {
    stacks := stacks^
    split_move := strings.split(move, " ")
    num_of_boxes := str_to_int(split_move[1])
    from := str_to_int(split_move[3]) - 1
    to := str_to_int(split_move[5]) - 1

    for _ in 0..<num_of_boxes {
        box := pop(&stacks[from])
        append(&stacks[to], box)
    }
}

apply_move2 :: proc(stacks: ^Stacks, move: string) {
    stacks := stacks^
    split_move := strings.split(move, " ")
    num_of_boxes := str_to_int(split_move[1])
    from := str_to_int(split_move[3]) - 1
    to := str_to_int(split_move[5]) - 1
    temp_stack: [dynamic]u8
    for _ in 0..<num_of_boxes {
        box := pop(&stacks[from])
        append(&temp_stack, box)
    }
    slice.reverse(temp_stack[:])
    append(&stacks[to], ..temp_stack[:])
}

main :: proc() {
    stacks, moves := parse_input(input)
    stacks2 := clone_stacks(stacks)
    
    for move in moves do apply_move(&stacks, move)
    fmt.print("Part 1: ")
    for stack in stacks do fmt.print(cast(rune) stack[len(stack)-1])

    for move in moves do apply_move2(&stacks2, move)
    fmt.print("\nPart 2: ")
    for stack in stacks2 do fmt.print(cast(rune) stack[len(stack)-1])
}