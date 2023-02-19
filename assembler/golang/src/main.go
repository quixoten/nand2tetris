package main

import (
	"fmt"
	"os"
	"path"
	"strings"
)

func main() {
	if len(os.Args) != 2 {
		abort(fmt.Sprintf("usage: %s program.asm\n", os.Args[0]))
	}

	sourcePath := os.Args[1]
	ext := path.Ext(sourcePath)
	destPath := strings.TrimRight(sourcePath, ext) + ".hack"

	lexer := NewLexer(sourcePath)
	emitter := NewEmitter(destPath)
	parser := NewParser(lexer, emitter)
	parser.Program()
}

func abort(str string) {
	fmt.Println(str)
	os.Exit(1)
}
