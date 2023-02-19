package main

import (
	"fmt"
	"os"
)

type Emitter struct {
	File *os.File
}

func NewEmitter(file string) *Emitter {
	emitter := &Emitter{}

	f, err := os.OpenFile(file, os.O_WRONLY|os.O_CREATE|os.O_TRUNC, 0644)
	if err != nil {
		abort(fmt.Sprintf("%v", err))
	} else {
		emitter.File = f
	}

	return emitter
}

func (e *Emitter) Emitln(instruction interface{ Instruction }) {
	e.File.Write(instruction.Bytes())
	e.File.Write([]byte{10})
}
