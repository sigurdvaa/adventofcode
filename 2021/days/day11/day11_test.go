package day11

import (
	"testing"
)

var inputString string = `5483143223
2745854711
5264556173
6141336146
6357385478
4167524645
2176841721
6882881134
4846848554
5283751526`

func TestPartOne(t *testing.T) {
	grid := parseInput(inputString)
	got := gameOfOctopus(grid, 100)
	want := 1656

	if got != want {
		t.Errorf("got %d, wanted %d", got, want)
	}
}

func TestPartTwo(t *testing.T) {
	grid := parseInput(inputString)
	got := gameOfOctopusFirstSync(grid)
	want := 195

	if got != want {
		t.Errorf("got %d, wanted %d", got, want)
	}
}
