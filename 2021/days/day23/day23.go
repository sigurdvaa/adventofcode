package day23

import (
	"aoc_2021/input"
	"fmt"
	"strings"
)

const NUM_ROOMS int = 4
const ROOM_SIZE int = 2

type Rooms [NUM_ROOMS][ROOM_SIZE]Amphipod

type Amphipod rune

const (
	AmphipodAmber  Amphipod = 'A'
	AmphipodBronze Amphipod = 'B'
	AmphipodCopper Amphipod = 'C'
	AmphipodDesert Amphipod = 'D'
)

func (a Amphipod) energy() int {
	switch a {
	case AmphipodAmber:
		return 1
	case AmphipodBronze:
		return 10
	case AmphipodCopper:
		return 100
	case AmphipodDesert:
		return 1000
	default:
		panic(fmt.Sprintf("unknown energy for Amphipod: %c", a))
	}
}

func (a Amphipod) String() string {
	return string(a)
}

func ParseAmphipod(r rune) (a Amphipod, err error) {
	parse := map[Amphipod]struct{}{
		AmphipodAmber:  {},
		AmphipodBronze: {},
		AmphipodCopper: {},
		AmphipodDesert: {},
	}
	result := Amphipod(r)
	_, ok := parse[result]
	if !ok {
		return a, fmt.Errorf("cannot parse '%c' as Amphipod", r)
	}
	return result, nil
}

func parseInput(str string) Rooms {
	rooms := Rooms{}
	c := 0
	for line := range strings.SplitSeq(str, "\n") {
		if line == "" {
			continue
		}
		for _, a := range line {
			amphipod, err := ParseAmphipod(a)
			if err == nil {
				rooms[c%NUM_ROOMS][c/NUM_ROOMS] = amphipod
				c += 1
			}
		}
	}
	return rooms
}

type State struct {
	hallway [11]Amphipod
	rooms   Rooms
	energy  int
}

func organizeByLeastEnergy(initial Rooms) int {
	queue := []State{{[11]Amphipod{}, initial, 0}}
	seen := map[State]bool{queue[0]: true}
	for len(queue) != 0 {
		curr := queue[0]
		queue = queue[1:]
		if _, ok := seen[curr]; !ok {
			continue
		}
		// TODO branch to new states
		// TODO either use a minheap or run one iteration of bubble sort to bring min energy to top
	}
	return 0
}

func Run() {
	fmt.Println("Day 23: Amphipod")

	inputString := input.ReadDay("day23")
	rooms := parseInput(inputString)

	fmt.Printf("Part One: %d\n", organizeByLeastEnergy(rooms))
	fmt.Printf("Part Two: TODO\n")
}
