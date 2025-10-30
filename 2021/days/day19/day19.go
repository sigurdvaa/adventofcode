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

func getOffset(a []Coord, b []Coord) Coord {
	candidates := []Coord{}

	for _, c1 := range a {
		for _, c2 := range b {
			candidates = append(candidates, Coord{
				x: c1.x - c2.x,
				y: c1.y - c2.y,
				z: c1.z - c2.z,
			})
		}
	}

	return Coord{0, 0, 0}
}

func assembleMap(scanners [][]Coord) []Coord {
	minEdges := 12 * 11 / 2
	distances := getDistances(scanners)
	offsets := make([]*Coord, len(scanners))
	beacons := make([]Coord, len(scanners[0]))
	copy(beacons, scanners[0])
	offsets[0] = &Coord{0, 0, 0}

	scannerIdx := slices.Index(offsets, nil)
	for scannerIdx > -1 {
		for o, offset := range offsets {
			if offset == nil {
				continue
			}

			// for scanner i where offsets != nil, find overlapping distances
			overlap := []int{}
			for dist, scanners := range distances {
				if _, ok := scanners[scannerIdx]; ok {
					if _, ok := scanners[o]; ok {
						overlap = append(overlap, dist)
					}
				}
			}
			// if over minEdge threshold, find offset and add beacons
			if len(overlap) >= minEdges {
				offset := getOffset(distances[overlap[0]][scannerIdx], distances[overlap[0]][o])

				// add all scanner i beacons
			}

			// TODO REMOVE
			return beacons
		}

		scannerIdx = slices.Index(offsets, nil)
	}

	return beacons
}

func Run() {
	fmt.Println("Day 19: Beacon Scanner")

	inputString := input.ReadDay("day19")
	scanners := parseInput(inputString)
	beacons := assembleMap(scanners)

	fmt.Printf("Part One: %d\n", len(beacons))
	fmt.Printf("Part Two: TODO\n")
}
