package day19

import (
	"aoc_2021/input"
	"fmt"
	"log"
	"strconv"
	"strings"
)

type Coord struct {
	x int
	y int
	z int
	s int
}

func parseInput(str string) [][]Coord {
	var scanner []Coord
	scanners := [][]Coord{}

	for line := range strings.SplitSeq(str, "\n") {
		if line == "" {
			continue
		}
		if strings.HasPrefix(line, "---") {
			if len(scanner) > 0 {
				scanners = append(scanners, scanner)
			}
			scanner = []Coord{}
		} else {
			split := strings.Split(line, ",")
			x, err := strconv.Atoi(split[0])
			if err != nil {
				log.Panic(err)
			}
			y, err := strconv.Atoi(split[1])
			if err != nil {
				log.Panic(err)
			}
			z, err := strconv.Atoi(split[2])
			if err != nil {
				log.Panic(err)
			}
			scanner = append(scanner, Coord{x, y, z, len(scanners)})
		}
	}

	if len(scanner) > 0 {
		scanners = append(scanners, scanner)
	}
	return scanners
}

func abs(n int) int {
	if n < 0 {
		return -n
	}
	return n
}

func getDistance(a Coord, b Coord) int {
	return abs(a.x-b.x) + abs(a.y-b.y) + abs(a.z-b.z)
	// xdiff := math.Pow(float64(a.x)-float64(b.x), 2)
	// ydiff := math.Pow(float64(a.y)-float64(b.y), 2)
	// zdiff := math.Pow(float64(a.z)-float64(b.z), 2)
	// return math.Sqrt(xdiff + ydiff + zdiff)
}

func getDistances(scanners [][]Coord) map[int][]Coord {
	dists := map[int][]Coord{}
	for _, scanner := range scanners {
		for o := 0; o < len(scanner)-1; o++ {
			for i := o + 1; i < len(scanner); i++ {
				dist := getDistance(scanner[o], scanner[i])
				dists[dist] = append(dists[dist], scanner[o])
				dists[dist] = append(dists[dist], scanner[i])
			}
		}
	}

	same := map[Coord]int{}
	for _, v := range dists {
		for _, c := range v {
			if c.s == 4 || c.s == 1 {
				same[c] = len(v)
			}
		}
	}

	count := 0
	for k, v := range same {
		if v > 2 {
			fmt.Println(k, v)
			if k.s == 4 {
				count += 1
			}
		}
	}
	fmt.Println(count)
	return dists
}

func Run() {
	fmt.Println("Day 19: Beacon Scanner")

	inputString := input.ReadDay("day19")
	scanners := parseInput(inputString)
	distances := getDistances(scanners)
	_ = distances

	fmt.Printf("Part One: TODO\n")
	fmt.Printf("Part Two: TODO\n")
}
