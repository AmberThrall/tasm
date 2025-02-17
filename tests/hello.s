ENTRY _start

_msg: 
    DB "Hello World!",0xA
    EQU msg_len $ - _msg

_start:
    mov ebx, 1       ; stdout
    mov ecx, _msg    ; what to print
    mov edx, msg_len ; message length
    mov di, 5        ; print it 5 times

_loop:
    mov eax, 4       ; write
    int 0x80
    dec di 
    jnz _loop

    mov eax, 1       ; exit
    mov ebx, 0       ; status code 0
    int 0x80

