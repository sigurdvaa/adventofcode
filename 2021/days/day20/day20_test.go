package day20

import (
	"testing"
)

var inputString string = `..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#..#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..#.##.#....##..#.####....##...##..#...#......#.#.......#.......##..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###`

func TestPartOne(t *testing.T) {
	algo, img := parseInput(inputString)
	img = enhanceImg(algo, img, 2)
	got := countLitPixels(img)
	want := 35

	if got != want {
		t.Errorf("got %d, wanted %d", got, want)
	}
}

func TestPartTwo(t *testing.T) {
	algo, img := parseInput(inputString)
	img = enhanceImg(algo, img, 50)
	got := countLitPixels(img)
	want := 3351

	if got != want {
		t.Errorf("got %d, wanted %d", got, want)
	}
}
