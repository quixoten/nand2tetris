package main

import "fmt"

type Instruction interface {
	Bytes() []byte
	GetNumber() int
	SetNumber(int)
	String() string
}

type StaticAddress struct {
	Token   *Token
	Number  int
	Address int
}

type DynamicAddress struct {
	Token   *Token
	Number  int
	Label   string
	Address int
}

type Computation struct {
	Number int
	Dest   *Token
	Comp   *Token
	Jump   *Token
}

func (i *Computation) Bytes() []byte {
	return []byte(i.TranslateComp() + i.TranslateDest() + i.TranslateJump())
}

func (i *Computation) GetNumber() int {
	return i.Number
}

func (i *Computation) SetNumber(n int) {
	i.Number = n
}

func (i *Computation) String() string {
	var dest, jump string

	if i.Dest == nil {
		dest = ""
	} else {
		dest = i.Dest.Data + "="
	}

	if i.Jump == nil {
		jump = ""
	} else {
		jump = ";" + i.Jump.Data
	}

	return fmt.Sprintf("Computation<%s>",
		dest+i.Comp.Data+jump,
	)
}

func (i *Computation) TranslateComp() string {
	switch i.Comp.Data {
	case "0":
		return "1110101010"
	case "1":
		return "1110111111"
	case "-1":
		return "1110111010"
	case "D":
		return "1110001100"
	case "A":
		return "1110110000"
	case "M":
		return "1111110000"
	case "!D":
		return "1110001101"
	case "!A":
		return "1110110001"
	case "!M":
		return "1111110001"
	case "-D":
		return "1110001111"
	case "-A":
		return "1110110011"
	case "-M":
		return "1111110011"
	case "D+1":
		return "1110011111"
	case "A+1":
		return "1110110111"
	case "M+1":
		return "1111110111"
	case "D-1":
		return "1110001110"
	case "A-1":
		return "1110110010"
	case "M-1":
		return "1111110010"
	case "D+A":
		return "1110000010"
	case "D+M":
		return "1111000010"
	case "D-A":
		return "1110010011"
	case "D-M":
		return "1111010011"
	case "A-D":
		return "1110000111"
	case "M-D":
		return "1111000111"
	case "D&A":
		return "1110000000"
	case "D&M":
		return "1111000000"
	case "D|A":
		return "1110010101"
	case "D|M":
		return "1111010101"
	default:
		abort(fmt.Sprintf("%s is not a valid instruction", i.Comp.Data))
	}

	return ""
}

func (i *Computation) TranslateDest() string {
	if i.Dest == nil {
		return "000"
	}

	switch i.Dest.Data {
	case "M":
		return "001"

	case "D":
		return "010"

	case "MD":
		return "011"

	case "A":
		return "100"

	case "AM":
		return "101"

	case "AD":
		return "110"

	case "AMD":
		return "111"

	default:
		abort(fmt.Sprintf("%s is not a valid destination", i.Dest.Data))
	}

	return ""
}

func (i *Computation) TranslateJump() string {
	if i.Jump == nil {
		return "000"
	}

	switch i.Jump.Data {
	case "JGT":
		return "001"

	case "JEQ":
		return "010"

	case "JGE":
		return "011"

	case "JLT":
		return "100"

	case "JNE":
		return "101"

	case "JLE":
		return "110"

	case "JMP":
		return "111"

	default:
		abort(fmt.Sprintf("\"%s\" is not a valid jump directive", i.Jump.Data))
	}

	return ""
}

func (i *DynamicAddress) Bytes() []byte {
	return []byte(fmt.Sprintf("0%015b", i.Address))
}

func (i *DynamicAddress) GetNumber() int {
	return i.Number
}

func (i *DynamicAddress) SetNumber(n int) {
	i.Number = n
}

func (i *DynamicAddress) String() string {
	return fmt.Sprintf("DynamicAddress<@%s>", i.Token.Data)
}

func (i *StaticAddress) Bytes() []byte {
	return []byte(fmt.Sprintf("0%015b", i.Address))
}

func (i *StaticAddress) GetNumber() int {
	return i.Number
}

func (i *StaticAddress) SetNumber(n int) {
	i.Number = n
}

func (i *StaticAddress) String() string {
	return fmt.Sprintf("StaticAddress<@%s>", i.Token.Data)
}
