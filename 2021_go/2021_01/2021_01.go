package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func parse_input() []int {
	input, _ := os.ReadFile("input.txt")
	splitted := strings.Split(string(input), "\r\n")
	var res []int
	for _, v := range splitted {
		if len(v) == 0 {
			continue
		}
		depth, _ := strconv.Atoi(v)
		res = append(res, depth)
	}
	return res
}

func sum(slice []int) int {
	sum := 0
	for _, v := range slice {
		sum += v
	}
	return sum
}

func number_of_depth_increases(input []int, window int) int {
	count := 0
	for i := 0; i < len(input); i++ {
		window1 := input[i : i+window]
		window2 := input[i+1 : i+window+1]
		if sum(window1) < sum(window2) {
			count++
		}
	}
	return count
}

func main() {
	input := parse_input()
	fmt.Println("Part 1:", number_of_depth_increases(input, 1))
	fmt.Println("Part 2:", number_of_depth_increases(input, 3))
}
