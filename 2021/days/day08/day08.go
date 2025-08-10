package day08

import (
	"aoc_2021/input"
	"fmt"
	"slices"
	"sort"
	"strings"
)

type display struct {
	signals []string
	output  []string
}

func sortString(str string) string {
	split := strings.Split(str, "")
	sort.Strings(split)
	return strings.Join(split, "")
}

func parseInput(str string) []display {
	displays := []display{}
	for line := range strings.SplitSeq(str, "\n") {
		if line == "" {
			continue
		}
		split := strings.Split(line, " | ")

		signals := strings.Split(split[0], " ")
		for i := range signals {
			signals[i] = sortString(signals[i])
		}

		output := strings.Split(split[1], " ")
		for i := range output {
			output[i] = sortString(output[i])
		}

		displays = append(displays, display{signals, output})
	}
	return displays
}

func countEasyDigits(displays []display) int {
	uniqueLen := []int{2, 3, 4, 7}
	count := 0

	for _, disp := range displays {
		for _, digit := range disp.output {
			if slices.Contains(uniqueLen, len(digit)) {
				count += 1
			}
		}
	}

	return count
}

func identifyUniqueLenSignals(displ display) map[string]int {
	lenMap := map[int]int{2: 1, 3: 7, 4: 3, 7: 8}
	signalMap := map[string]int{}

	for _, signal := range displ.signals {
		val, ok := lenMap[len(signal)]
		if ok {
			signalMap[signal] = val
		}
	}

	return signalMap
}

func identifySignals(displ display) map[string]int {
	signalMap := identifyUniqueLenSignals(displ)

	// top segment: 7-1
	// bot right segment: in 9 out of 10 signals
	// // only digit 2 is missing bot right segment
	// top right segment: 1 - bot right segment

	// 0: 1, 7
	// 2: -
	// 3: 1, 7
	// 5: -
	// 6: 5
	// 9: 1, 3, 4, 5, 7

	return signalMap
}

func sumOutput(displays []display) int {
	sum := 0
	for _, displ := range displays {
		signalMap := identifySignals(displ)
		fmt.Println(signalMap)
		sum += 1
	}
	return sum
}

func Run() {
	fmt.Println("Day 8: Seven Segment Search")

	inputString := input.ReadDay("day08")
	displays := parseInput(inputString)

	fmt.Printf("Part One: %d\n", countEasyDigits(displays))
	fmt.Printf("Part Two: TODO\n")
}
