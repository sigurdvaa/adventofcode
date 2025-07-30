package day05

import (
	"aoc_2021/input"
	"fmt"
	"log"
	"strconv"
	"strings"
)

type Pos struct {
	x int
	y int
}

type Vent struct {
	start Pos
	end   Pos
}

func parsePos(str string) Pos {
	split := strings.Split(str, ",")

	x, err := strconv.Atoi(split[0])
	if err != nil {
		log.Fatalln(err)
	}

	y, err := strconv.Atoi(split[1])
	if err != nil {
		log.Fatalln(err)
	}

	return Pos{x, y}
}

func parseInput(stringInput string) []Vent {
	var vents []Vent
	for line := range strings.SplitSeq(stringInput, "\n") {
		if line == "" {
			continue
		}
		split := strings.Split(line, " -> ")
		start := parsePos(split[0])
		end := parsePos(split[1])
		vents = append(vents, Vent{start, end})
	}
	return vents
}

func dangerousStraightVentsScore(vents []Vent) int {
	lines := make(map[Pos]int)

	for _, vent := range vents {
		if vent.start.x == vent.end.x {
			y1 := vent.start.y
			y2 := vent.end.y
			if y1 > y2 {
				y1, y2 = y2, y1
			}

			for y := y1; y <= y2; y++ {
				lines[Pos{vent.start.x, y}] += 1
			}

		} else if vent.start.y == vent.end.y {
			x1 := vent.start.x
			x2 := vent.end.x
			if x1 > x2 {
				x1, x2 = x2, x1
			}
			for x := x1; x <= x2; x++ {
				lines[Pos{x, vent.start.y}] += 1
			}
		}
	}

	score := 0
	for _, v := range lines {
		if v >= 2 {
			score += 1
		}
	}

	return score
}

func dangerousVentsScore(vents []Vent) int {
	lines := make(map[Pos]int)

	for _, vent := range vents {
		if vent.start.x == vent.end.x {
			y1 := vent.start.y
			y2 := vent.end.y
			if y1 > y2 {
				y1, y2 = y2, y1
			}

			for y := y1; y <= y2; y++ {
				lines[Pos{vent.start.x, y}] += 1
			}

		} else if vent.start.y == vent.end.y {
			x1 := vent.start.x
			x2 := vent.end.x
			if x1 > x2 {
				x1, x2 = x2, x1
			}
			for x := x1; x <= x2; x++ {
				lines[Pos{x, vent.start.y}] += 1
			}
		} else {
			x := vent.start.x
			y := vent.start.y
			if vent.start.x < vent.end.x && vent.start.y < vent.end.y {
				for x <= vent.end.x && y <= vent.end.y {
					lines[Pos{x, y}] += 1
					x += 1
					y += 1
				}
			} else if vent.start.x > vent.end.x && vent.start.y > vent.end.y {
				for x >= vent.end.x && y >= vent.end.y {
					lines[Pos{x, y}] += 1
					x -= 1
					y -= 1
				}
			} else if vent.start.x < vent.end.x && vent.start.y > vent.end.y {
				for x <= vent.end.x && y >= vent.end.y {
					lines[Pos{x, y}] += 1
					x += 1
					y -= 1
				}
			} else if vent.start.x > vent.end.x && vent.start.y < vent.end.y {
				for x >= vent.end.x && y <= vent.end.y {
					lines[Pos{x, y}] += 1
					x -= 1
					y += 1
				}
			} else {
				panic("unreachable")
			}
		}
	}

	score := 0
	for _, v := range lines {
		if v >= 2 {
			score += 1
		}
	}

	return score
}

func Run() {
	fmt.Println("Day 5: Hydrothermal Venture")

	inputString := input.ReadDay("day05")
	vents := parseInput(inputString)

	fmt.Printf("Part One: %d\n", dangerousStraightVentsScore(vents))
	fmt.Printf("Part Two: %d\n", dangerousVentsScore(vents))
}
