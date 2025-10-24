package day17

import (
	"testing"
)

var inputString string = `target area: x=20..30, y=-10..-5`

func TestPartOne(t *testing.T) {
	x1, x2, y1, y2 := parseInput(inputString)
	hightestY, _ := highestPosition(x1, x2, y1, y2)
	got := hightestY
	want := 45

	if got != want {
		t.Errorf("got %d, wanted %d", got, want)
	}
}

func TestPartTwo(t *testing.T) {
	x1, x2, y1, y2 := parseInput(inputString)
	_, hits := highestPosition(x1, x2, y1, y2)
	got := hits
	want := 112

	if got != want {
		t.Errorf("got %d, wanted %d", got, want)
	}
}
