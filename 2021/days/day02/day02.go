package day02

import (
	"aoc_2021/input"
	"fmt"
	"log"
	"strconv"
	"strings"
)

type pos struct {
	x int
	y int
}

func parseInput(inputString string) []pos {
	var ins []pos
	for _, line := range strings.Split(inputString, "\n") {
		if line == "" {
			continue
		}

		split := strings.Split(line, " ")
		num, err := strconv.Atoi(split[1])
		if err != nil {
			log.Fatal(err)
		}

		x, y := 0, 0
		switch split[0] {
		case "forward":
			x = num
		case "down":
			y = num
		case "up":
			y = num * -1
		default:
			log.Fatalf("unknown direction: %s", split[0])
		}

		ins = append(ins, pos{x, y})
	}
	return ins
}

func positionProduct(ins []pos) int {
	x, y := 0, 0
	for _, pos := range ins {
		x += pos.x
		y += pos.y
	}
	return x * y
}

func Run() {
	fmt.Println("Day 2: Dive!")

	inputString := input.ReadDay(2)
	ins := parseInput(inputString)

	fmt.Printf("Part One: %d\n", positionProduct(ins))
	fmt.Printf("Part Two: TODO\n")
}
