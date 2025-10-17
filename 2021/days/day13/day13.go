package day13

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

func parseInput(str string) ([]Pos, []Pos) {
	dots := []Pos{}
	folds := []Pos{}
	dotsDone := false

	for line := range strings.SplitSeq(str, "\n") {
		if line == "" {
			dotsDone = true
			continue
		}

		if !dotsDone {
			split := strings.Split(line, ",")
			x, err := strconv.Atoi(split[0])
			if err != nil {
				log.Fatal(err)
			}
			y, err := strconv.Atoi(split[1])
			if err != nil {
				log.Fatal(err)
			}
			dots = append(dots, Pos{x, y})
		} else {
			split := strings.Split(line, "=")
			n, err := strconv.Atoi(split[1])
			if err != nil {
				log.Fatal(err)
			}
			if split[0][len(split[0])-1:] == "x" {
				folds = append(folds, Pos{x: n, y: 0})
			} else {
				folds = append(folds, Pos{x: 0, y: n})
			}
		}

	}

	return dots, folds
}

func dotsAfterFold(dots []Pos, folds []Pos) map[Pos]struct{} {
	dotsSet := map[Pos]struct{}{}
	for _, pos := range dots {
		dotsSet[pos] = struct{}{}
	}

	for _, fold := range folds {
		nextDotsSet := map[Pos]struct{}{}
		for dot := range dotsSet {
			if fold.x != 0 && dot.x > fold.x {
				n := fold.x - (dot.x - fold.x)
				nextDotsSet[Pos{x: n, y: dot.y}] = struct{}{}
			} else if fold.y != 0 && dot.y > fold.y {
				n := fold.y - (dot.y - fold.y)
				nextDotsSet[Pos{x: dot.x, y: n}] = struct{}{}
			} else {
				nextDotsSet[Pos{x: dot.x, y: dot.y}] = struct{}{}
			}
		}
		dotsSet = nextDotsSet
	}

	return dotsSet
}

func printDots(dotsSet map[Pos]struct{}) {
	xMax := 0
	yMax := 0
	for dot := range dotsSet {
		if dot.x > xMax {
			xMax = dot.x
		}
		if dot.y > yMax {
			yMax = dot.y
		}
	}

	dots := [][]string{}
	for range yMax + 1 {
		row := []string{}
		for range xMax + 1 {
			row = append(row, ".")
		}
		dots = append(dots, row)
	}

	for dot := range dotsSet {
		dots[dot.y][dot.x] = "#"
	}

	for _, row := range dots {
		fmt.Println(row)
	}
}

func Run() {
	fmt.Println("Day 13: Transparent Origami")

	inputString := input.ReadDay("day13")
	dots, folds := parseInput(inputString)

	fmt.Printf("Part One: %d\n", len(dotsAfterFold(dots, folds[:1])))
	dotsSet := dotsAfterFold(dots, folds)
	fmt.Printf("Part Two: \n")
	printDots(dotsSet)
}
