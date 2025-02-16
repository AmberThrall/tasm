global _start

_msg: 
    db "Hello World!",0xA

_start:
    mov ebx, 1      ; stdout
    mov ecx, _msg
    mov edx, 13    ; message length
    mov edi, 3      ; print it 5 times

_loop:
    mov eax, 4  ; write
    int 0x80

    dec edi
    jnz _loop

_exit:
    mov eax, 1      ; exit
    mov ebx, 0      ; status code 0
    int 0x80

