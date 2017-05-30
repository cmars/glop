package main

import (
	"io/ioutil"
	"log"
	"os"

	"github.com/cmars/glop"
)

func main() {
	if len(os.Args) < 2 {
		log.Fatalf("Usage: %s <glop.yaml>", os.Args[0])
	}
	f, err := os.Open(os.Args[1])
	if err != nil {
		log.Fatalf("cannot open %s: %v", os.Args[1], err)
	}
	defer f.Close()
	b, err := ioutil.ReadAll(f)
	if err != nil {
		log.Fatalf("cannot read from %s: %v", os.Args[1], err)
	}
	st, err := glop.ParseYAML(b)
	if err != nil {
		log.Fatalf("cannot parse %s: %v", os.Args[1], err)
	}
	log.Printf("%+v", st)
}
