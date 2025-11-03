package day19

import (
	"aoc_2021/input"
	"fmt"
	"log"
	"slices"
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

func getDistances(scanner []Coord) [][]int {
	dists := make([][]int, len(scanner))
	for o := 0; o < len(scanner)-1; o++ {
		for i := o + 1; i < len(scanner); i++ {
			dist := getDistance(scanner[o], scanner[i])
			dists[o] = append(dists[o], dist)
			dists[i] = append(dists[i], dist)
		}
	}
	return dists
}

func getOverlap(aCoords []Coord, bCoords []Coord) [][]Coord {
	overlap := [][]Coord{}
	aDists := getDistances(aCoords)
	bDists := getDistances(bCoords)
	for a, adists := range aDists {
		for b, bdists := range bDists {
			for _, dist := range adists {
				if slices.Contains(bdists, dist) {
					overlap = append(overlap, []Coord{aCoords[a], bCoords[b]})
				}
			}
		}
	}
	return overlap
}

func assembleMap(scanners [][]Coord) []Coord {
	assembled := make([]bool, len(scanners))
	minEdges := 12 * 11 / 2
	// distances := getDistances(scanners)

	for slices.Contains(assembled, false) {
		for curr := range scanners {
			if assembled[curr] {
				continue
			}
			for other := range scanners {
				if !assembled[curr] {
					continue
				}
				overlap := getOverlap(scanners[other], scanners[curr])
				if len(overlap) >= minEdges {
					// get offset and rotation?
				}
			}
		}
	}

	return []Coord{}
}

func Run() {
	fmt.Println("Day 19: Beacon Scanner")

	inputString := input.ReadDay("day19")
	scanners := parseInput(inputString)
	beacons := assembleMap(scanners)

	fmt.Printf("Part One: %d\n", len(beacons))
	fmt.Printf("Part Two: TODO\n")
}
