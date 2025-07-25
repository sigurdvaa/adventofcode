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

func powerConsumption(size int, report []int) int {
	commonBits := make([]int, size)
	gammaRate := 0
	epsilonRate := 0

	b := 1
	for i := 0; i < size; i++ {
		for _, num := range report {
			if num&b > 0 {
				commonBits[i] += 1
			} else {
				commonBits[i] -= 1
			}
		}
		b = b << 1
	}

	for i := len(commonBits) - 1; i >= 0; i-- {
		b := commonBits[i]
		gammaRate = gammaRate << 1
		epsilonRate = epsilonRate << 1
		if b == 0 {
			log.Fatalln("even bit found")
		}
		if b > 0 {
			gammaRate += 1
		} else {
			epsilonRate += 1
		}
	}

	return gammaRate * epsilonRate
}

func oxygenGeneratorRating(size int, report []int) int {
	b := 1 << (size - 1)
	for len(report) > 1 {
		curr := 0
		for _, num := range report {
			if b&num > 0 {
				curr += 1
			} else {
				curr -= 1
			}
		}

		var temp []int
		keep := b
		if curr < 0 {
			keep = 0
		}
		for _, num := range report {
			if b&num == keep {
				temp = append(temp, num)
			}
		}

		report = temp
		b = b >> 1
	}
	return report[0]
}

func co2ScrubberRating(size int, report []int) int {
	b := 1 << (size - 1)
	for len(report) > 1 {
		curr := 0
		for _, num := range report {
			if b&num > 0 {
				curr += 1
			} else {
				curr -= 1
			}
		}

		var temp []int
		keep := 0
		if curr < 0 {
			keep = b
		}
		for _, num := range report {
			if b&num == keep {
				temp = append(temp, num)
			}
		}

		report = temp
		b = b >> 1
	}
	return report[0]
}

func lifeSupportRating(size int, report []int) int {
	oxygen := oxygenGeneratorRating(size, report)
	co2 := co2ScrubberRating(size, report)
	return oxygen * co2
}

func Run() {
	fmt.Println("Day 3: Binary Diagnostic")

	inputString := input.ReadDay("day03")
	size, report := parseInput(inputString)

	fmt.Printf("Part One: %d\n", powerConsumption(size, report))
	fmt.Printf("Part Two: %d\n", lifeSupportRating(size, report))
}
