// push pointer n

@n
D=A
@THIS
A=D+A
D=M
@SP
A=M
M=D
@SP
M=M+1

// push temp n

@n
D=A
@R5
A=D+A
D=M
@SP
A=M
M=D
@SP
M=M+1

// pop pointer n

@n
D=A
@THIS
D=D+A
@R15
M=D
@SP
AM=M-1
D=M
@R15
A=M
M=D

// push local n

// @n
// D=A
// @LCL
// A=D+M
// D=M
// @SP
// A=M
// M=D
// @SP
// M=M+1

// pop local n  popは最初変数で実装したが下のサイトによるとR15を使っていた。
// https://www.csie.ntu.edu.tw/~cyy/courses/introCS/16fall/lectures/handouts/lec11_VMI_4up.pdf

// @3
// D=A
// @LCL
// D=D+M
// @R15
// M=D
// @SP
// AM=M-1
// D=M
// @R15
// A=M
// M=D
