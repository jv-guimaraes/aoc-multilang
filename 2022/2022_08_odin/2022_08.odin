package main

import "core:fmt"
import "core:strings"

INPUT :: #load("input.txt", string)
DIRECTIONS: [4][2]int : {{1, 0}, {-1, 0}, {0, 1}, {0, -1}}

parse_input :: proc() -> [][]int {
    lines := strings.split_lines(INPUT)
    width, height := len(lines[0]), len(lines)
    
    forest := make([][]int, height)
    for col in 0..<height {
        forest[col] = make([]int, width)
        for row in 0..<width {
            forest[col][row] = int(lines[col][row] - 48)
        }
    }
    
    return forest[:]
}

visible_trees :: proc(forest: [][]int) -> int {
    width, height := len(forest[0]), len(forest)
    is_visible := make([][]bool, height)
    for _, row in is_visible do is_visible[row] = make([]bool, width)
    
    tallest_tree1, tallest_tree2: int = -1, -1

    // From the left and from the top
    for y in 0..<height {
        for x in 0..<width {
            if forest[y][x] > tallest_tree1 {
                is_visible[y][x] = true
                tallest_tree1 = forest[y][x]
            }
            if forest[x][y] > tallest_tree2 {
                is_visible[x][y] = true
                tallest_tree2 = forest[x][y]
            }
        }
        tallest_tree1, tallest_tree2 = -1, -1
    }

    // From the right and from the bottom
    for y := height-1; y >= 0; y -= 1 {
        for x := width-1; x >= 0; x -= 1 {
            if forest[y][x] > tallest_tree1 {
                is_visible[y][x] = true
                tallest_tree1 = forest[y][x]
            }
            if forest[x][y] > tallest_tree2 {
                is_visible[x][y] = true
                tallest_tree2 = forest[x][y]
            }
        }
        tallest_tree1, tallest_tree2 = -1, -1
    }

    total := 0
    for row in is_visible {
        for col in row {
            if col == true do total += 1
        }
    }
    return total
}

scenic_score :: proc(forest: [][]int, start_y, start_x: int) -> int {
    highest_score := 0
    width, height := len(forest[0]), len(forest)
    tree := forest[start_y][start_x]
    scores := [4]int{}

    for dir, score_ix in DIRECTIONS {
        dir_y, dir_x := dir[1], dir[0]
        y, x := start_y + dir_y, start_x + dir_x
        for y >= 0 && x >= 0 && y < height && x < width {
            scores[score_ix] += 1
            if forest[y][x] >= tree do break
            y, x = y + dir_y, x + dir_x
        }
    }
    return scores[0] * scores[1] * scores[2] * scores[3]
}

main :: proc() {
    forest := parse_input()
    fmt.println("Part 1:", visible_trees(forest))
    
    highest_score := 0
    for _, y in forest {
        for _, x in forest[y] {
            score := scenic_score(forest, y, x)
            if score > highest_score do highest_score = score
        }
    }
    fmt.println("Part 2:", highest_score)
}