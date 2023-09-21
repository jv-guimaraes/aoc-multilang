package main

import (
	"fmt"
	"os"
	"strconv"
	"strings"
)

type Point struct {
	x, y int
}

func (p1 Point) minus(p2 Point) Point {
	return Point{p1.x - p2.x, p1.y - p2.y}
}

type Pair struct {
	src, dst Point
}

func parse_line(points string) (Point, Point) {
	split_points := strings.Split(points, " -> ")
	a_str, b_str := split_points[0], split_points[1]
	a_split, b_split := strings.Split(a_str, ","), strings.Split(b_str, ",")
	ax, _ := strconv.Atoi(a_split[0])
	ay, _ := strconv.Atoi(a_split[1])
	bx, _ := strconv.Atoi(b_split[0])
	by, _ := strconv.Atoi(b_split[1])
	return Point{ax, ay}, Point{bx, by}
}

func parse_input(filename string) []Pair {
	input_bytes, _ := os.ReadFile(filename)
	input := string(input_bytes)
	var res []Pair
	for _, line := range strings.Split(input, "\r\n") {
		a, b := parse_line(line)
		res = append(res, Pair{a, b})
	}
	return res
}

func abs(x int) int {
	if x < 0 {
		return -x
	}
	return x
}

func pair_walk(pair Pair) []Point {
	var res []Point = []Point{pair.src}
	distance := pair.dst.minus(pair.src)
	for distance.x != 0 || distance.y != 0 {
		if distance.x != 0 {
			x := distance.x / abs(distance.x)
			pair.src.x += x
			distance.x -= x
		}
		if distance.y != 0 {
			y := distance.y / abs(distance.y)
			pair.src.y += y
			distance.y -= y
		}
		res = append(res, pair.src)
	}
	return res
}

func visited_more_than_once(pairs []Pair, ignore_diagonals bool) int {
	var visited_points = make(map[Point]int)
	for _, pair := range pairs {
		if ignore_diagonals && (pair.src.x != pair.dst.x) && (pair.src.y != pair.dst.y) {
			continue
		}
		for _, point := range pair_walk(pair) {
			_, in_map := visited_points[point]
			if in_map {
				visited_points[point] += 1
			} else {
				visited_points[point] = 1
			}
		}
	}
	count := 0
	for _, v := range visited_points {
		if v > 1 {
			count++
		}
	}
	return count
}

func main() {
	pairs := parse_input("input.txt")
	fmt.Println("Part 1:", visited_more_than_once(pairs, true))
	fmt.Println("Part 2:", visited_more_than_once(pairs, false))
}
