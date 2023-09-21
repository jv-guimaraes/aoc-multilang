package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

func parse_input(file string) []string {
	input, _ := os.ReadFile(file)
	line_split := strings.Split(string(input), "\r\n")
	return line_split
}

func most_and_least_common(lines []string, col int) (byte, byte) {
	count := 0
	for row := 0; row < len(lines); row++ {
		if lines[row][col] == '1' {
			count++
		} else {
			count--
		}
	}
	if count > 0 {
		return '1', '0'
	}
	if count < 0 {
		return '0', '1'
	}
	return '-', '-'
}

func gamma_epsilon(lines []string) (int64, int64) {
	var gamma, epsilon []byte
	for col := 0; col < len(lines[0]); col++ {
		most, least := most_and_least_common(lines, col)
		gamma = append(gamma, most)
		epsilon = append(epsilon, least)
	}
	gamma_int, _ := strconv.ParseInt(string(gamma), 2, 64)
	epsilon_int, _ := strconv.ParseInt(string(epsilon), 2, 64)
	return gamma_int, epsilon_int
}

func filter(lines []string, col int, bit byte) []string {
	var new_slice []string
	for row := 0; row < len(lines); row++ {
		if lines[row][col] == bit {
			new_slice = append(new_slice, lines[row])
		}
	}
	return new_slice
}

func oxygen_co2(lines []string) (int64, int64) {
	oxygen := make([]string, len(lines))
	copy(oxygen, lines)
	co2 := make([]string, len(lines))
	copy(co2, lines)

	for col := 0; len(oxygen) > 1; col++ {
		most, _ := most_and_least_common(oxygen, col)
		if most == '-' {
			oxygen = filter(oxygen, col, '1')
		} else {
			oxygen = filter(oxygen, col, most)
		}
	}

	for col := 0; len(co2) > 1; col++ {
		_, least := most_and_least_common(co2, col)
		if least == '-' {
			co2 = filter(co2, col, '0')
		} else {
			co2 = filter(co2, col, least)
		}
	}
	oxygen_int, _ := strconv.ParseInt(oxygen[0], 2, 64)
	co2_int, _ := strconv.ParseInt(co2[0], 2, 64)
	return oxygen_int, co2_int
}

func main() {
	lines := parse_input("input.txt")
	g, e := gamma_epsilon(lines)
	fmt.Println("Part 1:", g*e)

	o, c := oxygen_co2(lines)
	fmt.Println("Part 2:", o*c)
}
