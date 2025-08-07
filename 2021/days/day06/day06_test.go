package day06

import (
	"testing"
)

var inputString string = "3,4,3,1,2"

func TestPartOne(t *testing.T) {
	fish := parseInput(inputString)
	got := lanternfishGrowth(fish, 80)
	want := 5934

	if got != want {
		t.Errorf("got %d, wanted %d", got, want)
	}
}

func TestPartTwo(t *testing.T) {
	fish := parseInput(inputString)
	got := lanternfishGrowth(fish, 256)
	want := 26984457539

	if got != want {
		t.Errorf("got %d, wanted %d", got, want)
	}
}
