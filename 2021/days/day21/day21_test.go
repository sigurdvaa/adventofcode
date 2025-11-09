package day21

import (
	"testing"
)

var inputString string = `Player 1 starting position: 4
Player 2 starting position: 8`

func TestPartOne(t *testing.T) {
	p1, p2 := parseInput(inputString)
	_, _ = p1, p2
	got := 1
	want := 2

	if got != want {
		t.Errorf("got %d, wanted %d", got, want)
	}
}

func TestPartTwo(t *testing.T) {
}
