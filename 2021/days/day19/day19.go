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
			scanner = append(scanner, Coord{x, y, z})
		}
	}

	if len(scanner) > 0 {
		scanners = append(scanners, scanner)
	}
	return scanners
}

func pow(x, n int) int {
	for range n - 1 {
		x *= x
	}
	return x
}

func getDistance(a Coord, b Coord) int {
	return pow(a.x-b.x, 2) + pow(a.y-b.y, 2) + pow(a.z-b.z, 2)
}

func getDistances(scanners [][]Coord) map[int]map[int][]Coord {
	dists := map[int]map[int][]Coord{}
	for s, scanner := range scanners {
		for o := 0; o < len(scanner)-1; o++ {
			for i := o + 1; i < len(scanner); i++ {
				dist := getDistance(scanner[o], scanner[i])
				if _, ok := dists[dist]; !ok {
					dists[dist] = map[int][]Coord{}
				}
				dists[dist][s] = append(dists[dist][s], scanner[o])
				dists[dist][s] = append(dists[dist][s], scanner[i])
			}
		}
	}
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
