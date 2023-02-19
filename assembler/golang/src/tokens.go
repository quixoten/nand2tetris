package main

type TokenType int

const (
	TT_Comp TokenType = iota
	TT_Dest
	TT_DynamicAddress
	TT_EOF
	TT_Equal
	TT_Jump
	TT_Label
	TT_Newline
	TT_StaticAddress
)

type Token struct {
	Data   string
	Type   TokenType
	Line   int
	Column int
}

func NewToken(data string, tt TokenType, line int, column int) *Token {
	return &Token{
		Data:   data,
		Type:   tt,
		Line:   line,
		Column: column,
	}
}

func (t *Token) String() string {
	switch t.Type {
	case TT_Comp:
		return "TT_Comp<" + t.Data + ">"
	case TT_Dest:
		return "TT_Dest<" + t.Data + ">"
	case TT_DynamicAddress:
		return "TT_DynamicAddress<" + t.Data + ">"
	case TT_EOF:
		return "TT_EOF<>"
	case TT_Equal:
		return "TT_Equal<" + t.Data + ">"
	case TT_Jump:
		return "TT_Jump<" + t.Data + ">"
	case TT_Label:
		return "TT_Label<" + t.Data + ">"
	case TT_Newline:
		return "TT_Newline<>"
	case TT_StaticAddress:
		return "TT_StaticAddress<" + t.Data + ">"
	default:
		return "Unknown<>"
	}
}

func (tt TokenType) String() string {
	switch tt {
	case TT_Comp:
		return "Comp"
	case TT_Dest:
		return "Dest"
	case TT_DynamicAddress:
		return "DynamicAddress"
	case TT_EOF:
		return "EOF"
	case TT_Equal:
		return "Equal"
	case TT_Jump:
		return "Jump"
	case TT_Label:
		return "Label"
	case TT_Newline:
		return "Newline"
	case TT_StaticAddress:
		return "StaticAddress"
	}

	return "Unknown"
}
