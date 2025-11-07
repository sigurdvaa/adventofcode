package day20

import (
	"testing"
)

var inputString string = `# . . # .
#[. . .].
#[# . .]#
.[. # .].
. . # # #`

func TestPartOne(t *testing.T) {
	parseInput(inputString)
	got := 1
	want := 2

	if got != want {
		t.Errorf("got %d, wanted %d", got, want)
	}
}

func TestPartTwo(t *testing.T) {
}
