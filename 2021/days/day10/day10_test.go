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
	_ = parseInput(inputString)

	got := 1
	want := 26397

	if got != want {
		t.Errorf("got %d, wanted %d", got, want)
	}
}

func TestPartTwo(t *testing.T) {
}
