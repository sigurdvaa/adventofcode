package day01

import (
	"testing"
)

var inputString string = `199
200
208
210
200
207
240
269
260
263
`
var depths []int = parseInput(inputString)

func TestPartOne(t *testing.T) {

	got := depthMeasurementIncreases(depths, 1)
	want := 7

	if got != want {
		t.Errorf("got %d, wanted %d", got, want)
	}
}

func TestPartTwo(t *testing.T) {

	got := depthMeasurementIncreases(depths, 3)
	want := 5

	if got != want {
		t.Errorf("got %d, wanted %d", got, want)
	}
}
