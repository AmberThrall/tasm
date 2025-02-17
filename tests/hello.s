entry _start

_msg: 
    db "Hello World!",0xA

_print:
    mov eax, 4      ; write
    mov ebx, 1      ; stdout
    int 0x80
    ret

_start:
    mov ecx, _msg   ; what to print
    mov edx, 13     ; message length
    mov di, 5       ; print it 5 times

_loop:
    call _print
    dec di 
    jnz _loop

_exit:
    mov eax, 1      ; exit
    mov ebx, 0      ; status code 0
    int 0x80

