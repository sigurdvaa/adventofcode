package input

import (
	"fmt"
	"log"
	"os"
)

func ReadDay(day string) string {
	content, err := os.ReadFile(fmt.Sprintf("input/%s.txt", day))
	if err != nil {
		log.Fatal(err)
	}
	return string(content)
}
