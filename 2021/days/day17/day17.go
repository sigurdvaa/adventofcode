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
	y2 = value

	value, err = strconv.Atoi(ysplit[1])
	if err != nil {
		log.Fatal(err)
	}
	y1 = value

	return x1, x2, y1, y2
}

func launchProbe(x1, x2, xv, y1, y2, yv int) (int, bool) {
	highestY := 0
	x := 0
	y := 0
	hit := false

	for {
		x += xv
		y += yv

		if y > highestY {
			highestY = y
		}

		if x > x2 || y < y2 {
			break
		}

		if x >= x1 && y <= y1 {
			hit = true
			break
		}

		yv -= 1
		if xv > 0 {
			xv -= 1
		}
	}

	return highestY, hit
}

func highestPosition(x1, x2, y1, y2 int) (int, int) {
	highestY := 0
	hits := 0
	minY, maxY := y2, -y2
	minX, maxX := 0, x2
	xsum := 0
	for s := range x1 {
		xsum += s
		if xsum >= x1 {
			minX = s
			break
		}
	}

	for yv := minY; yv <= maxY; yv++ {
		for xv := minX; xv <= maxX; xv++ {
			currHighestY, hit := launchProbe(x1, x2, xv, y1, y2, yv)
			if hit {
				hits += 1
				if currHighestY > highestY {
					highestY = currHighestY
				}
			}
		}
	}

	return highestY, hits
}

func Run() {
	fmt.Println("Day 17: Trick Shot")

	inputString := input.ReadDay("day17")
	x1, x2, y1, y2 := parseInput(inputString)

	highestY, hits := highestPosition(x1, x2, y1, y2)
	fmt.Printf("Part One: %d\n", highestY)
	fmt.Printf("Part Two: %d\n", hits)
}
