package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

type Direction string

const (
	forward Direction = "forward"
	up      Direction = "up"
	down    Direction = "down"
)

type Move struct {
	direction Direction
	length    int
}

func parse_input(file string) []Move {
	input, _ := os.ReadFile(file)
	line_split := strings.Split(string(input), "\r\n")
	var moves []Move
	for _, line := range line_split {
		move_split := strings.Split(line, " ")
		direction := move_split[0]
		length, _ := strconv.Atoi(move_split[1])
		switch direction {
		case "forward":
			moves = append(moves, Move{forward, length})
		case "up":
			moves = append(moves, Move{up, length})
		case "down":
			moves = append(moves, Move{down, length})
		}
	}
	return moves
}

func part1(input []Move) {
	var x, y int
	for _, move := range input {
		switch move.direction {
		case forward:
			x += move.length
		case up:
			y -= move.length
		case down:
			y += move.length
		}
	}
	fmt.Println("Part 1:", x*y)
}

func part2(input []Move) {
	var horizontal, depth, aim int
	for _, move := range input {
		switch move.direction {
		case forward:
			horizontal += move.length
			depth += aim * move.length
		case up:
			aim -= move.length
		case down:
			aim += move.length
		}
	}
	fmt.Println("Part 2:", horizontal*depth)
}

func main() {
	input := parse_input("input.txt")
	part1(input)
	part2(input)
}
