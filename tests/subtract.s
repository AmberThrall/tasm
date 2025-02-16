entry _start

_start:
    ; Compute 0x6C - 0x54 and write the result to stdout in base (EBX)
    mov eax, 0x6c
    sub eax, 0x54
    mov ebx, 10
    mov ecx, 0x09000000
    xor edi, edi

_loop:
    xor edx, edx
    db 0xF7, 0xF3                       ; div ebx
    db 0x80, 0xFA, 0x09                 ; cmp dl, 9
    jle _skip_a
    add dl, 0x07
_skip_a:
    add dl, 0x30
    dec ecx
    inc edi
    mov [ecx], dl
    db 0x85, 0xC0                       ; cmp eax, 0
    jnz _loop

    ; Write to stdout
    mov eax, 4
    mov ebx, 1
    mov edx, edi
    int 0x80

    ; Exit
    mov eax, 1
    xor ebx, ebx
    int 0x80

