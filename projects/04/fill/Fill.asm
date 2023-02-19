// This file is part of www.nand2tetris.org
// and the book "The Elements of Computing Systems"
// by Nisan and Schocken, MIT Press.
// File name: projects/04/Fill.asm

// Runs an infinite loop that listens to the keyboard input.
// When a key is pressed (any key), the program blackens the screen,
// i.e. writes "black" in every pixel;
// the screen should remain fully black as long as the key is pressed. 
// When no key is pressed, the program clears the screen, i.e. writes
// "white" in every pixel;
// the screen should remain fully clear as long as no key is pressed.

// last-word = SCREEN + 8192 (8K)
  @SCREEN
  D=A

  @8192
  D=D+A

  @last-word
  M=D

(FILL-ENTIRE-SCREEN)
  // start <next-word> at the top left of the screen.
  // <next-word> is a pointer
    @SCREEN
    D=A

    @next-word
    M=D

  // set the fill-value based on KBD state
    @fill-value
    M=0

    @KBD
    D=M

    @KEY-PRESSED
    D;JNE

    @FILL-NEXT-WORD
    0;JMP

    (KEY-PRESSED)
    @fill-value
    M=-1

  (FILL-NEXT-WORD)

    // fill in the next 16 pixels
      @fill-value
      D=M
      @next-word
      A=M
      M=D

    // advance the <next-word> forward
      @next-word
      MD=M+1

    @last-word
    D=M-D // D = last-word - next-word

    @FILL-NEXT-WORD
    D;JGT // last-word - next-word > 0, fill next word

@FILL-ENTIRE-SCREEN
0;JMP
