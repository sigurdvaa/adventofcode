package day23

import (
	"testing"
)

var inputString string = `#############
#...........#
###B#C#B#D###
  #A#D#C#A#
  #########`

func TestPartOne(t *testing.T) {
	_ = parseInput(inputString)
	got := 1
	want := 2

	if got != want {
		t.Errorf("got %d, wanted %d", got, want)
	}
}

func TestPartTwo(t *testing.T) {
}
