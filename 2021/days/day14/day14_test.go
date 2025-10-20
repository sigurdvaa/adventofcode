package day14

import (
	"testing"
)

var inputString string = `NNCB

CH -> B
HH -> N
CB -> H
NH -> C
HB -> C
HC -> B
HN -> C
NN -> C
BH -> H
NC -> B
NB -> B
BN -> B
BB -> N
BC -> B
CC -> N
CN -> C`

func TestPartOne(t *testing.T) {
	template, rules := parseInput(inputString)
	got := diffMostLeastCommonElements(template, rules, 10)
	want := 1588

	if got != want {
		t.Errorf("got %d, wanted %d", got, want)
	}
}

func TestPartTwo(t *testing.T) {
	template, rules := parseInput(inputString)
	got := diffMostLeastCommonElements(template, rules, 40)
	want := 2188189693529

	if got != want {
		t.Errorf("got %d, wanted %d", got, want)
	}
}
