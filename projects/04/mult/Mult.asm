// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Mult.asm

// Multiplies R0 and R1 and stores the result in R2.
// (R0, R1, R2 refer to RAM[0], RAM[1], and RAM[2], respectively.)
//
// This program only needs to handle arguments that satisfy
// R0 >= 0, R1 >= 0, and R0*R1 < 32768.

// RAM[2] = 0
  @0
  D=A
  @R2
  M=D

// operand = RAM[0]
  @R0
  D=M

  @END-OF-PROGRAM
  D;JEQ // if RAM[0] = 0, we're done

  @operand
  M=D

// iterations = RAM[1]
  @R1
  D=M

  @END-OF-PROGRAM
  D;JEQ // if RAM[1] = 0, we're done

  @iterations
  M=D

// if R0 < R1, swap <iterations> and <operand> so we finish in fewer clock
// cycles

  @R0
  D=M-D // D = R0 - R1

  @ITERATE
  D;JGE // R0 >= R1, start iterating

  // operand = RAM[1]
    @R1
    D=M
    @operand
    M=D

  // iterations = RAM[0]
    @R0
    D=M
    @iterations
    M=D

(ITERATE)
  @operand
  D=M

  @R2
  M=D+M

  @iterations
  MD=M-1

  @ITERATE
  D;JGT

(END-OF-PROGRAM)
  @END-OF-PROGRAM
  0;JMP
