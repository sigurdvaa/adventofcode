package day21

import (
	"aoc_2021/input"
	"fmt"
	"strconv"
	"strings"
)

func parseInput(str string) (int, int) {
	starts := []int{}
	for line := range strings.SplitSeq(str, "\n") {
		if line == "" {
			continue
		}
		split := strings.Split(line, ": ")
		num, err := strconv.Atoi(split[1])
		if err != nil {
			panic(err)
		}
		starts = append(starts, num)
	}
	return starts[0], starts[1]
}

type Dice struct {
	face  int
	total int
}

func (d *Dice) roll() int {
	d.face += 1
	d.total += 1
	if d.face > 100 {
		d.face = 1
	}
	return d.face
}

func (d *Dice) turn() int {
	sum := d.roll()
	sum += d.roll()
	sum += d.roll()
	return sum
}

func playDeterministicDice(p1, p2 int) int {
	dice := Dice{}
	p1score, p2score := 0, 0

	for {
		p1 += dice.turn()
		p1 %= 10
		if p1 == 0 {
			p1 = 10
		}
		p1score += p1
		if p1score >= 1000 {
			break
		}

		p2 += dice.turn()
		p2 %= 10
		if p2 == 0 {
			p2 = 10
		}
		p2score += p2
		if p2score >= 1000 {
			break
		}
	}
	return min(p1score, p2score) * dice.total
}

const GAME_LIMIT int = 21
const POS_LIMIT int = 10
const PLAYERS int = 2

type Game struct {
	pos    [PLAYERS]int
	score  [PLAYERS]int
	player int
}
type Wins [PLAYERS]int
type Cache map[Game]Wins
type Rolls map[int]int

func (g *Game) copy() Game {
	newGame := Game{}
	newGame.player = g.player
	for i := range PLAYERS {
		newGame.pos[i] = g.pos[i]
		newGame.score[i] = g.score[i]
	}
	return newGame
}

func recursQuantumDice(seen Cache, rolls Rolls, game Game) Wins {
	for i := range PLAYERS {
		if game.score[i] >= GAME_LIMIT {
			wins := Wins{}
			wins[i] = 1
			seen[game] = wins
			return wins
		}
	}

	wins := Wins{}
	for roll, count := range rolls {
		nextGame := game.copy()
		nextGame.pos[game.player] = (nextGame.pos[game.player] + roll) % POS_LIMIT
		if nextGame.pos[game.player] == 0 {
			nextGame.pos[game.player] = 10
		}
		nextGame.score[game.player] += nextGame.pos[game.player]
		nextGame.player = (game.player + 1) % PLAYERS
		nextWins, ok := seen[nextGame]
		if !ok {
			nextWins = recursQuantumDice(seen, rolls, nextGame)
		}
		wins[0] += nextWins[0] * count
		wins[1] += nextWins[1] * count
	}

	seen[game] = wins
	return wins
}

func precalcRolls() map[int]int {
	rolls := map[int]int{}
	for x := 1; x < 4; x++ {
		for y := 1; y < 4; y++ {
			for z := 1; z < 4; z++ {
				rolls[x+y+z] += 1
			}
		}
	}
	return rolls
}

func playQuantumDice(p1, p2 int) int {
	seen := Cache{}
	rolls := precalcRolls()
	game := Game{
		pos:    [PLAYERS]int{p1, p2},
		score:  [PLAYERS]int{0, 0},
		player: 0,
	}
	wins := recursQuantumDice(seen, rolls, game)
	return max(wins[0], wins[1])
}

func Run() {
	fmt.Println("Day 21: Dirac Dice")

	inputString := input.ReadDay("day21")
	p1, p2 := parseInput(inputString)

	fmt.Printf("Part One: %d\n", playDeterministicDice(p1, p2))
	fmt.Printf("Part Two: %d\n", playQuantumDice(p1, p2))
}
