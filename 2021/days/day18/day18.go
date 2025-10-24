package day18

import (
	"aoc_2021/input"
	"fmt"
	"log"
	"strconv"
	"strings"
)

type Pair struct {
	left  *Pair
	right *Pair
	value int
}

func parseNumber(number *[]rune) *Pair {
	value := -1
	var left *Pair
	var right *Pair

	if (*number)[0] == '[' {
		*number = (*number)[1:]
		left = parseNumber(number)
		if (*number)[0] != ',' {
			log.Fatal("missing comma: ", string(*number))
		}
		*number = (*number)[1:]
		right = parseNumber(number)
	} else {
		for i := range len(*number) {
			if (*number)[i] == ',' || (*number)[i] == ']' {
				num, err := strconv.Atoi(string((*number)[:i]))
				if err != nil {
					log.Fatal(err)
				}
				value = num
				*number = (*number)[i:]
				break
			}
		}
	}

	if len(*number) > 0 && (*number)[0] == ']' {
		*number = (*number)[1:]
	}

	return &Pair{left, right, value}
}

func parseInput(str string) []*Pair {
	numbers := []*Pair{}
	for line := range strings.SplitSeq(str, "\n") {
		if line == "" {
			continue
		}
		number := []rune(line)
		numbers = append(numbers, parseNumber(&number))
	}
	return numbers
}

func Run() {
	fmt.Println("Day 18: Snailfish")

	inputString := input.ReadDay("day18")
	_ = parseInput(inputString)

	fmt.Printf("Part One: TODO\n")
	fmt.Printf("Part Two: TODO\n")
}
