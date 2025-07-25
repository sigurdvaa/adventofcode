package day03

import (
	"aoc_2021/input"
	"fmt"
	"log"
	"strings"
)

func parseInput(inputString string) (int, []int) {
	var report []int
	size := 0

	for _, line := range strings.Split(inputString, "\n") {
		if line == "" {
			continue
		}

		b := 0
		for i, c := range strings.Split(line, "") {
			b = b << 1
			switch c {
			case "0":
			case "1":
				b += 1
			default:
				log.Fatalf("invalid char for bit representation: %s", c)
			}
			if i > size {
				size = i
			}
		}
		report = append(report, b)
	}

	return size + 1, report
}

func power_consumption(size int, report []int) int {
	common_bits := make([]int, size)
	gamma_rate := 0
	epsilon_rate := 0

	b := 1
	for i := 0; i < size; i++ {
		for _, num := range report {
			if num&b > 0 {
				common_bits[i] += 1
			} else {
				common_bits[i] -= 1
			}
		}
		b = b << 1
	}

	for i := len(common_bits) - 1; i >= 0; i-- {
		b := common_bits[i]
		gamma_rate = gamma_rate << 1
		epsilon_rate = epsilon_rate << 1
		if b == 0 {
			log.Fatalln("even bit found")
		}
		if b > 0 {
			gamma_rate += 1
		} else {
			epsilon_rate += 1
		}
	}

	return gamma_rate * epsilon_rate
}

func Run() {
	fmt.Println("Day 3: Binary Diagnostic")

	inputString := input.ReadDay("day03")
	size, report := parseInput(inputString)

	fmt.Printf("Part One: %d\n", power_consumption(size, report))
	fmt.Printf("Part Two: TODO\n")
}
