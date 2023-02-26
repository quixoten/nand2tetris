package main

import (
	"bufio"
	"fmt"
	"os"
)

type FileRune struct {
	Rune   rune
	Line   int
	Column int
}

type Lexer struct {
	File    *os.File
	Reader  *bufio.Reader
	Current *FileRune
	Next    *FileRune
}

func NewLexer(file string) *Lexer {
	lexer := &Lexer{
		Current: &FileRune{Rune: 0, Line: 1, Column: 0},
		Next:    &FileRune{Rune: 0, Line: 1, Column: 0},
	}

	if f, err := os.Open(file); err != nil {
		abort(fmt.Sprintf("%v", err))
	} else {
		lexer.File = f
		lexer.Reader = bufio.NewReader(f)
	}

	lexer.ConsumeRune() // Once to fill in Current
	lexer.ConsumeRune() // And once to fill in Next

	return lexer
}

func (l *Lexer) ConsumeRune() {
	var nextRune rune

	for {
		nextRune, _, _ = l.Reader.ReadRune()
		if nextRune != '\r' {
			break
		}
	}

	l.Current, l.Next = l.Next, &FileRune{Rune: nextRune}

	if l.Current.Rune == '\n' {
		l.Next.Line = l.Current.Line + 1
		l.Next.Column = 1
	} else {
		l.Next.Line = l.Current.Line
		l.Next.Column = l.Current.Column + 1
	}
}

func (l *Lexer) ConsumeToken() (token *Token) {
	l.SkipWhitespace()
	l.SkipComment()

	switch {
	case 0 == l.Current.Rune:
		token = NewToken("", TT_EOF, l.Current.Line, l.Current.Column)

	case '\n' == l.Current.Rune:
		token = NewToken(string(l.Current.Rune), TT_Newline, l.Current.Line, l.Current.Column)

	case '@' == l.Current.Rune:
		if IsLetter(l.Next.Rune) {
			token = l.ParseDynamicAddress()
		} else if IsDigit(l.Next.Rune) {
			token = l.ParseStaticAddress()
		} else {
			abort(fmt.Sprintf("Unexpected character '%c' while parsing '@<addr>' at %d:%d", l.Next.Rune, l.Next.Line, l.Next.Column))
		}

	case ';' == l.Current.Rune:
		token = l.ConsumeJump()

	case '(' == l.Current.Rune:
		token = l.ConsumeLabel()

	case '=' == l.Current.Rune:
		token = l.ConsumeComp()

	case IsDestOrCompCharacter(l.Current.Rune):
		token = l.ConsumeDestOrComp()

	default:
		abort(fmt.Sprintf("Unexpected character '%c' at %d:%d", l.Current.Rune, l.Current.Line, l.Current.Column))
	}

	l.ConsumeRune()

	return token
}

func (l *Lexer) SkipWhitespace() {
LOOP:
	for {
		switch l.Current.Rune {
		case ' ', '\t':
			l.ConsumeRune()
		default:
			break LOOP
		}
	}
}

func (l *Lexer) SkipComment() {
	if l.Current.Rune == '/' && l.Next.Rune == '/' {
		for !IsNewlineOrEnd(l.Next.Rune) {
			l.ConsumeRune()
		}
		l.ConsumeRune()
	}
}

func (l *Lexer) ParseDynamicAddress() *Token {
	addr := ""
	line := l.Current.Line
	column := l.Current.Column
LOOP:
	for {
		switch {
		case IsValidLabelCharacter(l.Next.Rune):
			addr += string(l.Next.Rune)
			l.ConsumeRune()
		default:
			break LOOP
		}
	}

	return NewToken(addr, TT_DynamicAddress, line, column)
}

func (l *Lexer) ParseStaticAddress() *Token {
	addr := ""
	line := l.Current.Line
	column := l.Current.Column
LOOP:
	for {
		switch {
		case IsDigit(l.Next.Rune):
			addr += string(l.Next.Rune)
			l.ConsumeRune()
		default:
			break LOOP
		}
	}

	return NewToken(addr, TT_StaticAddress, line, column)
}

func (l *Lexer) ConsumeDestOrComp() *Token {
	data := string(l.Current.Rune)
	line := l.Current.Line
	column := l.Current.Column

	for {
		switch {
		case IsDestOrCompCharacter(l.Next.Rune):
			data += string(l.Next.Rune)
			l.ConsumeRune()
		case l.Next.Rune == '=' || l.Next.Rune == ' ':
			return NewToken(data, TT_Dest, line, column)
		case IsNewlineOrEnd(l.Next.Rune) || l.Next.Rune == ';':
			return NewToken(data, TT_Comp, line, column)
		default:
			abort(fmt.Sprintf("unexpected character '%c' %d:%d", l.Next.Rune, l.Next.Line, l.Next.Column))
		}
	}
}

func (l *Lexer) ConsumeComp() *Token {
	data := ""
	line := l.Current.Line
	column := l.Current.Column

	for {
		switch {
		case IsDestOrCompCharacter(l.Next.Rune):
			data += string(l.Next.Rune)
			l.ConsumeRune()
		case l.Next.Rune == ' ' || l.Next.Rune == ';' || IsNewlineOrEnd(l.Next.Rune):
			return NewToken(data, TT_Comp, line, column)
		default:
			abort(fmt.Sprintf("unexpected character '%c' at %d:%d", l.Next.Rune, l.Next.Line, l.Next.Column))
		}
	}
}

func (l *Lexer) ConsumeJump() *Token {
	data := ""
	line := l.Current.Line
	column := l.Current.Column
LOOP:
	for {
		switch {
		case IsJumpLetter(l.Next.Rune):
			data += string(l.Next.Rune)
			l.ConsumeRune()
		case IsNewlineOrEnd(l.Next.Rune) || l.Next.Rune == ' ':
			break LOOP
		default:
			abort(fmt.Sprintf(
				"Unexpected character '%c' while parsing jump at %d:%d",
				l.Next.Rune,
				l.Next.Line,
				l.Next.Column,
			))
		}
	}

	return NewToken(data, TT_Jump, line, column)
}

func (l *Lexer) ConsumeLabel() *Token {
	var name string

	line := l.Current.Line
	column := l.Current.Column

LOOP:
	for {
		switch {
		case IsValidLabelCharacter(l.Next.Rune):
			name += string(l.Next.Rune)
			l.ConsumeRune()
		case l.Next.Rune == ')':
			l.ConsumeRune()
			break LOOP
		case l.Next.Rune == '\n':
			abort(fmt.Sprintf(
				"Unexpected end of line while parsing label at %d:%d",
				l.Next.Line,
				l.Next.Column,
			))
		case l.Next.Rune == 0:
			abort(fmt.Sprintf(
				"Unexpected end of file while parsing label at %d:%d",
				l.Next.Line,
				l.Next.Column,
			))
		default:
			abort(fmt.Sprintf(
				"Unexpected character '%c' while parsing label at %d:%d",
				l.Next.Rune,
				l.Next.Line,
				l.Next.Column,
			))
		}
	}

	return NewToken(name, TT_Label, line, column)
}

func IsNewlineOrEnd(r rune) bool {
	switch r {
	case 0, '\n':
		return true
	default:
		return false
	}
}

func IsDestOrCompCharacter(r rune) bool {
	switch r {
	case '!', '&', '+', '-', '0', '1', 'A', 'D', 'M', '|':
		return true
	default:
		return false
	}
}

func IsDigit(r rune) bool {
	return (r >= '0' && r <= '9')
}

func IsJumpLetter(r rune) bool {
	switch r {
	case 'E', 'G', 'J', 'L', 'M', 'N', 'P', 'Q', 'T':
		return true
	default:
		return false

	}
}

func IsLetter(r rune) bool {
	return (r >= 'a' && r <= 'z') || (r >= 'A' && r <= 'Z')
}

func IsValidLabelCharacter(r rune) bool {
	switch {
	case IsLetter(r):
		fallthrough
	case IsDigit(r):
		fallthrough
	case r == '_' || r == '.' || r == '$' || r == '-':
		return true
	}

	return false
}
