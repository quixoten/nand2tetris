package main

import (
	"fmt"
	"strconv"
)

type Parser struct {
	Lexer        *Lexer
	Emitter      *Emitter
	Current      *Token
	Next         *Token
	Symbols      map[string]int
	Dest         string
	Comp         string
	Instructions []interface{ Instruction }
}

const MaxAddress = 0b1111111111111111

func DefaultSymbols() map[string]int {
	symbols := make(map[string]int)

	symbols["SP"] = 0
	symbols["LCL"] = 1
	symbols["ARG"] = 2
	symbols["THIS"] = 3
	symbols["THAT"] = 4
	symbols["R0"] = 0
	symbols["R1"] = 1
	symbols["R2"] = 2
	symbols["R3"] = 3
	symbols["R4"] = 4
	symbols["R5"] = 5
	symbols["R6"] = 6
	symbols["R7"] = 7
	symbols["R8"] = 8
	symbols["R9"] = 9
	symbols["R10"] = 10
	symbols["R11"] = 11
	symbols["R12"] = 12
	symbols["R13"] = 13
	symbols["R14"] = 14
	symbols["R15"] = 15
	symbols["SCREEN"] = 16384
	symbols["KBD"] = 24576

	return symbols
}

func NewParser(lexer *Lexer, emitter *Emitter) *Parser {
	parser := &Parser{
		Lexer:   lexer,
		Emitter: emitter,
		Symbols: DefaultSymbols(),
	}

	parser.ConsumeToken()
	parser.ConsumeToken()

	return parser
}

func (p *Parser) ConsumeToken() {
	token := p.Lexer.ConsumeToken()

	//log.Println(token)
	p.Current, p.Next = p.Next, token
}

func (p *Parser) Program() {
	fmt.Printf("Assembling \"%s\" into \"%s\"\n", p.Lexer.File.Name(), p.Emitter.File.Name())

	for p.Current.Type == TT_Newline {
		p.ConsumeToken()
	}

	for p.Current.Type != TT_EOF {
		p.Instruction()
	}

	nextDynamicAddress := 16
	for _, i := range p.Instructions {
		if d, ok := i.(*DynamicAddress); ok {
			if addr, ok := p.Symbols[d.Label]; ok {
				d.Address = addr
			} else {
				d.Address = nextDynamicAddress
				p.Symbols[d.Label] = d.Address
				nextDynamicAddress += 1
			}
		}
	}

	for _, instruction := range p.Instructions {
		p.Emitter.Emitln(instruction)
	}

	fmt.Printf("Done.\n")
}

func (p *Parser) Instruction() {
	switch p.Current.Type {
	case TT_StaticAddress:
		p.AppendInstruction(p.ConsumeStaticAddress())

	case TT_DynamicAddress:
		p.AppendInstruction(p.ConsumeDynamicAddress())

	case TT_Label:
		if label, ok := p.Symbols[p.Current.Data]; ok {
			abort(fmt.Sprintf("Found duplicate label named \"%s\"", label))
		}
		p.Symbols[p.Current.Data] = len(p.Instructions)
		p.ConsumeToken()
		p.NewlineOrEnd()

	case TT_Dest:
		dest := p.ConsumeDest()
		comp, jump := p.ConsumeComp()

		p.AppendInstruction(&Computation{
			Dest: dest,
			Comp: comp,
			Jump: jump,
		})

	case TT_Comp:
		comp, jump := p.ConsumeComp()

		p.AppendInstruction(&Computation{
			Dest: nil,
			Comp: comp,
			Jump: jump,
		})

	default:
		abort(fmt.Sprintf(
			"Invalid token: %s<%s> at %d:%d",
			p.Current.Type,
			p.Current.Data,
			p.Current.Line,
			p.Current.Column,
		))
	}
}

func (p *Parser) AppendInstruction(instruction interface{ Instruction }) {
	instruction.SetNumber(len(p.Instructions))
	p.Instructions = append(p.Instructions, instruction)
	//log.Printf("========================> %s", instruction)
}

func (p *Parser) ConsumeStaticAddress() interface{ Instruction } {
	stringAddr := p.Current.Data
	token := p.Current

	addr, err := strconv.Atoi(stringAddr)
	if err != nil {
		abort(fmt.Sprintf("invalid number for address: %s", stringAddr))
	}

	if addr > MaxAddress {
		abort(fmt.Sprintf("The address \"%d\" at %d:%d is larger than %d.", addr, token.Line, token.Column, MaxAddress))
	}

	p.ConsumeToken()
	p.NewlineOrEnd()

	return &StaticAddress{
		Token:   token,
		Address: addr,
	}

}

func (p *Parser) ConsumeDynamicAddress() interface{ Instruction } {
	label := p.Current.Data
	token := p.Current

	p.ConsumeToken()
	p.NewlineOrEnd()

	return &DynamicAddress{
		Token: token,
		Label: label,
	}

}

func (p *Parser) ConsumeDest() *Token {
	dest := p.Current
	p.ConsumeToken()
	return dest
}

func (p *Parser) ConsumeComp() (comp *Token, jump *Token) {
	comp = p.Current
	p.ConsumeToken()

	if p.Current.Type == TT_Jump {
		jump = p.Current
		p.ConsumeToken()
	}

	p.NewlineOrEnd()

	return comp, jump
}

func (p *Parser) NewlineOrEnd() {
	switch p.Current.Type {
	case TT_Newline:
		for p.Current.Type == TT_Newline {
			p.ConsumeToken()
		}

		return

	case TT_EOF:
		return

	default:
		abort(fmt.Sprintf("Expected EOF or Newline at %d:%d", p.Lexer.Current.Line, p.Lexer.Current.Column))
	}
}

func (p *Parser) Match(tt TokenType) {
	switch p.Current.Type {
	case tt:
		p.ConsumeToken()
	default:
		abort(fmt.Sprintf("Expected %s at %d:%d", p.Current.Line, p.Current.Column))

	}
}
