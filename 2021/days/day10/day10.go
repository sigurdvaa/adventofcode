package day10

import (
	"aoc_2021/input"
	"fmt"
	"slices"
	"strings"
)

func parseInput(str string) [][]rune {
	nav := [][]rune{}
	for line := range strings.SplitSeq(str, "\n") {
		if line == "" {
			continue
		}
		nav = append(nav, []rune(line))
	}
	return nav
}

func lineError(line []rune) (int, []rune) {
	open := []rune{'(', '[', '{', '<'}
	stack := []rune{}

	for _, r := range line {
		if slices.Contains(open, r) {
			stack = append(stack, r)
			continue
		}

		prev := stack[len(stack)-1]
		stack = stack[:len(stack)-1]

		switch r {
		case ')':
			if prev != '(' {
				return 3, stack
			}
		case ']':
			if prev != '[' {
				return 57, stack
			}
		case '}':
			if prev != '{' {
				return 1197, stack
			}
		case '>':
			if prev != '<' {
				return 25137, stack
			}
		}
	}

	return 0, stack
}

func syntaxErrorScore(lines [][]rune) int {
	score := 0

	for _, line := range lines {
		err, _ := lineError(line)
		score += err
	}

	return score
}

func middleAutocompletionScore(lines [][]rune) int {
	scores := []int{}
	valueMap := map[rune]int{'(': 1, '[': 2, '{': 3, '<': 4}

	for _, line := range lines {
		err, stack := lineError(line)
		if err == 0 {
			score := 0
			for i := len(stack) - 1; i >= 0; i-- {
				score *= 5
				score += valueMap[stack[i]]
			}
			scores = append(scores, score)
		}
	}

	slices.Sort(scores)
	return scores[len(scores)/2]
}

func Run() {
	fmt.Println("Day 10: Syntax Scoring")

	inputString := input.ReadDay("day10")
	lines := parseInput(inputString)

	fmt.Printf("Part One: %d\n", syntaxErrorScore(lines))
	fmt.Printf("Part Two: %d\n", middleAutocompletionScore(lines))
}
