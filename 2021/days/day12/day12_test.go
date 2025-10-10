package day12

import (
	"testing"
)

var inputString string = `start-A
start-b
A-c
A-b
b-d
A-end
b-end`

func TestPartOne(t *testing.T) {
	caveMap := parseInput(inputString)
	_ = caveMap
	got := 1
	want := 2

	if got != want {
		t.Errorf("got %d, wanted %d", got, want)
	}
}

func TestPartTwo(t *testing.T) {
}
