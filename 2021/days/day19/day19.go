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

const MIN_EDGES int = 12 * 11 / 2

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
	maxV := 0
	maxK := 0
	for k, v := range cands {
		if v > maxV {
			maxV = v
			maxK = k
		}
	}
	return maxK, maxV >= MIN_EDGES
}

func rot90(c *Coord, ax [2]int) {
	c[ax[0]], c[ax[1]] = c[ax[1]], -c[ax[0]]
}

var (
	ROLL = [2]int{1, 2}
	CW   = [2]int{0, 1}
	CCW  = [2]int{1, 0}
	ROTS = [24][2]int{
		ROLL, CW, CW, CW,
		ROLL, CCW, CCW, CCW,
		ROLL, CW, CW, CW,
		ROLL, CCW, CCW, CCW,
		ROLL, CW, CW, CW,
		ROLL, CCW, CCW, CCW,
	}
)

func getRotationAndOffset(overlap [][]Coord) ([][2]int, Coord) {
	rots := [][2]int{}
	for _, rot := range ROTS {
		rots = append(rots, rot)
		xCands := map[int]int{}
		yCands := map[int]int{}
		zCands := map[int]int{}
		for o := range overlap {
			rot90(&overlap[o][1], rot)
			xdiff := overlap[o][0][0] - overlap[o][1][0]
			xCands[xdiff] += 1
			ydiff := overlap[o][0][1] - overlap[o][1][1]
			yCands[ydiff] += 1
			zdiff := overlap[o][0][2] - overlap[o][1][2]
			zCands[zdiff] += 1
		}
		if xmatch, xok := matchFound(xCands); xok {
			if ymatch, yok := matchFound(yCands); yok {
				if zmatch, zok := matchFound(zCands); zok {
					return rots, Coord{xmatch, ymatch, zmatch}
				}
			}
		}
	}
	panic("unreachable")
}

func assembleMap(scanners [][]Coord) ([]Coord, []*Coord) {
	assembled := make([]*Coord, len(scanners))
	assembled[0] = &Coord{0, 0, 0}
	for slices.Contains(assembled, nil) {
		for curr := range scanners {
			if assembled[curr] != nil {
				continue
			}
			for other := range scanners {
				if assembled[other] == nil {
					continue
				}
				overlap := getOverlap(scanners[other], scanners[curr])
				if len(overlap) >= MIN_EDGES {
					rots, offset := getRotationAndOffset(overlap)
					for c := range scanners[curr] {
						for _, rot := range rots {
							rot90(&scanners[curr][c], rot)
						}
						coord := scanners[curr][c]
						scanners[curr][c] = Coord{
							coord[0] + offset[0],
							coord[1] + offset[1],
							coord[2] + offset[2],
						}
					}
					assembled[curr] = &offset
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
	return beacons, assembled
}

func abs(n int) int {
	if n < 0 {
		return -n
	}
	return n
}

func largestManhattanDist(offsets []*Coord) int {
	largest := 0
	for o := range len(offsets) - 1 {
		for i := o + 1; i < len(offsets); i++ {
			dist := abs(offsets[o][0]-offsets[i][0]) +
				abs(offsets[o][1]-offsets[i][1]) +
				abs(offsets[o][2]-offsets[i][2])
			if dist > largest {
				largest = dist
			}
		}
	}
	return largest
}

func Run() {
	fmt.Println("Day 19: Beacon Scanner")

	inputString := input.ReadDay("day19")
	scanners := parseInput(inputString)
	beacons, offsets := assembleMap(scanners)

	fmt.Printf("Part One: %d\n", len(beacons))
	fmt.Printf("Part Two: %d\n", largestManhattanDist(offsets))
}
