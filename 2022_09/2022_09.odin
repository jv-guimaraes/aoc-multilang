package main

import "core:fmt"
import "core:strings"

Vec2 :: [2]int
INPUT :: #load("input.txt", string)

string_to_int :: proc(str: string) -> (res: int) {
    place_value := 1
    #reverse for v in str {
        res += (cast(int) v - 48) * place_value
        place_value *= 10
    }
    return
}

dir_to_vec2 :: proc(dir: u8) -> Vec2 {
    switch dir {
        case 'R': return Vec2{1, 0}
        case 'L': return Vec2{-1, 0}
        case 'U': return Vec2{0, 1}
        case 'D': return Vec2{0, -1}
    }
    panic("Ivalid direction. Couldn't convert to Vec2")
}

distance_to_step :: proc(vec: Vec2) -> Vec2 {
    vec := vec
    x_abs := abs(vec.x)
    y_abs := abs(vec.y)
    switch x_abs + y_abs {
        case 0, 1: vec = Vec2{}
        case 2:
            if x_abs == 2 do vec.x /= 2
            else if y_abs == 2 do vec.y /= 2
            else do vec = Vec2{}
        case 3:
            if x_abs == 2 do vec.x /= 2
            else if y_abs == 2 do vec.y /= 2
        case 4:
            vec /= Vec2{2, 2}
        case:
            panic("Invalid distance vector")

    }
    return vec
}

calculate_moves :: proc(moves: string) -> int {
    head, tail := Vec2{0, 0}, Vec2{0, 0}
    visited := [dynamic]Vec2{}

    for move in strings.split_lines(moves) {
        dir := move[0]
        steps := string_to_int(strings.split(move, " ")[1])
        for _ in 0..<steps {
            head += dir_to_vec2(dir)
            distance := head - tail
            tail += distance_to_step(distance)
            
            found := false
            for t in visited {
                if t == tail {
                    found = true
                    break
                }
            }
            if !found do append(&visited, tail)
        }
    }

    return len(visited)
}

calculate_moves2 :: proc(moves: string) -> int {
    body := [10]Vec2{}
    visited := [dynamic]Vec2{}
    
    // Move the head
    for move in strings.split_lines(moves) {
        dir := move[0]
        steps := string_to_int(strings.split(move, " ")[1])
        for _ in 0..<steps {
            body[0] += dir_to_vec2(dir)
            for i in 1..<len(body) {
                distance := body[i - 1] - body[i]
                body[i] += distance_to_step(distance)
                
                if (i == 9) {
                    found := false
                    for t in visited {
                        if t == body[i] {
                            found = true
                            break
                        }
                    }
                    if !found do append(&visited, body[i])
                }
            }
        }
    }

    return len(visited)
}

main :: proc() {
    fmt.println("Part 1:", calculate_moves(INPUT))
    fmt.println("Part 2:", calculate_moves2(INPUT))
}