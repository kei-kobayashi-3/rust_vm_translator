// push local n
@n
D=A
@LCL
A=D+M
D=M
@SP
A=M
M=D
@SP
M=M+1

// pop local n
@n
D=A
@LCL
D=D+M

@SP
M=M-1
@SP
A=D
M=M
@SP
M=M+1
