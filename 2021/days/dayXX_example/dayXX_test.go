//go:build exclude

// remove above comment and change "dayXX"
package dayXX

import (
	"testing"
)

func TestPartOne(t *testing.T) {
	got := 1
	want := 2

	if got != want {
		t.Errorf("got %d, wanted %d", got, want)
	}
}

func TestPartTwo(t *testing.T) {
}
