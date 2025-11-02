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

type Offset struct {
	coord Coord
	xdir  int
	ydir  int
	zdir  int
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

func getMaxKey(count map[int]int) int {
	maxCount := 0
	maxKey := 0
	for k, v := range count {
		if v > maxCount {
			maxCount = v
			maxKey = k
		}
	}
	return maxKey
}

func getOffset(dists map[int]map[int][]Coord, overlap []int, a int, b int, aOffset *Offset) Offset {
	// xCands := map[int]int{}
	// yCands := map[int]int{}
	// zCands := map[int]int{}
	cands := map[Offset]int{}

	for _, dist := range overlap {
		for _, c1 := range dists[dist][a] {
			c1 = Coord{
				x: (c1.x * aOffset.xdir) + aOffset.coord.x,
				y: (c1.y * aOffset.ydir) + aOffset.coord.y,
				z: (c1.z * aOffset.zdir) + aOffset.coord.z,
			}
			for _, c2 := range dists[dist][b] {
				cands[Offset{
					coord: Coord{c1.x - c2.x, c1.y - c2.y, c1.z - c2.z},
					xdir:  1, ydir: 1, zdir: 1}] += 1
				cands[Offset{
					coord: Coord{c1.x + c2.x, c1.y - c2.y, c1.z - c2.z},
					xdir:  -1, ydir: 1, zdir: 1}] += 1
				cands[Offset{
					coord: Coord{c1.x + c2.x, c1.y + c2.y, c1.z - c2.z},
					xdir:  -1, ydir: -1, zdir: 1}] += 1
				cands[Offset{
					coord: Coord{c1.x + c2.x, c1.y + c2.y, c1.z + c2.z},
					xdir:  -1, ydir: -1, zdir: -1}] += 1
				cands[Offset{
					coord: Coord{c1.x - c2.x, c1.y + c2.y, c1.z + c2.z},
					xdir:  1, ydir: -1, zdir: -1}] += 1
				cands[Offset{
					coord: Coord{c1.x - c2.x, c1.y - c2.y, c1.z + c2.z},
					xdir:  1, ydir: 1, zdir: -1}] += 1
				cands[Offset{
					coord: Coord{c1.x + c2.x, c1.y - c2.y, c1.z + c2.z},
					xdir:  -1, ydir: 1, zdir: -1}] += 1
				cands[Offset{
					coord: Coord{c1.x - c2.x, c1.y + c2.y, c1.z - c2.z},
					xdir:  1, ydir: -1, zdir: 1}] += 1
			}
		}
	}
	maxv := 0
	var maxo Offset
	for k, v := range cands {
		if v > maxv {
			maxv = v
			maxo = k
		}
		if k.coord.x == 20 {
			fmt.Println(v, k)
		}
		if v >= 12 {
			return k
		}
	}
	fmt.Println("err", maxv, maxo, a, b)
	panic("no canidate found")
}

func assembleMap(scanners [][]Coord) []Coord {
	minEdges := 12 * 11 / 2
	distances := getDistances(scanners)
	offsets := make([]*Offset, len(scanners))
	beacons := make([]Coord, len(scanners[0]))
	copy(beacons, scanners[0])
	offsets[0] = &Offset{coord: Coord{0, 0, 0}, xdir: 1, ydir: 1, zdir: 1}

	scannerIdx := 0
	for {
		scannerIdx += 1
		if scannerIdx >= len(offsets) {
			scannerIdx = slices.Index(offsets, nil)
			if scannerIdx == -1 {
				fmt.Println(len(beacons))
				return beacons
			}
		} else if offsets[scannerIdx] != nil {
			continue
		}

		for o, otherOffset := range offsets {
			if otherOffset == nil {
				continue
			}

			// for scanner where offsets != nil, find overlapping distances
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
				scannerOffset := getOffset(distances, overlap, o, scannerIdx, offsets[o])
				fmt.Println(scannerIdx, scannerOffset)
				offsets[scannerIdx] = &scannerOffset
				// add all scanner i beacons
				for _, b := range scanners[scannerIdx] {
					b = Coord{
						x: (b.x * scannerOffset.xdir) + scannerOffset.coord.x,
						y: (b.y * scannerOffset.ydir) + scannerOffset.coord.y,
						z: (b.z * scannerOffset.zdir) + scannerOffset.coord.z,
					}
					if !slices.Contains(beacons, b) {
						beacons = append(beacons, b)
					} else {
						fmt.Println(scannerIdx, "skip")
					}
				}
				break
			}
		}
	}
}

func Run() {
	fmt.Println("Day 19: Beacon Scanner")

	inputString := input.ReadDay("day19")
	scanners := parseInput(inputString)
	beacons := assembleMap(scanners)

	fmt.Printf("Part One: %d\n", len(beacons))
	fmt.Printf("Part Two: TODO\n")
}
