package days

import (
	"aoc_2021/days/day01"
	"aoc_2021/days/day02"
	"fmt"
)

var DAYS = [...]func(){day01.Run, day02.Run}

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
