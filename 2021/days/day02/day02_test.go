package day02

import (
	"testing"
)

var inputString string = `forward 5
down 5
forward 8
up 3
down 8
forward 2
`

var ins []pos = parseInput(inputString)

func TestPartOne(t *testing.T) {
	got := positionProduct(ins)
	want := 150

	if got != want {
		t.Errorf("got %d, wanted %d", got, want)
	}
}

func TestPartTwo(t *testing.T) {
	got := positionAimProduct(ins)
	want := 900

	if got != want {
		t.Errorf("got %d, wanted %d", got, want)
	}
}
