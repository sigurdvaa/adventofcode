package day13

import (
	"testing"
)

var inputString string = `6,10
0,14
9,10
0,3
10,4
4,11
6,0
6,12
4,1
0,13
10,12
3,4
3,0
8,4
1,10
2,14
8,10
9,0

fold along y=7
fold along x=5`

func TestPartOne(t *testing.T) {
	dots, folds := parseInput(inputString)
	got := len(dotsAfterFold(dots, folds[:1]))
	want := 17

	if got != want {
		t.Errorf("got %d, wanted %d", got, want)
	}
}

func TestPartTwo(t *testing.T) {
}
