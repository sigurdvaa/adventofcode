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

func identifySignalsByLen(displ display) (map[string]int, []string, []string) {
	uniqueLenMaps := map[int]int{2: 1, 3: 7, 4: 4, 7: 8}
	signalMap := map[string]int{}
	len5 := []string{}
	len6 := []string{}

	for _, signal := range displ.signals {
		sigLen := len(signal)
		val, ok := uniqueLenMaps[sigLen]
		if ok {
			signalMap[signal] = val
		} else {
			if sigLen == 6 {
				len6 = append(len6, signal)
			} else {
				len5 = append(len5, signal)
			}
		}
	}

	return signalMap, len5, len6
}

func removeSliceAt(slice []string, i int) []string {
	slice[i] = slice[len(slice)-1]
	return slice[:len(slice)-1]
}

func signalContainsSignal(a string, b string) bool {
	for _, r := range b {
		if !strings.ContainsRune(a, r) {
			return false
		}
	}
	return true
}

func identifySignals(displ display) map[string]int {
	signalMap, len5, len6 := identifySignalsByLen(displ)
	digitMap := map[int]string{}
	for k, v := range signalMap {
		digitMap[v] = k
	}

	// len 6:  0, 6, 9
	// // known 4-pattern in 9-pattern (remove 9 from 6-list)
	for i, sig := range len6 {
		if signalContainsSignal(sig, digitMap[4]) {
			signalMap[sig] = 9
			digitMap[9] = sig
			len6 = removeSliceAt(len6, i)
		}
	}

	// // known 1-pattern in 0-pattern (remove 0 from 6-list)
	for i, sig := range len6 {
		if signalContainsSignal(sig, digitMap[1]) {
			signalMap[sig] = 0
			digitMap[0] = sig
			len6 = removeSliceAt(len6, i)
		}
	}

	// // which leaves 6
	signalMap[len6[0]] = 6
	digitMap[6] = len6[0]

	// len 5:  2, 3, 5
	// // known 1-pattern in 3-pattern (remove 3 from 5-list)
	for i, sig := range len5 {
		if signalContainsSignal(sig, digitMap[1]) {
			signalMap[sig] = 3
			digitMap[3] = sig
			len5 = removeSliceAt(len5, i)
		}
	}

	// // unkown 5-pattern in known 6-pattern (remove 5 from 5-list)
	for i, sig := range len5 {
		if signalContainsSignal(digitMap[6], sig) {
			signalMap[sig] = 5
			digitMap[5] = sig
			len5 = removeSliceAt(len5, i)
		}
	}

	// // which leaves 2
	signalMap[len5[0]] = 2
	digitMap[2] = len5[0]

	return signalMap
}

func sumOutput(displays []display) int {
	sum := 0
	for _, displ := range displays {
		signalMap := identifySignals(displ)
		output := 0
		for _, sig := range displ.output {
			output *= 10
			output += signalMap[sig]
		}
		sum += output
	}
	return sum
}

func Run() {
	fmt.Println("Day 8: Seven Segment Search")

	inputString := input.ReadDay("day08")
	displays := parseInput(inputString)

	fmt.Printf("Part One: %d\n", countEasyDigits(displays))
	fmt.Printf("Part Two: %d\n", sumOutput(displays))
}
