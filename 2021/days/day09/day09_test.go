package day09

import (
	"testing"
)

var inputString string = `2199943210
3987894921
9856789892
8767896789
9899965678`

func TestPartOne(t *testing.T) {
	heightMap := parseInput(inputString)

	got := sumRiskLevelLowPoints(heightMap)
	want := 15

	if got != want {
		t.Errorf("got %d, wanted %d", got, want)
	}
}

func TestPartTwo(t *testing.T) {
	heightMap := parseInput(inputString)

	got := productLargestBasins(heightMap)
	want := 1134

	if got != want {
		t.Errorf("got %d, wanted %d", got, want)
	}
}
