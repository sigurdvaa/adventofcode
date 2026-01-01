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
	rooms := parseInput(inputString)
	got := organizeByLeastEnergy(rooms)
	want := 12521

	if got != want {
		t.Errorf("got %d, wanted %d", got, want)
	}
}

func TestPartTwo(t *testing.T) {
}
