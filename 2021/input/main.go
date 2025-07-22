package input

import (
	"fmt"
	"log"
	"os"
)

func ReadDay(day int) string {
	content, err := os.ReadFile(fmt.Sprintf("input/day%02d.txt", day))
	if err != nil {
		log.Fatal(err)
	}
	return string(content)
}
