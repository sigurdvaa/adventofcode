package day16

import (
	"testing"
)

func TestPartOne(t *testing.T) {
	tests := []struct {
		input string
		want  int
	}{
		{"D2FE28", 6},
		{"8A004A801A8002F478", 16},
		{"620080001611562C8802118E34", 12},
		{"C0015000016115A2E0802F182340", 23},
		{"A0016C880162017C3686B18A3D4780", 31},
	}

	for _, test := range tests {
		packet := parseInput(test.input)
		got := sumPacketVersion(packet)
		want := test.want

		if got != want {
			t.Errorf("got %d, wanted %d", got, want)
		}
	}
}

func TestPartTwo(t *testing.T) {
}
