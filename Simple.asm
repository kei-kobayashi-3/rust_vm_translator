@7
D=A
@SP
A=M
M=D
@SP
M=M+1
@8
D=A
@SP
A=M
M=D
@SP
M=M+1

// add
@SP
M=M-1 // spの指しているメモリを−１する
@SP
A=M   // spの指しているメモリをAレジスタにセットする
D=M   // Aにセットされている先のデータがMを示す　それをDレジスタにセット
@SP
AM=M-1 // spのデータを−１する　そのデータをAレジスタにもセットする
M=D+M  // spの指している（ポインタ）のメモリをD＋Mにする
@SP
M=M+1

// neg
@SP
M=M-1
@SP
A=M
M=-M
@SP
M=M+1

// eq D-M=0になるときjump 1 else 0 とする
@SP
M=M-1 // spの指しているメモリを−１する
@SP
A=M   // spの指しているメモリをAレジスタにセットする
D=M   // Aにセットされている先のデータがMを示す　それをDレジスタにセット
@SP
AM=M-1 // spのデータを−１する　そのデータをAレジスタにもセットする
MD=D-M // D-MをMとDに保存

@ZERO_%N // 区別が必要
D;JEQ  // Dが０の場合

@SP
A=M
M=0
@GO
0;JMP

(ZERO_%N)
@SP
A=M
M=-1

(GO_%N)
@SP
M=M+1

// gt
@SP
M=M-1 // spの指しているメモリを−１する
@SP
A=M   // spの指しているメモリをAレジスタにセットする
D=M   // Aにセットされている先のデータがMを示す　それをDレジスタにセット
@SP
AM=M-1 // spのデータを−１する　そのデータをAレジスタにもセットする
MD=D-M // D-MをMとDに保存

@ZERO_%N
D;JGT  // Dが>の場合

@SP
A=M
M=0
@GO_%N
0;JMP

(ZERO_%N)
@SP
A=M
M=-1

(GO_%N)
@SP
M=M+1

// and
@SP
M=M-1 // spの指しているメモリを−１する
@SP
A=M   // spの指しているメモリをAレジスタにセットする
D=M   // Aにセットされている先のデータがMを示す　それをDレジスタにセット
@SP
AM=M-1 // spのデータを−１する　そのデータをAレジスタにもセットする
M=D&M //  Mに保存
@SP
M=M+1
