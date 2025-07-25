package day03

import (
	"testing"
)

var inputString string = `00100
11110
10110
10111
10101
01111
00111
11100
10000
11001
00010
01010
`

func TestPartOne(t *testing.T) {
	size, report := parseInput(inputString)

	got := powerConsumption(size, report)
	want := 198

	if got != want {
		t.Errorf("got %d, wanted %d", got, want)
	}
}

func TestPartTwo(t *testing.T) {
	size, report := parseInput(inputString)

	got := lifeSupportRating(size, report)
	want := 230

	if got != want {
		t.Errorf("got %d, wanted %d", got, want)
	}
}
