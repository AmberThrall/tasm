.globl _start

.section .data
msg:
    .ascii "Hello World\n"

.section .text
_start:
    movl $4, %eax       # sys call for write
    movl $1, %ebx       # set fd to 1 (stdout)
    movl $msg, %ecx     # set buffer address
    movl $12, %edx      # set msg size
    int $0x80           # interrupt kernel to make sys call


