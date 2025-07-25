package days

import (
	"aoc_2021/days/day01"
	"aoc_2021/days/day02"
	"aoc_2021/days/day03"
	"aoc_2021/days/day04"
	"aoc_2021/days/day05"
	"aoc_2021/days/day06"
	"aoc_2021/days/day07"
	"aoc_2021/days/day08"
	"aoc_2021/days/day09"
	"aoc_2021/days/day10"
	"aoc_2021/days/day11"
	"aoc_2021/days/day12"
	"aoc_2021/days/day13"
	"aoc_2021/days/day14"
	"aoc_2021/days/day15"
	"aoc_2021/days/day16"
	"aoc_2021/days/day17"
	"aoc_2021/days/day18"
	"aoc_2021/days/day19"
	"aoc_2021/days/day20"
	"aoc_2021/days/day21"
	"aoc_2021/days/day22"
	"aoc_2021/days/day23"
	"aoc_2021/days/day24"
	"aoc_2021/days/day25"
	"fmt"
)

var DAYS = [...]func(){
	day01.Run,
	day02.Run,
	day03.Run,
	day04.Run,
	day05.Run,
	day06.Run,
	day07.Run,
	day08.Run,
	day09.Run,
	day10.Run,
	day11.Run,
	day12.Run,
	day13.Run,
	day14.Run,
	day15.Run,
	day16.Run,
	day17.Run,
	day18.Run,
	day19.Run,
	day20.Run,
	day21.Run,
	day22.Run,
	day23.Run,
	day24.Run,
	day25.Run,
}

func Run(num int) {
	if num == 0 {
		for _, f := range DAYS {
			f()
			fmt.Println()
		}
	} else {
		DAYS[num-1]()
	}
}
