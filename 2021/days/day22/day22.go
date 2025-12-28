package day22

import (
	"aoc_2021/input"
	"fmt"
	"strconv"
	"strings"
)

type Cuboid struct {
	value  bool
	xStart int
	xEnd   int
	yStart int
	yEnd   int
	zStart int
	zEnd   int
}

func (c *Cuboid) copy() Cuboid {
	return Cuboid{
		c.value,
		c.xStart, c.xEnd,
		c.yStart, c.yEnd,
		c.zStart, c.zEnd,
	}
}

func (c *Cuboid) copySimple() Cuboid {
	return Cuboid{
		value:  c.value,
		xStart: max(c.xStart, -50),
		xEnd:   min(c.xEnd, 50),
		yStart: max(c.yStart, -50),
		yEnd:   min(c.yEnd, 50),
		zStart: max(c.zStart, -50),
		zEnd:   min(c.zEnd, 50),
	}
}

func (c *Cuboid) size() int {
	x := c.xEnd - c.xStart + 1
	y := c.yEnd - c.yStart + 1
	z := c.zEnd - c.zStart + 1

	if x < 0 || y < 0 || z < 0 {
		return 0
	}

	return x * y * z
}

type Coord struct {
	x int
	y int
	z int
}

func parseRange(str string) (start int, end int) {
	split := strings.Split(str[2:], "..")
	start, err := strconv.Atoi(split[0])
	if err != nil {
		panic(err)
	}
	end, err = strconv.Atoi(split[1])
	if err != nil {
		panic(err)
	}
	return start, end
}

func parseInput(str string) []Cuboid {
	procedure := []Cuboid{}

	for line := range strings.SplitSeq(str, "\n") {
		if line == "" {
			continue
		}
		valueSplit := strings.Split(line, " ")
		value := false
		if valueSplit[0] == "on" {
			value = true
		}
		coordSplit := strings.Split(valueSplit[1], ",")
		xStart, xEnd := parseRange(coordSplit[0])
		yStart, yEnd := parseRange(coordSplit[1])
		zStart, zEnd := parseRange(coordSplit[2])
		procedure = append(procedure, Cuboid{value, xStart, xEnd, yStart, yEnd, zStart, zEnd})
	}

	return procedure
}

func overlappingRange(x1, x2, y1, y2 int) bool {
	return (x1 >= y1 && x1 <= y2) ||
		(x2 >= y1 && x2 <= y2) ||
		(y1 >= x1 && y1 <= x2) ||
		(y2 >= x1 && y2 <= x2) ||
		(x1 < y1 && x2 > y2) ||
		(y1 < x1 && y2 > x2)
}

func cuboidsIntersect(a Cuboid, b Cuboid) bool {
	if !overlappingRange(a.xStart, a.xEnd, b.xStart, b.xEnd) {
		return false
	}
	if !overlappingRange(a.yStart, a.yEnd, b.yStart, b.yEnd) {
		return false
	}
	if !overlappingRange(a.zStart, a.zEnd, b.zStart, b.zEnd) {
		return false
	}
	return true
}

func splitOuterCuboid(outer Cuboid, inner Cuboid) []Cuboid {
	split := []Cuboid{}

	//    _ _ _
	// 1 |_|_|_|
	// 3 |_|x|_| 4 // x: 5 front on z-axis, 6 behind on z-axis
	// 2 |_|_|_| x
	//	 y

	// split 1
	top := outer.copy()
	top.yEnd = inner.yStart - 1
	if top.size() > 0 {
		split = append(split, top)
	}

	// split 2
	bot := outer.copy()
	bot.yStart = inner.yEnd + 1
	if bot.size() > 0 {
		split = append(split, bot)
	}

	// split 3
	midLeft := outer.copy()
	midLeft.xEnd = inner.xStart - 1
	midLeft.yStart = max(outer.yStart, inner.yStart)
	midLeft.yEnd = min(outer.yEnd, inner.yEnd)
	if midLeft.size() > 0 {
		split = append(split, midLeft)
	}

	// split 4
	midRight := outer.copy()
	midRight.xStart = inner.xEnd + 1
	midRight.yStart = max(outer.yStart, inner.yStart)
	midRight.yEnd = min(outer.yEnd, inner.yEnd)
	if midRight.size() > 0 {
		split = append(split, midRight)
	}

	// split 5
	midFront := outer.copy()
	midFront.xStart = max(outer.xStart, inner.xStart)
	midFront.xEnd = min(outer.xEnd, inner.xEnd)
	midFront.yStart = max(outer.yStart, inner.yStart)
	midFront.yEnd = min(outer.yEnd, inner.yEnd)
	midFront.zStart = outer.zStart
	midFront.zEnd = inner.zStart - 1
	if midFront.size() > 0 {
		split = append(split, midFront)
	}

	// split 6
	midBack := outer.copy()
	midBack.xStart = max(outer.xStart, inner.xStart)
	midBack.xEnd = min(outer.xEnd, inner.xEnd)
	midBack.yStart = max(outer.yStart, inner.yStart)
	midBack.yEnd = min(outer.yEnd, inner.yEnd)
	midBack.zStart = inner.zEnd + 1
	midBack.zEnd = outer.zEnd
	if midBack.size() > 0 {
		split = append(split, midBack)
	}

	return split
}

func runProcedure(procedure []Cuboid, simple bool) int {
	cuboids := []Cuboid{}

	for _, proc := range procedure {
		if simple {
			proc = proc.copySimple()
			if proc.size() < 1 {
				continue
			}
		}

		// overlap with existing cuboids are limited, and so will the growth be (~6 split, ~8 neighbors)
		nextCuboids := make([]Cuboid, 0, len(cuboids)+(6*8))

		// for each cuboids, check for intersect
		for _, c := range cuboids {
			if !cuboidsIntersect(c, proc) {
				nextCuboids = append(nextCuboids, c)
				continue
			}
			split := splitOuterCuboid(c, proc)
			nextCuboids = append(nextCuboids, split...)
		}

		if proc.value {
			nextCuboids = append(nextCuboids, proc)
		}

		cuboids = nextCuboids
	}

	count := 0
	for _, c := range cuboids {
		count += c.size()
	}
	return count
}

func Run() {
	fmt.Println("Day 22: Reactor Reboot")

	inputString := input.ReadDay("day22")
	procedure := parseInput(inputString)
	fmt.Printf("Part One: %d\n", runProcedure(procedure, true))
	fmt.Printf("Part Two: %d\n", runProcedure(procedure, false))
}
