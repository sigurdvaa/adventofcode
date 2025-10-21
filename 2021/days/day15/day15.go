package day15

import (
	"aoc_2021/input"
	"container/heap"
	"fmt"
	"log"
	"maps"
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
	risk    int
	x       int
	y       int
	visited map[string]struct{}
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
	queue := &MinHeap{{risk: 0, x: 0, y: 0, visited: map[string]struct{}{}}}
	heap.Init(queue)
	for queue.Len() > 0 {
		state := heap.Pop(queue).(State)
		_, ok := state.visited[fmt.Sprintf("%d,%d", state.x, state.y)]
		if ok {
			continue
		}
		state.visited[fmt.Sprintf("%d,%d", state.x, state.y)] = struct{}{}

		if state.y == len(risks)-1 && state.x == len(risks[state.y])-1 {
			return state.risk
		}

		if state.y > 0 {
			visited := map[string]struct{}{}
			maps.Copy(visited, state.visited)
			heap.Push(queue, State{
				risk:    state.risk + risks[state.y-1][state.x],
				x:       state.x,
				y:       state.y - 1,
				visited: visited,
			})
		}

		if state.y < len(risks)-1 {
			visited := map[string]struct{}{}
			maps.Copy(visited, state.visited)
			heap.Push(queue, State{
				risk:    state.risk + risks[state.y+1][state.x],
				x:       state.x,
				y:       state.y + 1,
				visited: visited,
			})
		}

		if state.x > 0 {
			visited := map[string]struct{}{}
			maps.Copy(visited, state.visited)
			heap.Push(queue, State{
				risk:    state.risk + risks[state.y][state.x-1],
				x:       state.x - 1,
				y:       state.y,
				visited: visited,
			})
		}

		if state.x < len(risks[state.y])-1 {
			visited := map[string]struct{}{}
			maps.Copy(visited, state.visited)
			heap.Push(queue, State{
				risk:    state.risk + risks[state.y][state.x+1],
				x:       state.x + 1,
				y:       state.y,
				visited: visited,
			})
		}
	}

	return -1
}

func Run() {
	fmt.Println("Day 15: Chiton")

	inputString := input.ReadDay("day15")
	risks := parseInput(inputString)

	fmt.Printf("Part One: %d\n", lowestTotalRisk(risks))
	fmt.Printf("Part Two: TODO\n")
}
