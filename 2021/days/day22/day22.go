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

func simplifyCuboid(c *Cuboid) {
	c.xStart = max(c.xStart, -50)
	c.xEnd = min(c.xEnd, 50)
	c.yStart = max(c.yStart, -50)
	c.yEnd = min(c.yEnd, 50)
	c.zStart = max(c.zStart, -50)
	c.zEnd = min(c.zEnd, 50)
}

func intersectingCubes(a Cuboid, b Cuboid) int {
	count := 0
	return count
}

func runProcedure(procedure []Cuboid, simple bool) int {
	cuboids := []Cuboid{}

	for _, proc := range procedure {
		// for each cuboids, check for intersections
		// if "on", add, split cuboids on any intersects
		// if "off", remove, split cuboids on any intersect
	}

	// TODO count cubes
	return 0
}

func Run() {
	fmt.Println("Day 22: Reactor Reboot")

	inputString := input.ReadDay("day22")
	procedure := parseInput(inputString)

	fmt.Printf("Part One: %d\n", runProcedure(procedure, true))
	// procedure = parseInput(inputString)
	// fmt.Printf("Part Two: %d\n", runProcedure(procedure, false))
}
