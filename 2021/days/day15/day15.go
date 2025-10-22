package day15

import (
	"aoc_2021/input"
	"container/heap"
	"fmt"
	"log"
	"strconv"
	"strings"
)

func parseInput(str string) [][]int {
	risks := [][]int{}

	for line := range strings.SplitSeq(str, "\n") {
		if line == "" {
			continue
		}
		row := []int{}
		for _, r := range line {
			num, err := strconv.Atoi(string(r))
			if err != nil {
				log.Fatal(err)
			}
			row = append(row, num)
		}
		risks = append(risks, row)
	}

	return risks
}

type State struct {
	risk int
	x    int
	y    int
}

type MinHeap []State

func (h MinHeap) Len() int           { return len(h) }
func (h MinHeap) Less(i, j int) bool { return h[i].risk < h[j].risk }
func (h MinHeap) Swap(i, j int)      { h[i], h[j] = h[j], h[i] }
func (h *MinHeap) Push(x any) {
	*h = append(*h, x.(State))
}
func (h *MinHeap) Pop() any {
	old := *h
	n := len(old)
	x := old[n-1]
	*h = old[0 : n-1]
	return x
}

func lowestTotalRisk(risks [][]int) int {
	queue := &MinHeap{{risk: 0, x: 0, y: 0}}
	visited := map[string]struct{}{}
	heap.Init(queue)
	for queue.Len() > 0 {
		state := heap.Pop(queue).(State)
		_, ok := visited[fmt.Sprintf("%d,%d", state.x, state.y)]
		if ok {
			continue
		}
		visited[fmt.Sprintf("%d,%d", state.x, state.y)] = struct{}{}

		if state.y == len(risks)-1 && state.x == len(risks[state.y])-1 {
			return state.risk
		}

		if state.y > 0 {
			heap.Push(queue, State{
				risk: state.risk + risks[state.y-1][state.x],
				x:    state.x,
				y:    state.y - 1,
			})
		}

		if state.y < len(risks)-1 {
			heap.Push(queue, State{
				risk: state.risk + risks[state.y+1][state.x],
				x:    state.x,
				y:    state.y + 1,
			})
		}

		if state.x > 0 {
			heap.Push(queue, State{
				risk: state.risk + risks[state.y][state.x-1],
				x:    state.x - 1,
				y:    state.y,
			})
		}

		if state.x < len(risks[state.y])-1 {
			heap.Push(queue, State{
				risk: state.risk + risks[state.y][state.x+1],
				x:    state.x + 1,
				y:    state.y,
			})
		}
	}

	return -1
}

func enlargeRiskMap(risks [][]int) [][]int {
	base := len(risks)
	size := base * 5
	large := make([][]int, size)

	for y := range size {
		row := make([]int, size)
		for x := range size {
			risk := risks[y%base][x%base] + (y / base) + (x / base)
			if risk > 9 {
				risk = (risk % 10) + 1
			}
			row[x] = risk
		}
		large[y] = row
	}

	return large
}

func Run() {
	fmt.Println("Day 15: Chiton")

	inputString := input.ReadDay("day15")
	risks := parseInput(inputString)
	largeRisks := enlargeRiskMap(risks)

	fmt.Printf("Part One: %d\n", lowestTotalRisk(risks))
	fmt.Printf("Part Two: %d\n", lowestTotalRisk(largeRisks))
}
