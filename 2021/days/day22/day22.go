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
	if c.xStart < -50 {
		c.xStart = -50
	}
	if c.xEnd > 50 {
		c.xEnd = 50
	}
	if c.yStart < -50 {
		c.yStart = -50
	}
	if c.yEnd > 50 {
		c.yEnd = 50
	}
	if c.zStart < -50 {
		c.zStart = -50
	}
	if c.zEnd > 50 {
		c.zEnd = 50
	}
}

func runProcedure(procedure []Cuboid, simple bool) int {
	reactor := map[Coord]bool{}

	for _, proc := range procedure {
		if simple {
			simplifyCuboid(&proc)
		}
		for x := proc.xStart; x <= proc.xEnd; x++ {
			for y := proc.yStart; y <= proc.yEnd; y++ {
				for z := proc.zStart; z <= proc.zEnd; z++ {
					coord := Coord{x, y, z}
					reactor[coord] = proc.value
				}
			}
		}
	}

	count := 0
	for _, v := range reactor {
		if v {
			count += 1
		}
	}
	return count
}

func Run() {
	fmt.Println("Day 22: Reactor Reboot")

	inputString := input.ReadDay("day22")
	procedure := parseInput(inputString)
	fmt.Printf("Part One: %d\n", runProcedure(procedure, true))
	procedure = parseInput(inputString)
	fmt.Printf("Part Two: %d\n", runProcedure(procedure, false))
}
