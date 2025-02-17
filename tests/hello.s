entry _start

_msg: 
    db "Hello World!",0xA

_print:
    mov eax, 4
    int 0x80
    ret

_start:
    mov ebx, 1      ; stdout
    mov ecx, _msg
    mov edx, 13    ; message length

    mov di, 5      ; print it 5 times

_loop:
    call _print
    dec di 
    jnz _loop

_exit:
    mov eax, 1      ; exit
    mov ebx, 0      ; status code 0
    int 0x80

