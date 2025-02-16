global _start

_msg: 
    db "Hello World",0xA

_start:
    mov ebx, 1
    mov ecx, _msg
    mov edx, 12
    mov edi, 5

_loop:
    mov eax, 4
    int 0x80

    dec edi
    jnz _loop

_exit:
    mov eax, 1
    mov ebx, 0
    int 0x80

