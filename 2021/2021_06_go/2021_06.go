package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func sum(numbers []int) int {
	count := 0
	for _, num := range numbers {
		count += num
	}
	return count
}

func parse_input(filename string) []int {
	file, _ := os.ReadFile(filename)
	split_input := strings.Split(string(file), ",")
	var res []int
	for _, num := range split_input {
		parsed_num, _ := strconv.Atoi(num)
		res = append(res, parsed_num)
	}
	return res
}

func simulate_fish(starting_fish []int, days int) int {
	fish_count := make([]int, 9)
	for _, fish := range starting_fish {
		fish_count[fish] += 1
	}

	for i := 0; i < days; i++ {
		zero_fish := fish_count[0]
		for j := 0; j < 8; j++ {
			fish_count[j] = fish_count[j+1]
		}
		fish_count[6] += zero_fish
		fish_count[8] = zero_fish
		// fmt.Println(i+1, ":", fish_count, ":", sum(fish_count))
	}
	return sum(fish_count)
}

func main() {
	starting_fish := parse_input("input.txt")
	fmt.Println(simulate_fish(starting_fish, 256))
}
