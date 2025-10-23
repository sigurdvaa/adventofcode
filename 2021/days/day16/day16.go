package day16

import (
	"aoc_2021/input"
	"fmt"
	"log"
	"strings"
)

var hexMap = map[rune]byte{
	'0': 0b0000,
	'1': 0b0001,
	'2': 0b0010,
	'3': 0b0011,
	'4': 0b0100,
	'5': 0b0101,
	'6': 0b0110,
	'7': 0b0111,
	'8': 0b1000,
	'9': 0b1001,
	'A': 0b1010,
	'B': 0b1011,
	'C': 0b1100,
	'D': 0b1101,
	'E': 0b1110,
	'F': 0b1111,
}

func parseInput(str string) []rune {
	return []rune(strings.TrimSpace(str))
}

type Stream struct {
	data []rune
	len  int
	buff byte
}

func (s *Stream) init(data []rune) {
	s.data = data
}

func (s *Stream) read() {
	if len(s.data) > 0 {
		s.buff = s.buff << 4
		s.buff += hexMap[s.data[0]]
		s.len += 4
		s.data = s.data[1:]
	} else {
		log.Fatal("no more data to read")
	}
}

func (s *Stream) getValue(size int) int {
	var value byte
	for range size {
		if s.len == 0 {
			s.read()
		}
		s.len -= 1
		value <<= 1
		value += (s.buff >> s.len) & 1
	}
	return int(value)
}

func sumPacketVersion(packet []rune) int {
	sum := 0
	stream := &Stream{}
	stream.init(packet)

	fmt.Println("version", stream.getValue(3))
	fmt.Println("type", stream.getValue(3))

	return sum
}

func Run() {
	fmt.Println("Day 16: Packet Decoder")

	inputString := input.ReadDay("day16")
	packet := parseInput(inputString)

	fmt.Printf("Part One: %d\n", sumPacketVersion(packet))
	fmt.Printf("Part Two: TODO\n")
}
