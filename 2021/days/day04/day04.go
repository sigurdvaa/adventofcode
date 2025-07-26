package day04

import (
	"aoc_2021/input"
	"fmt"
	"log"
	"strconv"
	"strings"
)

const ROW = 5
const COL = 5

type Board [ROW * COL]int
type Status [ROW * COL]bool

func parseInput(inputString string) ([]int, []Board) {
	var numbers []int
	var boards []Board
	var lines = strings.Split(inputString, "\n")

	for _, n := range strings.Split(lines[0], ",") {
		num, err := strconv.Atoi(n)
		if err != nil {
			log.Fatalln(err)
		}
		numbers = append(numbers, num)
	}

	board := Board{}
	i := 0
	for _, line := range lines[2:] {
		if line == "" {
			continue
		}

		for _, n := range strings.Split(line, " ") {
			if n == "" {
				continue
			}
			num, err := strconv.Atoi(n)
			if err != nil {
				log.Fatalln(err)
			}
			board[i] = num
			i++
		}

		if i == ROW*COL {
			boards = append(boards, board)
			board = Board{}
			i = 0
		}
	}

	if i != 0 {
		log.Fatalln("invalid board parsed")
	}

	return numbers, boards
}

func checkBingo(status Status, i int) bool {
	bingo := true
	rowStart := (i / ROW) * ROW
	for s := 0; s < 5; s++ {
		if !status[rowStart+s] {
			bingo = false
			break
		}
	}
	if bingo {
		return true
	}

	colStart := i % COL
	for s := 0; s < ROW*COL; s += COL {
		if !status[colStart+s] {
			return false
		}
	}
	return true
}

func sumUnmarked(board Board, status Status) int {
	sum := 0
	for i, num := range board {
		if !status[i] {
			sum += num
		}
	}
	return sum
}

func bingoScore(numbers []int, boards []Board) int {
	status := make([]Status, len(boards))
	for _, draw := range numbers {
		for b, board := range boards {
			for n, num := range board {
				if num == draw {
					status[b][n] = true
					if checkBingo(status[b], n) {
						return draw * sumUnmarked(board, status[b])
					}
					continue
				}
			}
		}
	}
	panic("no solution")
}

func Run() {
	fmt.Println("Day 4: Giant Squid")

	inputString := input.ReadDay("day04")
	numbers, boards := parseInput(inputString)

	fmt.Printf("Part One: %d\n", bingoScore(numbers, boards))
	fmt.Printf("Part Two: TODO\n")
}
