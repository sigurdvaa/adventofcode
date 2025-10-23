package day16

import (
	"aoc_2021/input"
	"fmt"
	"log"
	"math"
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

type Packet struct {
	version int
	typeid  int
	size    int
	value   int
	sub     []Packet
}

type Stream struct {
	data []rune
	len  int
	buff byte
}

func (s *Stream) init(data []rune) {
	s.data = data
}

func (s *Stream) readByte() {
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
			s.readByte()
		}
		s.len -= 1
		value <<= 1
		value += (s.buff >> s.len) & 1
	}
	return int(value)
}

func (s *Stream) getPacket() Packet {
	packet := Packet{}
	packet.version = s.getValue(3)
	packet.typeid = s.getValue(3)
	packet.size = 6

	switch packet.typeid {
	case 4: // literal
		for {
			tmp := s.getValue(5)
			packet.size += 5
			packet.value <<= 4
			packet.value += tmp & 0b1111
			if tmp&(1<<4) == 0 {
				break
			}
		}
	default: // operator
		lengthid := s.getValue(1)
		packet.size += 1

		switch lengthid {
		case 0:
			subbits := s.getValue(15)
			packet.size += 15
			total := 0
			for total < subbits {
				subpacket := s.getPacket()
				total += subpacket.size
				packet.size += subpacket.size
				packet.sub = append(packet.sub, subpacket)
			}
		case 1:
			num := s.getValue(11)
			packet.size += 11
			for range num {
				subpacket := s.getPacket()
				packet.size += subpacket.size
				packet.sub = append(packet.sub, subpacket)
			}
		}

		switch packet.typeid {
		case 0: // sum
			for _, p := range packet.sub {
				packet.value += p.value
			}
		case 1: // prod
			packet.value = 1
			for _, p := range packet.sub {
				packet.value *= p.value
			}
		case 2: // min
			packet.value = math.MaxInt
			for _, p := range packet.sub {
				if p.value < packet.value {
					packet.value = p.value
				}
			}
		case 3: // max
			for _, p := range packet.sub {
				if p.value > packet.value {
					packet.value = p.value
				}
			}
		case 5: // greater than
			if packet.sub[0].value > packet.sub[1].value {
				packet.value = 1
			}
		case 6: // less than
			if packet.sub[0].value < packet.sub[1].value {
				packet.value = 1
			}
		case 7: // equal
			if packet.sub[0].value == packet.sub[1].value {
				packet.value = 1
			}
		default:
			log.Fatal("unreachable")
		}

	}

	return packet
}

func recurSumVersions(packet Packet) int {
	sum := packet.version
	if len(packet.sub) == 0 {
		return sum
	}
	for _, p := range packet.sub {
		sum += recurSumVersions(p)
	}
	return sum
}

func sumPacketsVersion(packet []rune) int {
	stream := &Stream{}
	stream.init(packet)
	return recurSumVersions(stream.getPacket())
}

func sumPacketsValue(str []rune) int {
	stream := &Stream{}
	stream.init(str)
	return (stream.getPacket()).value
}

func Run() {
	fmt.Println("Day 16: Packet Decoder")

	inputString := input.ReadDay("day16")
	stream := parseInput(inputString)

	fmt.Printf("Part One: %d\n", sumPacketsVersion(stream))
	fmt.Printf("Part Two: %d\n", sumPacketsValue(stream))
}
