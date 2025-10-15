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
	got := numPathsVisitSmallOnce(caveMap)
	want := 10

	if got != want {
		t.Errorf("got %d, wanted %d", got, want)
	}
}

func TestPartTwo(t *testing.T) {
	caveMap := parseInput(inputString)
	got := numPathsVisitSingleSmallTwice(caveMap)
	want := 36

	if got != want {
		t.Errorf("got %d, wanted %d", got, want)
	}
}
