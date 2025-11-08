package day20

import (
	"aoc_2021/input"
	"fmt"
	"strings"
)

func parsePixel(p rune) bool {
	switch p {
	case '#':
		return true
	case '.':
		return false
	}
	panic(fmt.Sprintf("unknown pixel: %c", p))
}

func parseInput(str string) ([512]bool, [][]bool) {
	algo_parsed := false
	algo := [512]bool{}
	img := [][]bool{}
	for line := range strings.SplitSeq(str, "\n") {
		if line == "" {
			continue
		}
		if algo_parsed {
			row := []bool{}
			for _, r := range line {
				row = append(row, parsePixel(r))
			}
			img = append(img, row)
		} else {
			for i, r := range line {
				algo[i] = parsePixel(r)
			}
			algo_parsed = true
		}
	}
	return algo, img
}

func calcPixelValue(img [][]bool, x int, y int) int {
	val := 0
	for iy := y - 1; iy < y+2; iy++ {
		for ix := x - 1; ix < x+2; ix++ {
			val <<= 1
			if iy >= 0 && iy < len(img) {
				if ix >= 0 && ix < len(img[iy]) {
					if img[iy][ix] {
						val += 1
					}
				}
			}
		}
	}
	return val
}

func enhanceImg(algo [512]bool, img [][]bool, steps int) [][]bool {
	const PAD int = 1
	for range steps {
		nextY := len(img) + (PAD * 2)
		nextX := len(img[0]) + (PAD * 2)
		next := make([][]bool, nextY)
		for y := range nextY {
			next[y] = make([]bool, nextX)
			for x := range nextX {
				val := calcPixelValue(img, x-PAD, y-PAD)
				next[y][x] = algo[val]
			}
		}
		img = next
	}
	return img
}

func countLitPixels(img [][]bool) int {
	count := 0
	for _, row := range img {
		for _, p := range row {
			if p {
				count += 1
			}
		}
	}
	return count
}

func printImg(img [][]bool) {
	for _, row := range img {
		for _, p := range row {
			if p {
				fmt.Print("#")
			} else {
				fmt.Print(".")
			}
		}
		fmt.Println()
	}
}

func Run() {
	fmt.Println("Day 20: Trench Map")

	inputString := input.ReadDay("day20")
	algo, img := parseInput(inputString)
	img = enhanceImg(algo, img, 2)

	fmt.Printf("Part One: %d\n", countLitPixels(img))
	// 5704 high
	// 5583 high
	fmt.Printf("Part Two: TODO\n")
}
