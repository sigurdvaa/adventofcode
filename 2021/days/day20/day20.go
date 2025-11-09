package day20

import (
	"aoc_2021/input"
	"fmt"
	"strings"
)

type Img [][]bool
type Algo [512]bool

func parsePixel(p rune) bool {
	switch p {
	case '#':
		return true
	case '.':
		return false
	}
	panic(fmt.Sprintf("unknown pixel: '%c'", p))
}

func parseInput(str string) (Algo, Img) {
	algo_parsed := false
	algo := Algo{}
	img := Img{}
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

func calcPixelValue(img Img, x int, y int, infVal int) int {
	val := 0
	for iy := y - 1; iy < y+2; iy++ {
		for ix := x - 1; ix < x+2; ix++ {
			val <<= 1
			if iy >= 0 && iy < len(img) && ix >= 0 && ix < len(img[iy]) {
				if img[iy][ix] {
					val += 1
				}
			} else {
				val += infVal
			}
		}
	}
	return val
}

func enhanceImg(algo Algo, img Img, steps int) Img {
	infVal := 0
	const PAD int = 1
	for s := range steps {
		nextY := len(img) + (PAD * 2)
		nextX := len(img[0]) + (PAD * 2)
		nextImg := make(Img, nextY)
		for y := range nextY {
			nextImg[y] = make([]bool, nextX)
			for x := range nextX {
				val := calcPixelValue(img, x-PAD, y-PAD, infVal)
				nextImg[y][x] = algo[val]
			}
		}
		img = nextImg

		if s > 0 {
			if infVal == 1 && !algo[511] {
				infVal = 0
			} else if infVal == 0 && algo[0] {
				infVal = 1
			}
		} else {
			if algo[0] {
				infVal = 1
			}
		}
	}
	return img
}

func countLitPixels(img Img) int {
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

func Run() {
	fmt.Println("Day 20: Trench Map")

	inputString := input.ReadDay("day20")
	algo, img := parseInput(inputString)

	fmt.Printf("Part One: %d\n", countLitPixels(enhanceImg(algo, img, 2)))
	fmt.Printf("Part Two: %d\n", countLitPixels(enhanceImg(algo, img, 50)))
}
