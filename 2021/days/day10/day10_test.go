package day10

import (
	"testing"
)

var inputString string = `[({(<(())[]>[[{[]{<()<>>
[(()[<>])]({[<{<<[]>>(
{([(<{}[<>[]}>{[]{[(<()>
(((({<>}<{<{<>}{[]{[]{}
[[<[([]))<([[{}[[()]]]
[{[{({}]{}}([{[{{{}}([]
{<[[]]>}<{[{[{[]{()[[[]
[<(<(<(<{}))><([]([]()
<{([([[(<>()){}]>(<<{{
<{([{{}}[<[[[<>{}]]]>[]]`

func TestPartOne(t *testing.T) {
	lines := parseInput(inputString)

	got := syntaxErrorScore(lines)
	want := 26397

	if got != want {
		t.Errorf("got %d, wanted %d", got, want)
	}
}

func TestPartTwo(t *testing.T) {
	lines := parseInput(inputString)

	got := middleAutocompletionScore(lines)
	want := 288957

	if got != want {
		t.Errorf("got %d, wanted %d", got, want)
	}
}
