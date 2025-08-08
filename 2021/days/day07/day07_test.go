package day07

import (
	"testing"
)

var inputString string = "16,1,2,0,4,2,7,1,2,14"

func TestPartOne(t *testing.T) {
	got := 1
	want := 37

	if got != want {
		t.Errorf("got %d, wanted %d", got, want)
	}
}

func TestPartTwo(t *testing.T) {
}
