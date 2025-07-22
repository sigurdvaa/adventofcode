package days

import (
	"aoc_2021/days/day01"
)

var DAYS = [...]func(){day01.Run}

func Run(num int) {
	if num == 0 {
		for _, f := range DAYS {
			f()
		}
	} else {
		DAYS[num-1]()
	}
}
