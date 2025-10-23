package day17

import (
	"aoc_2021/input"
	"fmt"
	"log"
	"strconv"
	"strings"
)

func parseInput(str string) (int, int, int, int) {
	var x1, x2, y1, y2 int

	str = strings.TrimSpace(str)
	split := strings.Split(str, " ")
	xsplit := strings.Split(split[2][2:len(split[2])-1], "..")
	ysplit := strings.Split(split[3][2:len(split[3])], "..")

	value, err := strconv.Atoi(xsplit[0])
	if err != nil {
		log.Fatal(err)
	}
	x1 = value

	value, err = strconv.Atoi(xsplit[1])
	if err != nil {
		log.Fatal(err)
	}
	x2 = value

	value, err = strconv.Atoi(ysplit[0])
	if err != nil {
		log.Fatal(err)
	}
	y1 = value

	value, err = strconv.Atoi(ysplit[1])
	if err != nil {
		log.Fatal(err)
	}
	y2 = value

	return x1, x2, y1, y2
}

func highestPosition(x1, x2, y1, y2 int) int {

	minX := 0
	maxX := x2 / 2
	s := 0
	sum := 0
	for {
		s += 1
		sum += s
		if sum >= x1 {
			minX = s
			break
		}
	}

	fmt.Println(x1, x2, minX, maxX)

	return 0
}

func Run() {
	fmt.Println("Day 17: Trick Shot")

	inputString := input.ReadDay("day17")
	x1, x2, y1, y2 := parseInput(inputString)

	fmt.Printf("Part One: %d\n", highestPosition(x1, x2, y1, y2))
	fmt.Printf("Part Two: TODO\n")
}
