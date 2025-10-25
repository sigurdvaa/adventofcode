package day18

import (
	"aoc_2021/input"
	"fmt"
	"log"
	"math"
	"strconv"
	"strings"
)

type Pair struct {
	left   *Pair
	right  *Pair
	parent *Pair
	value  int
}

func (p *Pair) add(r *Pair) *Pair {
	parent := &Pair{left: p, right: r, value: -1}
	parent.left.parent = parent
	parent.right.parent = parent
	return parent
}

func (p *Pair) addValueToSide(prev *Pair, value int, dir string, side string) {
	switch dir {
	case "up":
		switch side {
		case "left":
			if p.left != nil && p.left != prev {
				p.left.addValueToSide(p, value, "down", side)
			} else if p.parent != nil {
				p.parent.addValueToSide(p, value, dir, side)
			}
		case "right":
			if p.right != nil && p.right != prev {
				p.right.addValueToSide(p, value, "down", side)
			} else if p.parent != nil {
				p.parent.addValueToSide(p, value, dir, side)
			}
		}
	case "down":
		if p.value > -1 {
			p.value += value
		} else {
			switch side {
			case "left":
				if p.right != nil {
					p.right.addValueToSide(p, value, dir, side)
				}
			case "right":
				if p.left != nil {
					p.left.addValueToSide(p, value, dir, side)
				}
			}
		}
	}
}

func (p *Pair) _reduceExplode(depth int) bool {
	if depth == 4 && p.value == -1 {
		p.parent.addValueToSide(p, p.left.value, "up", "left")
		p.parent.addValueToSide(p, p.right.value, "up", "right")

		p.left.parent = nil
		p.left = nil
		p.right.parent = nil
		p.right = nil

		p.value = 0
		return true
	}
	if p.left != nil && p.left._reduceExplode(depth+1) {
		return true
	}
	if p.right != nil && p.right._reduceExplode(depth+1) {
		return true
	}
	return false
}

func (p *Pair) _reduceSplit() bool {
	if p.value > 9 {
		p.left = &Pair{}
		p.left.parent = p
		p.left.value = p.value / 2

		p.right = &Pair{}
		p.right.parent = p
		p.right.value = int(math.Ceil(float64(p.value) / 2))

		p.value = -1
		return true
	}
	if p.left != nil && p.left._reduceSplit() {
		return true
	}
	if p.right != nil && p.right._reduceSplit() {
		return true
	}
	return false
}

func (p *Pair) reduce() {
	for {
		if p._reduceExplode(0) {
			continue
		}
		if p._reduceSplit() {
			continue
		}
		break
	}
}

func (p *Pair) magnitude() int {
	if p.value > -1 {
		return p.value
	}
	left := 3 * p.left.magnitude()
	right := 2 * p.right.magnitude()
	return left + right
}

func (p *Pair) clone() *Pair {
	clone := &Pair{}
	clone.value = p.value
	if p.left != nil {
		clone.left = p.left.clone()
		clone.left.parent = clone
	}
	if p.right != nil {
		clone.right = p.right.clone()
		clone.right.parent = clone
	}
	return clone
}

func parseNumber(number *[]rune) *Pair {
	pair := &Pair{}
	if (*number)[0] == '[' {
		*number = (*number)[1:]

		pair.left = parseNumber(number)
		if (*number)[0] != ',' {
			log.Fatal("missing comma: ", string(*number))
		}
		*number = (*number)[1:]
		pair.right = parseNumber(number)

		pair.left.parent = pair
		pair.right.parent = pair
		pair.value = -1
	} else {
		for i := range len(*number) {
			if (*number)[i] == ',' || (*number)[i] == ']' {
				num, err := strconv.Atoi(string((*number)[:i]))
				if err != nil {
					log.Fatal(err)
				}

				pair.value = num
				*number = (*number)[i:]
				break
			}
		}
	}

	if len(*number) > 0 && (*number)[0] == ']' {
		*number = (*number)[1:]
	}
	return pair
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

func sumNumbers(numbers []*Pair) *Pair {
	number := numbers[0].clone()
	for i := 1; i < len(numbers); i++ {
		number = number.add(numbers[i].clone())
		number.reduce()
	}
	return number
}

func largestMagnitudeTwoNumbers(numbers []*Pair) int {
	largest := 0
	for o := range len(numbers) - 1 {
		for i := o + 1; i < len(numbers); i++ {
			number := numbers[o].clone().add(numbers[i].clone())
			number.reduce()
			largest = max(largest, number.magnitude())
			number = numbers[i].clone().add(numbers[o].clone())
			number.reduce()
			largest = max(largest, number.magnitude())
		}
	}
	return largest
}

func Run() {
	fmt.Println("Day 18: Snailfish")

	inputString := input.ReadDay("day18")
	numbers := parseInput(inputString)
	number := sumNumbers(numbers)

	fmt.Printf("Part One: %d\n", number.magnitude())
	fmt.Printf("Part Two: %d\n", largestMagnitudeTwoNumbers(numbers))
}
