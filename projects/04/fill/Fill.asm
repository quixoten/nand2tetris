// The (fill-screen-with-black) and (fill-screen-with-white) "blocks" below are
// identical except that one sets each address in the screen memory map to -1
// (black), and the other sets each address to 0 (white). This duplication is
// desirable to keep all the state in the registers during iteration. This
// reduces the total number of instructions (PER ITERATION) by 5 (compared to
// the DRY implmentation I tried first).
//
// Before entering a draw loop D is set to 8192 (the total number of addresses
// in the memory map). On each iteration, the number 24576 (one past the last
// address in the memory map) is reduced by D to provide the next memory
// address we want to draw. D is reduced by 1 and the next iteration starts. In
// other words...
//
// Upon the first iteration:
// A = 24576 - 8192 = 16,384 (the start of the screen memory map)
//
// On the second iteration D will have been reduced by 1 to 8191, so now:
// A = 24576 - 8191 = 16,384 (the second word of the screen memory map)
//
// This process continues until D=1:
// A = 24576 - 1 = 24575 (the last word of the screen memory map)
//
// When D is then reduced to 0, the loop ends and control flow starts back over
// at the top of the program where the keyboard memory map is checked.

(check-keyboard)

@KBD
D=M
@fill-screen-with-white
D;JEQ


(fill-screen-with-black)
  @8192
  D=A
  (fill-line-with-black)
    @24576

    A=A-D
    M=-1
    D=D-1
    @fill-line-with-black
    D;JGT
  @check-keyboard
  0;JMP


(fill-screen-with-white)
  @8192
  D=A
  (fill-line-with-white)
    @24576
    A=A-D
    M=0
    D=D-1
    @fill-line-with-white
    D;JGT
  @check-keyboard
  0;JMP
