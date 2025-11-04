package day19

import (
	"aoc_2021/input"
	"fmt"
	"log"
	"slices"
	"strconv"
	"strings"
)

type Coord [3]int

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
	return pow(a[0]-b[0], 2) + pow(a[1]-b[1], 2) + pow(a[2]-b[2], 2)
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

func matchFound(cands map[int]int) (int, bool) {
	minEdges := 12 * 11 / 2
	maxV := 0
	maxK := 0
	for k, v := range cands {
		if v > maxV {
			maxV = v
			maxK = k
		}
	}
	return maxK, maxV >= minEdges*2
}

func getRotationAndOffset(overlap [][]Coord) (Coord, Coord) {
	rot := Coord{}
	for x := range 3 {
		for y := range 3 {
			for z := range 3 {
				if x == y || y == z || x == z {
					continue
				}
				xCands := map[int]int{}
				yCands := map[int]int{}
				zCands := map[int]int{}
				rot = Coord{x, y, z}
				for _, o := range overlap {
					diff1 := o[0][0] + o[1][x]
					diff2 := o[0][0] - o[1][x]
					xCands[diff1] += 1
					xCands[diff2] += 1

					diff1 = o[0][1] + o[1][y]
					diff2 = o[0][1] - o[1][y]
					yCands[diff1] += 1
					yCands[diff2] += 1

					diff1 = o[0][2] + o[1][z]
					diff2 = o[0][2] - o[1][z]
					zCands[diff1] += 1
					zCands[diff2] += 1
				}
				xOff, xOk := matchFound(xCands)
				yOff, yOk := matchFound(yCands)
				zOff, zOk := matchFound(zCands)
				if xOk && yOk && zOk {
					return rot, Coord{xOff, yOff, zOff}
				}
			}
		}
	}
	panic("unreachable")
}

func assembleMap(scanners [][]Coord) []Coord {
	assembled := make([]bool, len(scanners))
	assembled[0] = true
	minEdges := 12 * 11 / 2
	// distances := getDistances(scanners)
	for slices.Contains(assembled, false) {
		for curr := range scanners {
			if assembled[curr] {
				continue
			}
			for other := range scanners {
				if !assembled[other] {
					continue
				}
				overlap := getOverlap(scanners[other], scanners[curr])
				if len(overlap) >= minEdges*4 {
					rot, offset := getRotationAndOffset(overlap)
					fmt.Println(other, curr, rot, offset)
					for c := range scanners[curr] {
						coord := scanners[curr][c]
						scanners[curr][c] = Coord{
							coord[rot[0]] + offset[0],
							coord[rot[1]] + offset[1],
							coord[rot[2]] + offset[2],
						}
						if curr == 4 {
							fmt.Println(scanners[curr][c])
						}
					}
					assembled[curr] = true
					break
				}
			}
		}
	}

	beacons := []Coord{}
	for _, scanner := range scanners {
		for _, coord := range scanner {
			if !slices.Contains(beacons, coord) {
				beacons = append(beacons, coord)
			}
		}
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
