package day21

import (
	"aoc_2021/input"
	"fmt"
	"strconv"
	"strings"
)

func parseInput(str string) (int, int) {
	starts := []int{}
	for line := range strings.SplitSeq(str, "\n") {
		if line == "" {
			continue
		}
		split := strings.Split(line, ": ")
		num, err := strconv.Atoi(split[1])
		if err != nil {
			panic(err)
		}
		starts = append(starts, num)
	}
	return starts[0], starts[1]
}

type Dice struct {
	face  int
	total int
}

func (d *Dice) roll() int {
	d.face += 1
	d.total += 1
	if d.face > 100 {
		d.face = 1
	}
	return d.face
}

func (d *Dice) turn() int {
	sum := d.roll()
	sum += d.roll()
	sum += d.roll()
	return sum
}

func playDiracDice(p1, p2 int) int {
	dice := Dice{}
	p1score, p2score := 0, 0

	for {
		p1 += dice.turn()
		p1 %= 10
		if p1 == 0 {
			p1 = 10
		}
		p1score += p1
		if p1score >= 1000 {
			break
		}

		p2 += dice.turn()
		p2 %= 10
		if p2 == 0 {
			p2 = 10
		}
		p2score += p2
		if p2score >= 1000 {
			break
		}
	}
	fmt.Println(p1, p1score, p2, p2score, dice.total)
	return min(p1score, p2score) * dice.total
}

func Run() {
	fmt.Println("Day 21: Dirac Dice")

	inputString := input.ReadDay("day21")
	p1, p2 := parseInput(inputString)
	_, _ = p1, p2

	fmt.Printf("Part One: %d\n", playDiracDice(p1, p2))
	fmt.Printf("Part Two: TODO\n")
}
